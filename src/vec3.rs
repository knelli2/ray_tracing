#![allow(unused)]
use std::{
    default::Default,
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    process::Output,
};

use num_traits::{Float, NumAssignRef, One, Zero};
use rand::distr::uniform::SampleUniform;

use crate::utils::{random_float, random_float_range};

pub trait Vec3Trait: Float + NumAssignRef + SampleUniform {}
impl<T: Float + NumAssignRef + SampleUniform> Vec3Trait for T {}

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3<T = f32>
where
    T: Vec3Trait,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

// ==================
// Construction/Conversion
// ==================

impl<T: Vec3Trait> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn random() -> Vec3<T> {
        Vec3::new(random_float(), random_float(), random_float())
    }

    pub fn random_range(min: T, max: T) -> Vec3<T> {
        Vec3::new(
            random_float_range(min, max),
            random_float_range(min, max),
            random_float_range(min, max),
        )
    }

    pub fn random_unit() -> Vec3<T> {
        loop {
            let trial = Self::random_range(-T::one(), T::one());
            let trial_length = trial.length();
            // Prevent against zero division
            if trial_length > T::from(10).unwrap() * T::min_positive_value() {
                return trial / trial_length;
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3<T>) -> Vec3<T> {
        let random_unit = Vec3::random_unit();
        if random_unit.dot(normal) > T::zero() {
            random_unit
        } else {
            -random_unit
        }
    }
}

impl<T: Vec3Trait> From<(T, T, T)> for Vec3<T> {
    fn from(coord: (T, T, T)) -> Self {
        Vec3 {
            x: coord.0,
            y: coord.1,
            z: coord.2,
        }
    }
}

impl<T: Vec3Trait> From<[T; 3]> for Vec3<T> {
    fn from(coord: [T; 3]) -> Self {
        Vec3 {
            x: coord[0],
            y: coord[1],
            z: coord[2],
        }
    }
}

// ==================
// Ops
// ==================

impl<T: Vec3Trait> Neg for Vec3<T>
where
    T: Vec3Trait + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Vec3Trait> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Vec3Trait> Add<T> for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T: Vec3Trait> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Vec3Trait> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Vec3Trait> Sub<T> for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<T: Vec3Trait> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Vec3Trait> Mul for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: rhs.x * self.x,
            y: rhs.y * self.y,
            z: rhs.z * self.z,
        }
    }
}

impl<T: Vec3Trait> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl<T: Vec3Trait> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: Vec3Trait> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Vec3Trait> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// ==================
// Zero/One
// ==================

impl<T: Vec3Trait> Vec3<T> {
    pub fn zero() -> Self {
        Vec3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn unit_x() -> Self {
        Vec3 {
            x: T::one(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn unit_y() -> Self {
        Vec3 {
            x: T::zero(),
            y: T::one(),
            z: T::zero(),
        }
    }

    pub fn unit_z() -> Self {
        Vec3 {
            x: T::zero(),
            y: T::zero(),
            z: T::one(),
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = T::from(1.0e-8).unwrap();
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

impl<T: Vec3Trait> Zero for Vec3<T> {
    fn zero() -> Self {
        Self::zero()
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

// ==================
// Functions
// ==================

impl<T: Vec3Trait> Vec3<T> {
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn make_unit(&mut self) {
        *self /= self.length();
    }
}

pub fn reflect<T: Vec3Trait>(v: &Vec3<T>, normal: &Vec3<T>) -> Vec3<T> {
    *v - *normal * T::from(2.).unwrap() * v.dot(normal)
}
