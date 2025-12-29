use std::f64::consts::PI;

use num_traits::{FloatConst, float::Float};

pub fn degrees_to_radians<T: Float + FloatConst>(degrees: T) -> T {
    degrees * T::PI() / T::from(180).unwrap()
}

pub fn radians_to_decrees<T: Float + FloatConst>(radians: T) -> T {
    radians * T::from(180).unwrap() / T::PI()
}
