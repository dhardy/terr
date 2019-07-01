// Copyright 2019 Diggory Hardy
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module concerns surfaces represented by a function `h: ℝ² → ℝ`.

mod perlin;

pub use perlin::{Perlin, PerlinError};

use crate::RealField;

/// A map of 2D coordinate to height
pub trait UnboundedSurface<F: RealField> {
    /// Determine the height of the terrain at the given coordinate.
    fn get(&self, x: F, y: F) -> F;
}


/// An infinite, flat surface.
#[derive(Debug, Clone, Copy, Default)]
pub struct Flat<F: RealField>(F);

impl<F: RealField> Flat<F> {
    /// Construct with the given elevation
    pub fn new(elevation: F) -> Self {
        Flat(elevation)
    }
}

impl<F: RealField> UnboundedSurface<F> for Flat<F> {
    fn get(&self, _: F, _: F) -> F {
        self.0
    }
}
