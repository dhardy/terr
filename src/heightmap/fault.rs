// Copyright 2019 Diggory Hardy
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nalgebra as na;
use na::RealField;
use super::Heightmap;
use rand::{Rng, distributions::{UnitCircle, uniform::SampleUniform}};

/// Displace terrain via a random fault-line
/// 
/// A random fault-line is sampled. For all points, the signed distance `d`
/// from the fault-line is calculated (negative in one direction), and terrain
/// height is increased by the result of `displacement(d)`.
/// 
/// It is recommended that `displacement` be a smooth function except at the
/// discontinuity `d = 0`. A couple of suggestions follow:
/// 
/// ```rust
/// // Params: h>0 is height, r>0 is width
/// let displacement = |d| {
///     if d >= 0.0 && d < r {
///         h * (1.0 - (d / r).powi(2)).powi(2)
///     } else {
///         0.0
///     }
/// }
/// ```
/// 
/// ```rust
/// // Params: h>0 is height, r<0 scales width
/// let displacement = |d| {
///     if d >= 0.0 {
///         h * (r * d).exp()
///     } else {
///         0.0
///     }
/// }
/// ```
/// 
/// Unlike real faults, the fault plane is always vertical, straight, and has
/// uniform displacement along its entire length.
/// 
/// Source: [Gal19], section 3.1.2.
/// 
/// [Gal19]: https://www.doi.org/10.1111/cgf.13657
pub fn fault_displacement<F, R: Rng, D: Fn(F) -> F>(
        m: &mut Heightmap<F>,
        rng: &mut R,
        displacement: D)
where F: RealField + SampleUniform
{
    let (zero, one): (F, F) = (na::zero(), na::one());
    let xn = m.len0();
    let yn = m.len1();
    let xf: F = na::one::<F>() / na::convert(xn as f64);
    let yf: F = na::one::<F>() / na::convert(yn as f64);
    
    // Sample fault-line via random coordinate and direction vector
    let p: (F, F) = (rng.gen_range(zero, one), rng.gen_range(zero, one));
    let v = rng.sample(UnitCircle);
    let v: (F, F) = (na::convert(v[0]), na::convert(v[1]));
    
    for x in 0..xn {
        for y in 0..yn {
            // Take the dot-product of the vector from p to (x, y)
            let dx = na::convert::<_, F>(x as f64) * xf - p.0;
            let dy = na::convert::<_, F>(y as f64) * yf - p.1;
            let d = dx * v.0 + dy * v.1;
            let h = displacement(d);
            if h != zero {
                let h = m.get(x, y) + h;
                m.set(x, y, h);
            }
        }
    }
}
