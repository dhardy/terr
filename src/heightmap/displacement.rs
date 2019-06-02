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
use rand::{Rng, distributions::Distribution};

#[derive(Debug, PartialEq)]
pub enum Error {
    NotSquare,
    NotPowerOf2Plus1,
}

/// Perform mid-point displacement on the given height-map.
/// 
/// Mid-point displacement is a simple fractal-based procedural generation
/// algorithm for height-maps.
/// 
/// Applicable to square height-maps with side-length `2n + 1`.
/// (TODO: generalise to non-square and non-power-of-2 sizes?)
/// 
/// The four corners of the heightmap should be initialised before performing
/// mid-point displacement.
/// 
/// Parameters:
/// 
/// -   `m` the heightmap (restricted to squares with side-length of `2n + 1`)
/// -   `n0` (normally 0) is the number of midpoint displacement steps to skip
/// -   `distr` is the displacement distribution; for example one may use
///     `Uniform::new(-scale, scale)` or `Normal::new(0.0, scale)` where `scale`
///     is a scaling factor. Note that samples are multiplied by half the side
///     length of the current quad.
pub fn midpoint_displacement<F, R: Rng, D: Distribution<F>>(
        m: &mut Heightmap<F>,
        n0: u32,
        rng: &mut R,
        distr: D) -> Result<(), Error>
where F: RealField + Copy
{
    if m.len0() != m.len1() {
        return Err(Error::NotSquare);
    }
    let size_1 = m.len0() - 1;
    let n = size_1.trailing_zeros();
    if m.len0() != 2usize.pow(n) + 1 {
        return Err(Error::NotPowerOf2Plus1);
    }
    
    let mid2 = |a: F, b: F| { (a + b) * na::convert(0.5) };
    let mid4 = |a, b, c, d| { (a + b + c + d) * na::convert(0.25) };
    
    for i in n0..n {
        let quad_len = 2usize.pow(n - i);
        let mid_len = quad_len / 2;
        let scale: F = na::convert(mid_len as f64);
        
        let mut x = (0, quad_len);
        let mut y = (0, quad_len);
        let adv = |x: &mut (usize, usize)| {
            x.0 = x.1;
            x.1 += quad_len;
            x.1 > size_1
        };
        loop {
            let h00 = m.get(x.0, y.0);
            let h01 = m.get(x.0, y.1);
            let h10 = m.get(x.1, y.0);
            let h11 = m.get(x.1, y.1);
            let h0m = mid2(h00, h01) + scale * distr.sample(rng);
            let h1m = mid2(h10, h11) + scale * distr.sample(rng);
            let hm0 = mid2(h00, h10) + scale * distr.sample(rng);
            let hm1 = mid2(h01, h11) + scale * distr.sample(rng);
            let hmm = mid4(h0m, h1m, hm0, hm1) + scale * distr.sample(rng);
            
            let xm = x.0 + mid_len;
            let ym= y.0 + mid_len;
            m.set(x.0, ym, h0m);
            m.set(x.1, ym, h1m);
            m.set(xm, y.0, hm0);
            m.set(xm, y.1, hm1);
            m.set(xm, ym, hmm);
            
            if adv(&mut y) {
                y = (0, quad_len);
                if adv(&mut x) {
                    break;
                }
            }
        }
    }
    Ok(())
}


/// Perform diamond-square displacement on the given height-map.
/// 
/// Diamond-square displacement is a slightly improved variation of the
/// mid-point displacement algorithm. Reference:
/// [Wikipedia](https://en.wikipedia.org/wiki/Diamond-square_algorithm).
/// 
/// The four corners of the heightmap should be initialised before performing
/// mid-point displacement.
/// 
/// Parameters:
/// 
/// -   `m` the heightmap (restricted to squares with side-length of `2n + 1`)
/// -   `n0` (normally 0) is the number of midpoint displacement steps to skip
/// -   `distr` is the displacement distribution; for example one may use
///     `Uniform::new(-scale, scale)` or `Normal::new(0.0, scale)` where `scale`
///     is a scaling factor. Note that samples are multiplied by half the side
///     length of the current quad.
pub fn diamond_square<F, R: Rng, D: Distribution<F>>(
        m: &mut Heightmap<F>,
        n0: u32,
        rng: &mut R,
        distr: D) -> Result<(), Error>
where F: RealField + Copy
{
    #![allow(non_snake_case)]
    
    if m.len0() != m.len1() {
        return Err(Error::NotSquare);
    }
    let size_1 = m.len0() - 1;
    let n = size_1.trailing_zeros();
    if m.len0() != 2usize.pow(n) + 1 {
        return Err(Error::NotPowerOf2Plus1);
    }
    
    let mid3 = |a, b, c| { (a + b + c) * na::convert(1.0 / 3.0) };
    let mid4 = |a, b, c, d| { (a + b + c + d) * na::convert(0.25) };
    
    for i in n0..n {
        let quad_len = 2usize.pow(n - i);
        let mid_len = quad_len / 2;
        let scale: F = na::convert(mid_len as f64);
        let scale2: F = scale * na::convert(std::f64::consts::SQRT_2);
        
        let mut x = (0, quad_len);
        let mut y = (0, quad_len);
        let adv = |x: &mut (usize, usize)| {
            x.0 = x.1;
            x.1 += quad_len;
            x.1 > size_1
        };
        loop {
            // Displace mid of diamond, and two square points which do not
            // require future diamond mid-points.
            let h00 = m.get(x.0, y.0);
            let h01 = m.get(x.0, y.1);
            let h10 = m.get(x.1, y.0);
            let h11 = m.get(x.1, y.1);
            let xm = x.0 + mid_len;
            let ym= y.0 + mid_len;
            
            let hmm = mid4(h00, h01, h10, h11) + scale * distr.sample(rng);
            let h0m = if x.0 > mid_len {
                let hMm = m.get(x.0 - mid_len, ym);
                mid4(h00, h01, hmm, hMm) + scale2 * distr.sample(rng)
            } else {
                mid3(h00, h01, hmm) + scale2 * distr.sample(rng)
            };
            let hm0 = if y.0 > mid_len {
                let hmM = m.get(xm, y.0 - mid_len);
                mid4(h00, h10, hmm, hmM) + scale2 * distr.sample(rng)
            } else {
                mid3(h00, h10, hmm) + scale2 * distr.sample(rng)
            };
            
            m.set(x.0, ym, h0m);
            m.set(xm, y.0, hm0);
            m.set(xm, ym, hmm);
            
            if adv(&mut y) {
                // Displace square point at end of column
                let h00 = m.get(x.0, y.0);
                let h10 = m.get(x.1, y.0);
                let xm = x.0 + mid_len;
                let hmM = m.get(xm, y.0 - mid_len);
                let hm0 = mid3(h00, h10, hmM) + scale2 * distr.sample(rng);
                m.set(xm, y.0, hm0);
                
                y = (0, quad_len);
                if adv(&mut x) {
                    // Displace square points at end of rows
                    let mut h00;
                    let mut h01 = m.get(x.0, y.0);
                    loop {
                        h00 = h01;
                        h01 = m.get(x.0, y.1);
                        let ym = y.0 + mid_len;
                        let hMm = m.get(x.0 - mid_len, ym);
                        let h0m = mid3(h00, h01, hMm) + scale2 * distr.sample(rng);
                        m.set(x.0, ym, h0m);
                        
                        if adv(&mut y) {
                            break;
                        }
                    }
                    
                    break;
                }
            }
        }
    }
    Ok(())
}
