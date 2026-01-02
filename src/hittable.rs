use derivative::Derivative;
use rayon::prelude::*;
use std::{any::Any, sync::Arc};

use crate::{
    interval::Interval,
    materials::{
        material::{Material, SharedMaterial},
        metal::Metal,
    },
    point::Point,
    ray::Ray,
    vec3::Vec3,
};

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct HitRecord {
    pub hit: bool,
    pub point: Point,
    pub normal: Vec3,
    #[derivative(Default(value = "Arc::new(Metal::default())"))]
    #[derivative(Debug = "ignore")]
    pub material: SharedMaterial,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    /// Set HitRecord.hit = true, everything else default
    pub fn new_hit() -> Self {
        let mut record = Self::default();
        record.hit = true;
        record
    }

    pub fn new(
        hit: bool,
        point: Point,
        normal: Vec3,
        material: SharedMaterial,
        t: f32,
        front_face: bool,
    ) -> Self {
        Self {
            hit,
            point,
            normal,
            material,
            t,
            front_face,
        }
    }

    /// outward_normal is assumed to be unit length
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> HitRecord;
    fn as_any(&self) -> &dyn Any;
}

pub type SharedHittable = Arc<dyn Hittable>;
