use derivative::Derivative;
use std::{any::Any, sync::Arc};

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    materials::{material::SharedMaterial, metal::Metal},
    point::Point, ray::Ray,
};

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct Sphere {
    center: Ray,
    radius: f32,
    #[derivative(Default(value = "Arc::new(Metal::default())"))]
    #[derivative(Debug = "ignore")]
    material: SharedMaterial,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: SharedMaterial) -> Self {
        Self {
            center: Ray::new_t0(center, Point::zero()),
            radius: radius.max(0.),
            material,
        }
    }
    pub fn new_moving(center1: Point, center2: Point, radius: f32, material: SharedMaterial) -> Self {
        Self {
            center: Ray::new_t0(center1, center2 - center1,),
            radius: radius.max(0.),
            material,
        }
    }

    pub fn center(&self) -> &Ray {
        &self.center
    }
}

impl Hittable for Sphere {
    /// With P = Q + t*d
    /// |(C - P)|^2 = r2
    /// t^2 d*d - 2td*(C-Q) + |(C-Q)|^2 - r^2 = 0
    ///
    /// a = d*d
    /// b = 2d*(C-Q)
    /// h = -b/2
    /// c = |(C-Q)|^2 - r^2
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval) -> HitRecord {
        let d = ray.direction();
        let center = self.center.at(ray.time());
        let center_minus_Q =  center - *ray.origin();

        let a = d.length_squared();
        let h = d.dot(&center_minus_Q);
        let c = center_minus_Q.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0. {
            return HitRecord::default();
        }

        let discriminant_sqrt = discriminant.sqrt();

        let mut root = (h - discriminant_sqrt) / a;
        if !ray_t.contains(root) {
            root = (h + discriminant_sqrt) / a;
            if !ray_t.contains(root) {
                return HitRecord::default();
            }
        }

        let mut record = HitRecord::new_hit();
        record.t = root;
        record.point = ray.at(record.t);
        let normal = (record.point - center) / self.radius;
        record.set_face_normal(ray, &normal);
        record.material = self.material.clone();

        record
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
