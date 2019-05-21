// Copyright 2019 Diggory Hardy
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Functionality based on heightmaps

use nalgebra as na;
use na::{RealField, geometry::{Point2, Point3}};
use ncollide3d::procedural::{TriMesh, IndexBuffer};

pub use displacement::midpoint_displacement;

mod displacement;

/// Our heightmap representation.
pub struct Heightmap<F> {
    stride: usize,
    data: Vec<F>,
}

impl<F> Heightmap<F> {
    /// Get length in first dimension.
    pub fn len0(&self) -> usize { self.stride }
    /// Get length in second dimension.
    pub fn len1(&self) -> usize { self.data.len() / self.stride }
    
    /// Set value at the given coordinates.
    /// 
    /// Requires `x < self.len0()` and `y < self.len1()`.
    pub fn set(&mut self, x: usize, y: usize, val: F) {
        assert!(x < self.stride);
        self.data[x + y * self.stride] = val;
    }
}

impl<F: Clone> Heightmap<F> {
    /// Construct a new Heightmap with size `x×y` and all elements initialised to v.
    pub fn new(x: usize, y: usize, v: F) -> Self {
        Heightmap {
            stride: x,
            data: vec![v; x * y],
        }
    }
    
    /// Get value at the given coordinates.
    /// 
    /// Requires `x < self.len0()` and `y < self.len1()`.
    pub fn get(&self, x: usize, y: usize) -> F {
        assert!(x < self.stride);
        self.data[x + y * self.stride].clone()
    }
}

impl<F: RealField> Heightmap<F> {
    // Use naive conversion of heightmap to a `TriMesh`.
    // 
    // This approach does not cull any vertices, so the result may have a
    // very high triangle count.
    pub fn to_trimesh(&self, width: F, height: F) -> TriMesh<F> {
        let usubdivs = self.stride - 1;
        let vsubdivs = self.data.len() / self.stride - 1;
        
        // code adapted from ncollide::procedural::unit_quad:
        let twstep = na::one::<F>() / na::convert(usubdivs as f64);
        let thstep = na::one::<F>() / na::convert(vsubdivs as f64);
        let wstep = twstep * width;
        let hstep = thstep * height;
        let cw = na::convert::<f64, F>(0.5) * width;
        let ch = na::convert::<f64, F>(0.5) * height;

        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        let mut tex_coords = Vec::new();

        // create the vertices
        for i in 0usize..vsubdivs + 1 {
            for j in 0usize..usubdivs + 1 {
                let ni: F = na::convert(i as f64);
                let nj: F = na::convert(j as f64);

                let v = Point3::new(
                        nj * wstep - cw,
                        ni * hstep - ch,
                        self.get(i, j));
                vertices.push(v);
                let _1 = na::one::<F>();
                tex_coords.push(Point2::new(_1 - nj * twstep, _1 - ni * thstep))
            }
        }

        // create triangles
        fn dl_triangle(i: u32, j: u32, ws: u32) -> Point3<u32> {
            Point3::new((i + 1) * ws + j, i * ws + j, (i + 1) * ws + j + 1)
        }

        fn ur_triangle(i: u32, j: u32, ws: u32) -> Point3<u32> {
            Point3::new(i * ws + j, i * ws + (j + 1), (i + 1) * ws + j + 1)
        }

        for i in 0usize..vsubdivs {
            for j in 0usize..usubdivs {
                // build two triangles...
                triangles.push(dl_triangle(i as u32, j as u32, (usubdivs + 1) as u32));
                triangles.push(ur_triangle(i as u32, j as u32, (usubdivs + 1) as u32));
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
