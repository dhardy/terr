// Copyright 2019 Diggory Hardy
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Mesh manipulation

use nalgebra as na;
use na::{convert, RealField, geometry::{Point2, Point3}};
use ncollide3d::procedural::IndexBuffer;
use crate::unbounded::UnboundedSurface;

/// Type of tri-mesh used for drawing a terrain
pub use ncollide3d::procedural::TriMesh;


/// Sample a mesh on a surface
/// 
/// This gives a generic method of creating a mesh from a surface function,
/// but is not always the fastest or most accurate method of constructing a
/// mesh (check for more specific implementations).
/// 
/// Does not perform any mesh optimisation.
pub trait SampleMesh<F: RealField> {
    /// Sample a [`TriMesh`] on the given `surface` over the rectangle from
    /// `start` to `start + size` with the given number of `subdivs`-isions
    /// (i.e. with `(subdivs.0 + 1) * (subdivs.1 + 1)` sample points).
    fn sample_mesh(&self, start: (F, F), size: (F, F), subdivs: (u32, u32)) -> TriMesh<F>;
}


impl<F: RealField, U: UnboundedSurface<F>> SampleMesh<F> for U {
    fn sample_mesh(&self, start: (F, F), size: (F, F), subdivs: (u32, u32)) -> TriMesh<F> {
        let one: F = na::one();
        let np = (subdivs.0 + 1, subdivs.1 + 1);
        
        // code adapted from ncollide::procedural::unit_quad:
        let tx_step = one / convert(subdivs.0 as f64);
        let ty_step = one / convert(subdivs.1 as f64);
        let x_step = tx_step * size.0;
        let y_step = ty_step * size.1;

        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        let mut tex_coords = Vec::new();

        // create the vertices
        for iy in 0..np.1 {
            for ix in 0..np.0 {
                let fy: F = convert(iy as f64);
                let fx: F = convert(ix as f64);

                let v = Point3::new(
                        start.0 + fx * x_step,
                        start.1 + fy * y_step,
                        self.get(fy, fx));
                vertices.push(v);
                tex_coords.push(Point2::new(one - fx * tx_step, one - fy * ty_step))
            }
        }

        // create triangles
        let ws = np.0;
        
        let dl_triangle = |iy: u32, ix: u32| -> Point3<u32> {
            Point3::new((iy + 1) * ws + ix, iy * ws + ix, (iy + 1) * ws + ix + 1)
        };

        let ur_triangle = |iy: u32, ix: u32| -> Point3<u32> {
            Point3::new(iy * ws + ix, iy * ws + (ix + 1), (iy + 1) * ws + ix + 1)
        };

        for iy in 0..subdivs.1 {
            for ix in 0..subdivs.0 {
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
