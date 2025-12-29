use std::f64::consts::PI;

use num_traits::{FloatConst, float::Float};
use rand::{Rng, distr::uniform::SampleUniform, rngs::ThreadRng};

pub fn degrees_to_radians<T: Float + FloatConst>(degrees: T) -> T {
    degrees * T::PI() / T::from(180).unwrap()
}

pub fn radians_to_decrees<T: Float + FloatConst>(radians: T) -> T {
    radians * T::from(180).unwrap() / T::PI()
}

pub fn random_float<T: Float + SampleUniform>() -> T {
    rand::rng().random_range(T::zero()..T::one())
}

pub fn random_float_range<T: Float + SampleUniform>(min: T, max: T) -> T {
    rand::rng().random_range(min..max)
}
