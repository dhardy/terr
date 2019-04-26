// Copyright 2019 Diggory Hardy

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Height-map generation utilies.

use alga::general::RealField;
use nalgebra::{Scalar, DimName, Vector};
use nalgebra::base::{storage::Storage, U1};
use rand::{Rng, distributions::Distribution};
use std::marker::PhantomData;

/// Height-map generation trait
/// 
/// This trait allows a generator to be built via multiple modifiers, much like
/// with the `Iterator` trait.
pub trait Height {
    /// Scalar type
    type N: Scalar + RealField;
    /// Dimension of ground coordinates
    type D: DimName;
    
    /// Calculate the height at the given coordinate, assuming the coordinate
    /// is within bounds and the height is real.
    fn sample<S, R>(&self, coord: Vector<Self::N, Self::D, S>, rng: &mut R) -> Self::N
    where S: Storage<Self::N, Self::D, U1>, R: Rng;
    
    /// Add uncorrelated noise to the result
    fn add_noise<T>(self, distr: T) -> NoiseLayer<Self::N, Self::D, T, Self>
    where T: Distribution<Self::N>, Self: Sized {
        NoiseLayer::new(distr, self)
    }
}


/// An infinite plain with height zero.
pub struct Plain<N: Scalar + RealField, D: DimName> {
    _p_n: PhantomData<N>,
    _p_d: PhantomData<D>,
}

impl<N: Scalar + RealField, D: DimName> Plain<N, D> {
    /// Create a plain generator
    pub fn new() -> Self {
        Plain { _p_n: Default::default(), _p_d: Default::default() }
    }
}

impl<N: Scalar + RealField, D: DimName> Height for Plain<N, D> {
    type N = N;
    type D = D;
    
    fn sample<S, R>(&self, _coord: Vector<Self::N, Self::D, S>, _rng: &mut R) -> Self::N
    where S: Storage<Self::N, Self::D, U1>, R: Rng
    {
        N::zero()
    }
}


/// Add a layer of noise over a height-map
pub struct NoiseLayer<N, D, T, H>
where
    N: Scalar + RealField,
    D: DimName,
    T: Distribution<N>,
    H: Height<N=N, D=D>,
{
    _p_n: PhantomData<N>,
    _p_d: PhantomData<D>,
    distr: T,
    height: H,
}

impl<N, D, T, H> NoiseLayer<N, D, T, H>
where
    N: Scalar + RealField,
    D: DimName,
    T: Distribution<N>,
    H: Height<N=N, D=D>,
{
    pub fn new(distr: T, height: H) -> Self {
        NoiseLayer { _p_n: Default::default(), _p_d: Default::default(), distr, height }
    }
}

impl<N, D, T, H> Height for NoiseLayer<N, D, T, H>
where
    N: Scalar + RealField,
    D: DimName,
    T: Distribution<N>,
    H: Height<N=N, D=D>,
{
    type N = N;
    type D = D;
    
    fn sample<S, R>(&self, coord: Vector<Self::N, Self::D, S>, rng: &mut R) -> Self::N
    where S: Storage<Self::N, Self::D, U1>, R: Rng
    {
        self.height.sample(coord, rng) + self.distr.sample(rng)
    }
}
