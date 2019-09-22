// Copyright 2019 Diggory Hardy
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Functionality based on heightmaps

use nalgebra as na;
use na::{convert, try_convert, DMatrix, Dynamic, Vector3, RealField, geometry::{Point2, Point3}};
use ncollide3d::procedural::{TriMesh, IndexBuffer};
use ncollide3d::shape::HeightField;

use crate::unbounded::UnboundedSurface;

pub use displacement::{midpoint_displacement, diamond_square};
pub use fault::fault_displacement;
pub use voronoi::Voronoi;

mod displacement;
mod fault;
mod voronoi;
mod ncollide_impls;

/// A heightmap represents a (terrian) surface via a grid of height offsets.
/// 
/// The surface is defined via a grid of `dim.0 × dim.1` vertices, each with a
/// specified height `h` and corresponding coordinate `(x, y)`, resulting in a
/// world-coordinate of `(x, y, h)`.
/// 
/// Vertices may be referred to via an index `c = (cx, cy)`. A **cell** refers
/// to the rectangular region defined by the four indices `(cx, cy)`,
/// `(cx+1, cy)`, `(cx, cy+1)`, `(cx+1, cy+1)`, where `cx+1<dim.0` and
/// `cy+1<dim.1`.
/// 
/// A heightmap has local coordinates from `(0, 0)` to `(size, size)`. The
/// x-coordinate of a vertex `cx` is thus `size.0 * cx / (dim.0 - 1)`.
#[derive(Debug, Clone)]
pub struct Heightmap<F> {
    dim: (u32, u32),
    len_frac: (F, F),   // size / (dim - (1,1))
    size: (F, F),
    range: (F, F),  // (min, max) height
    data: Vec<F>,
}

// accessors
impl<F: RealField> Heightmap<F> {
    /// Get the grid dimension
    #[inline]
    pub fn dim(&self) -> (u32, u32) {
        self.dim
    }
    
    /// Get the size of the height-map
    #[inline]
    pub fn size(&self) -> (F, F) {
        self.size
    }
    
    /// Get the coordinates of the given vertex
    #[inline]
    pub fn coord_of(&self, cx: u32, cy: u32) -> (F, F) {
        let x = convert::<_, F>(cx as f64) * self.len_frac.0;
        let y = convert::<_, F>(cy as f64) * self.len_frac.1;
        (x, y)
    }
    
    // Find the cell at the given point, if any.
    // 
    // (Note that a 'cell' is defined by the *lowest* of its four vertices.)
    #[inline]
    pub fn cell_at_coord(&self, x: F, y: F) -> Option<(u32, u32)> {
        if F::zero() <= x && x <= self.size.0 {
            if F::zero() <= y && y <= self.size.1 {
                let cx = try_convert::<_, f64>(x / self.len_frac.0).unwrap() as u32;
                let cy = try_convert::<_, f64>(y / self.len_frac.1).unwrap() as u32;
                return Some((cx, cy));
            }
        }
        None
    }
    
    /// Get `(min, max)` altitudes
    #[inline]
    pub fn range(&self) -> (F, F) {
        self.range
    }
    
    /// Get value at the given vertex.
    /// 
    /// Requires `cx < self.dim().0 && cy < self.dim().1`.
    #[inline]
    pub fn get(&self, cx: u32, cy: u32) -> F {
        assert!(cx < self.dim.0);
        assert!(cy < self.dim.1);
        self.data[(cx as usize) + (cy as usize) * (self.dim.0 as usize)]
    }
    
    /// Set value at the given coordinates.
    /// 
    /// Requires `cx < self.dim().0 && cy < self.dim().1`.
    #[inline]
    pub fn set(&mut self, cx: u32, cy: u32, val: F) {
        assert!(cx < self.dim.0);
        assert!(cy < self.dim.1);
        self.range = (self.range.0.min(val), self.range.1.max(val));
        self.data[(cx as usize) + (cy as usize) * (self.dim.0 as usize)] = val;
    }
}

// constructors
impl<F: RealField> Heightmap<F> {
    /// Construct a new, flat Heightmap with the given `dim` and `size`.
    pub fn new_flat(dim: (u32, u32), size: (F, F)) -> Self {
        let x_frac: F = size.0 / convert((dim.0 - 1) as f64);
        let y_frac: F = size.1 / convert((dim.1 - 1) as f64);
        Heightmap {
            dim,
            len_frac: (x_frac, y_frac),
            size,
            range: (F::zero(), F::zero()),
            data: vec![F::zero(); dim.0 as usize * dim.1 as usize],
        }
    }
    
