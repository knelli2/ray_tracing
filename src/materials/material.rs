use std::{cell::RefCell, rc::Rc};

use crate::{color::Color, hittable::HitRecord, ray::Ray};

#[derive(Default, Debug)]
pub struct MaterialRecord {
    pub scattered: bool,
    pub attenuation: Color,
    pub scattered_ray: Ray,
}

impl MaterialRecord {
    pub fn new(scattered: bool, attenuation: Color, scattered_ray: Ray) -> Self {
        MaterialRecord {
            scattered,
            attenuation,
            scattered_ray,
        }
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> MaterialRecord {
        MaterialRecord {
            scattered: false,
            attenuation: Color::black(),
            scattered_ray: Ray::default(),
        }
    }
}

pub type SharedMaterial = Rc<RefCell<dyn Material>>;
