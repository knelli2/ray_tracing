use derivative::Derivative;
use std::{any::Any, sync::Arc};

use crate::{
    aabb::Aabb, hittable::{HitRecord, Hittable}, interval::Interval, materials::{material::SharedMaterial, metal::Metal}, point::Point, ray::Ray, vec3::Vec3
};

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct Sphere {
    center: Ray,
    radius: f32,
    #[derivative(Default(value = "Arc::new(Metal::default())"))]
    #[derivative(Debug = "ignore")]
    material: SharedMaterial,
    bbox: Aabb,
}

impl Sphere {
    /// Stationary Sphere
    pub fn new(center: Point, radius: f32, material: SharedMaterial) -> Self {
        let radius = radius.max(0.);
        let radius_all_dir = Vec3::new(radius, radius, radius);
        Self {
            center: Ray::new_t0(center, Point::zero()),
            radius: radius,
            material,
            bbox: Aabb::new_pt(center - radius_all_dir, center + radius_all_dir),
        }
    }
    /// Moving Sphere
    pub fn new_moving(center_start: Point, center_end: Point, radius: f32, material: SharedMaterial) -> Self {
        let radius = radius.max(0.);
        let radius_all_dir = Vec3::new(radius, radius, radius);
        let center = Ray::new_t0(center_start, center_end - center_start);
        let center_at_0 = center.at(0.);
        let center_at_1 = center.at(1.);
        let aabb_at_0 = Aabb::new_pt(center_at_0 - radius_all_dir, center_at_0 + radius_all_dir);
        let aabb_at_1 = Aabb::new_pt(center_at_1 - radius_all_dir, center_at_1 + radius_all_dir);

        Self {
            center: center,
            radius: radius.max(0.),
            material,
            bbox: Aabb::new_surround(&aabb_at_0, &aabb_at_1),
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
            return HitRecord::new_miss();
        }

        let discriminant_sqrt = discriminant.sqrt();

        let mut root = (h - discriminant_sqrt) / a;
        if !ray_t.contains(root) {
            root = (h + discriminant_sqrt) / a;
            if !ray_t.contains(root) {
                return HitRecord::new_miss();
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

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
