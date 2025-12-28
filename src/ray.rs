use crate::point::Point;
use crate::vec3::Vec3;

use std::default::Default;
use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Point, direction: &Vec3) -> Self {
        Self {
            origin: *origin,
            direction: *direction,
        }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
      &self.direction
    }

    pub fn at(&self, t: f32) -> Point {
      self.origin + self.direction * t
    }
}

// impl Ray {
//   pub fn
// }