    /// Construct a new Heightmap using the given evaluation function and with
    /// the given `dim` and `size`.
    pub fn from_surface(dim: (u32, u32), size: (F, F), surface: &dyn UnboundedSurface<F>) -> Self {
        let x_frac: F = size.0 / convert((dim.0 - 1) as f64);
        let y_frac: F = size.1 / convert((dim.1 - 1) as f64);
        let mut data = Vec::with_capacity(dim.0 as usize * dim.1 as usize);
        for iy in 0..dim.1 {
            let y = convert::<_, F>(iy as f64) * y_frac;
            for ix in 0..dim.0 {
                let x = convert::<_, F>(ix as f64) * x_frac;
                data.push(surface.get(x, y));
            }
        }
        
        Heightmap {
            dim,
            len_frac: (x_frac, y_frac),
            size,
            range: range(&data),
            data,
        }
    }
    
    pub fn add_surface(&mut self, surface: &dyn UnboundedSurface<F>, mult: F) {
        for iy in 0..self.dim.1 {
            for ix in 0..self.dim.0 {
                let (x, y) = self.coord_of(ix, iy);
                let h = self.get(ix, iy);
                self.set(ix, iy, h + mult * surface.get(x, y));
            }
        }
        self.range = range(&self.data);
    }
}

// conversions
impl<F: RealField> Heightmap<F> {
    // Convert to a HeightField
    pub fn to_heightfield(&self) -> HeightField<F> {
        let rows = Dynamic::new(self.dim.1 as usize);
        let cols = Dynamic::new(self.dim.0 as usize);
        let heights = DMatrix::from_row_slice_generic(rows, cols, &self.data[..]);
        let scale = Vector3::new(self.size.0, convert::<f64, F>(1.0), self.size.1);
        HeightField::new(heights, scale)
    }

    // Use naive conversion of heightmap to a `TriMesh`.
    // 
    // This approach does not cull any vertices, so the result may have a
    // very high triangle count.
    pub fn to_trimesh(&self) -> TriMesh<F> {
        let one: F = na::one();
        let (x_divs, y_divs) = (self.dim.0 - 1, self.dim.1 - 1);
        
        // code adapted from ncollide::procedural::unit_quad:
        let (x_step, y_step) = self.len_frac;
        let tx_step = one / convert(x_divs as f64);
        let ty_step = one / convert(y_divs as f64);

        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        let mut tex_coords = Vec::new();

        // create the vertices
        for iy in 0..self.dim.1 {
            for ix in 0..self.dim.0 {
                let fy: F = convert(iy as f64);
                let fx: F = convert(ix as f64);

                let v = Point3::new(fx * x_step, fy * y_step, self.get(iy, ix));
                vertices.push(v);
                tex_coords.push(Point2::new(one - fx * tx_step, one - fy * ty_step))
            }
        }

        // create triangles
        let ws = self.dim.0;
        
        let dl_triangle = |iy: u32, ix: u32| -> Point3<u32> {
            Point3::new((iy + 1) * ws + ix, iy * ws + ix, (iy + 1) * ws + ix + 1)
        };

        let ur_triangle = |iy: u32, ix: u32| -> Point3<u32> {
            Point3::new(iy * ws + ix, iy * ws + (ix + 1), (iy + 1) * ws + ix + 1)
        };

        for iy in 0..y_divs {
            for ix in 0..x_divs {
                // build two triangles...
                triangles.push(dl_triangle(iy, ix));
                triangles.push(ur_triangle(iy, ix));
            }
        }

        let mut mesh = TriMesh::new(
            vertices,
            None,
            Some(tex_coords),
            Some(IndexBuffer::Unified(triangles)),
        );
        mesh.recompute_normals();
        mesh
    }
}

// calculate (min, max) of data
// Note: can't use Iterator::min/max because it requires Ord bound
fn range<F: RealField>(s: &[F]) -> (F, F) {
    let mut min = F::max_value();
    let mut max = F::min_value();
    for x in s.iter() {
        min = min.min(*x);
        max = max.max(*x);
    }
    (min, max)
}
