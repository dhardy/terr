// Copyright 2019 Diggory Hardy
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Heightmap;
use nalgebra as na;
use na::RealField;
use rand::{Rng, distributions::{Distribution, Standard, Uniform, uniform::SampleUniform}};

/// A generalised Voronoi diagram generator
pub struct Voronoi<F> {
    points: Vec<(F, F)>,
}

impl<F: RealField> Voronoi<F> {
    /// Construct a new diagram with the given points
    /// 
    /// Points are denoted by a pair of coordinates in the range `[0, 1]`.
    pub fn with_points(points: Vec<(F, F)>) -> Self {
        Voronoi { points }
    }
    
    /// Apply to a `Heightmap`
    /// 
    /// The heightmap should be initialised to zero or an existing terrain (for
    /// additive generation). For all points we add the following:
    /// 
    /// ```none
    /// w[0] * d0 + w[1] * d1 + ...
    /// ```
    /// 
    /// where `w` is a list of weights, `f` is the supplied function, `i0` is
    /// the index of the closest point (and `i1` of the next closest, etc.),
    /// `d0` is the distance to the closest point (and `d0` the next, etc.).
    ///
    /// The distances `d0`, `d1`, etc. are calculated via the `dist` function
    /// with type `FnMut(F,F) -> F`: this is passed offsets in `x` and `y`
    /// directions, and returns the combined distance. This function may use
    /// the standard Euclidian metric `|x,y| (x*x + y*y).sqrt()` or may use a
    /// different metric, and may add perturbations to the distance.
    ///
    /// The length of the weight list `w` does not need to equal the number of
    /// points.
    /// 
    /// TODO: optimise (current alg is naive)
    pub fn apply_to<D: FnMut(F, F) -> F>(&self, m: &mut Heightmap<F>, w: &[F], mut dist: D){
        let cells = m.cells();
        let np = self.points.len();
        let nw = w.len().min(np);
        let mut d = vec![F::zero(); self.points.len()];
        
        for iy in 0..cells.1 {
            for ix in 0..cells.0 {
                for i in 0..np {
                    let p = self.points[i];
                    let c = m.coord_of(ix, iy);
                    d[i] = dist(p.0 - c.0, p.1 - c.1);
                }
                d.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mut h = m.get(ix, iy);
                for i in 0..nw {
                    h += w[i] * d[i];
                }
                m.set(ix, iy, h);
            }
        }
    }
}

impl<F: RealField + SampleUniform> Voronoi<F> where Standard: Distribution<F> {
    /// Construct a new diagram, generating `num` random points.
    /// 
    /// Coordinates are sampled from the range available on the heightmap.
    pub fn random<R: Rng + ?Sized>(m: &Heightmap<F>,
            num: usize, rng: &mut R) -> Self
    {
        let size = m.size();
        let half: F = na::convert(0.5);
        let x_range = Uniform::new(-half * size.0, half * size.0);
        let y_range = Uniform::new(-half * size.1, half * size.1);
        Voronoi {
            points: (0..num).map(|_| (rng.sample(&x_range), rng.sample(&y_range))).collect(),
        }
    }
}
