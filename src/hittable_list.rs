use std::{cell::RefCell, rc::Rc};

use log::debug;

use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::{hittable::HitRecord, point::Point, ray::Ray, vec3::Vec3};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<RefCell<dyn Hittable>>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> HitRecord {
        let mut last_record = HitRecord::default();
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            let object_record = object
                .borrow()
                .hit(ray, Interval::new(ray_t.min, closest_so_far));
            if object_record.hit {
                closest_so_far = object_record.t;
                last_record = object_record;
            }
        }

        last_record
    }
}

impl HittableList {
    pub fn from_hittable(object: Rc<RefCell<dyn Hittable>>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn add(&mut self, object: Rc<RefCell<dyn Hittable>>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}
