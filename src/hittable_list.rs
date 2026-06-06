use std::any::Any;
use std::sync::Arc;

use log::debug;

use crate::aabb::Aabb;
use crate::hittable::{Hittable, SharedHittable};
use crate::interval::Interval;
use crate::{hittable::HitRecord, point::Point, ray::Ray, vec3::Vec3};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<SharedHittable>,
    bbox: Aabb,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> HitRecord {
        let mut last_record = HitRecord::new_miss();
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            let object_record = object.hit(ray, Interval::new(ray_t.min, closest_so_far));
            if object_record.hit {
                closest_so_far = object_record.t;
                last_record = object_record;
            }
        }

        last_record
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

impl HittableList {
    pub fn from_hittable(object: SharedHittable) -> Self {
        Self {
            bbox: object.bounding_box(),
            objects: vec![object],
        }
    }

    pub fn add(&mut self, object: SharedHittable) {
        self.bbox.extend(&object.bounding_box());
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.bbox = Aabb::default();
    }
}
