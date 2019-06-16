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
/// Sample a random fault line, then, for all affected points, sample the signed
/// distance `d` from the fault line (may be negative), and offset the height of
/// affected points by `displacement(d)`.
/// 
/// The parameter `width` specifies the width of the fault; all affected points
/// should have `d` in the range `width.0 <= d <= width.1`. For values `d_`
/// outside this range, `displacement(d_)` should evaluate to 0.
/// 
/// The fault-line is uniformly sampled such that at least one point on the map
/// has `d` within the range specified by `width`.
/// 
/// It is recommended that `displacement` be a smooth function except at the
/// discontinuity `d = 0`. A couple of suggestions follow:
/// 
/// ```rust
/// let h = 1.0;    // height
/// let r = 1.0;    // width
/// let displacement = |d: f64| -> f64 {
///     if d >= 0.0 && d < r {
///         h * (1.0 - (d / r).powi(2)).powi(2)
///     } else {
///         0.0
///     }
/// };
/// ```
/// 
/// ```rust
/// let h = 1.0;    // height
/// let r = -1.0;    // width scale, <0
/// let displacement = |d: f64| -> f64 {
///     if d >= 0.0 {
///         h * (r * d).exp()
///     } else {
///         0.0
///     }
/// };
/// ```
/// 
/// Limitations: (1) only straight faults are generated, (2) the fault
/// displacement is always vertical, (3) fault displacement is uniform along
/// the entire length.
/// 
/// Source: [Gal19], section 3.1.2.
/// 
/// [Gal19]: https://www.doi.org/10.1111/cgf.13657
pub fn fault_displacement<F, R: Rng, D: Fn(F) -> F>(
        m: &mut Heightmap<F>,
        rng: &mut R,
        width: (F, F),
        displacement: D)
where F: RealField + SampleUniform
{
    let (zero, one): (F, F) = (na::zero(), na::one());
    let half: F = na::convert(0.5);
    let sqrt2: F = na::convert(std::f64::consts::SQRT_2);
    
    let xn = m.len0();
    let yn = m.len1();
    let xf: F = one / na::convert(xn as f64);
    let yf: F = one / na::convert(yn as f64);
    
    // Sample fault-line via random direction vector and offset from centre

    let v = rng.sample(UnitCircle);
    let v: (F, F) = (na::convert(v[0]), na::convert(v[1]));
    let offset = rng.gen_range(width.0 - half * sqrt2, width.1 + half * sqrt2);
    let p = (half + offset * v.0, half + offset * v.1);
    
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
