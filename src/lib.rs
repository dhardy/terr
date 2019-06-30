// Copyright 2019 Diggory Hardy
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Terrain tools
//! 
//! ## Representation of terrains
//!
//! Many digital models of terrains have been proposed in the literature:
//! pure functional representation (`h: ℝ² → ℝ`),
//! discrete heightfields aka Digital Elevation Models (potentially with
//! non-linear interpolation) for `O(n²)` memory usage,
//! layered representations of functions or heightfields representing heights
//! of multiple materials in a fixed sequence,
//! functional volumetric representation (`μ: ℝ³ → M` for material `M`),
//! voxels for `O(n³)` memory usage, and hybrid representations (e.g. a multi-
//! layered heightfield with local exceptions).
//! 
//! Currently this library is limited to single-layer heightfields.

/// Types usable as an approximation of the real numbers, ℝ.
/// 
/// Currently this is fixed as `nalgebra::RealField`.
pub use nalgebra::RealField;

pub mod unbounded;
pub mod heightmap;
pub mod mesh;
