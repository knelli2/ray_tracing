use std::f32::consts::PI;

use num_traits::{PrimInt, float::Float};
use rand::{Rng, distr::uniform::SampleUniform, rngs::ThreadRng};

#[derive(Default, Debug)]
pub struct Degrees {
    pub value: f32,
}

impl Degrees {
    pub fn new(value: f32) -> Self {
        Degrees { value }
    }
}

#[derive(Default, Debug)]
pub struct Radians {
    pub value: f32,
}

impl Radians {
    pub fn new(value: f32) -> Self {
        Radians { value }
    }
}

pub fn degrees_to_radians(degrees: &Degrees) -> Radians {
    Radians::new(degrees.value * PI / 180.)
}

pub fn radians_to_decrees(radians: &Radians) -> Degrees {
    Degrees::new(radians.value * 180. / PI)
}

pub fn random_float<T: Float + SampleUniform>() -> T {
    rand::rng().random_range(T::zero()..T::one())
}

pub fn random_float_range<T: Float + SampleUniform>(min: T, max: T) -> T {
    rand::rng().random_range(min..max)
}

pub fn random_int_range<T: PrimInt + SampleUniform>(min: T, max: T) -> T {
    rand::rng().random_range(min..max)
}