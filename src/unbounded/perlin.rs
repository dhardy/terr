// Copyright 2019 Diggory Hardy
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::RealField;
use crate::unbounded::UnboundedSurface;
use nalgebra::try_convert;


/// A Perlin noise generator
#[derive(Debug, Clone)]
pub struct Perlin<F: RealField> {
    scale: F,
    mask: u32,
    gradient: Vec<[F; 2]>,  // random unit gradient vectors
}

#[derive(Debug, Clone, Copy)]
pub enum PerlinError {
    NotPowerOf2,
}

impl<F: RealField> Perlin<F> {
    /// Construct a Perlin noise generator
    /// 
    /// The spatial scale (lacunarity) can be adjusted via the `scale`
    /// parameter. Each coordinate is first multiplied by `scale` when sampling.
    /// 
    /// A fixed number of gradients, `n`, is sampled immediately. These are
    /// sampled via the `sampler` function. Examples: `UnitCircle.sample(rng)`
    /// produces classic Perlin noise. Exponentially distributed slopes can lead
    /// to more interesting terrain:
    /// 
    /// ```rust
    /// let mut g = UnitCircle.sample(rng);
    /// let mut s = Exp1.sample(rng);
    /// [g[0] * s, g[1] * s]
    /// ```
    pub fn new<S: FnMut() -> [F; 2]>(scale: F, n: usize, mut sampler: S) -> Result<Self, PerlinError> {
        if n != 2usize.pow(n.trailing_zeros()) {
            return Err(PerlinError::NotPowerOf2);
        }
        
        let gradient = (0..n).into_iter()
            .map(|_| sampler())
            .collect::<Vec<[F; 2]>>();
        
        Ok(Perlin { scale, mask: (n - 1) as u32, gradient })
    }
}

impl<F: RealField> UnboundedSurface<F> for Perlin<F> {
    fn get(&self, x: F, y: F) -> F {
        let p = (x * self.scale, y * self.scale);
        let p0 = (p.0.floor(), p.1.floor());
        let p1 = (p0.0 + F::one(), p0.1 + F::one());
        
        let r0 = (p.0 - p0.0, p.1 - p0.1);
        let r1 = (p.0 - p1.0, p.1 - p1.1);
        
        // Get four random indices. This is probably overkill.
        let to_u64 = |x| -> u64 { try_convert::<_, f64>(x).unwrap() as u64 };
        let i00 = (to_u64(p0.0)).wrapping_add(to_u64(p0.1) << 32);
        let i01 = i00.wrapping_add(0x1);
        let i10 = i00.wrapping_add(0x1_0000_0000);
        let i11 = i00.wrapping_add(0x1_0000_0001);
        // TODO: use SIMD
        let m = self.mask;
        let hash = |mut x: u64| {
            // derived from PCG
            x = x.wrapping_mul(14647171131086947261);
            let rot = (x >> 59) as u32;
            let xsh = (((x >> 18) ^ x) >> 27) as u32;
            (xsh.rotate_right(rot) & m) as usize
        };
        let i00 = hash(i00);
        let i01 = hash(i01);
        let i10 = hash(i10);
        let i11 = hash(i11);
        
        let s = |x| x*x*(F::from_f32(3.0).unwrap() - F::from_f32(2.0).unwrap() * x);
        let s0 = s(r0.0);
        let s1 = s(r0.1);
        
        let lerp = |t, a, b| a + t * (b - a);
        let dp = |u: (F, F), v: [F; 2]| u.0 * v[0] + u.1 * v[1];
        
        let u = dp(r0, self.gradient[i00]);
        let v = dp((r1.0, r0.1), self.gradient[i01]);
        let a = lerp(s0, u, v);
        
        let u = dp((r0.0, r1.1), self.gradient[i10]);
        let v = dp(r1, self.gradient[i11]);
        let b = lerp(s0, u, v);
        
        lerp(s1, a, b)
    }
}
