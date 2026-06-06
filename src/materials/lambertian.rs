use std::{fmt::Debug, sync::Arc};

use crate::{
    color::Color,
    hittable::HitRecord,
    materials::material::{Material, MaterialRecord},
    ray::Ray,
    textures::{solid::Solid, texture::SharedTexture},
    vec3::Vec3,
};

// #[derive(Default, Debug)]
pub struct Lambertian {
    texture: SharedTexture,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian {
            texture: Arc::new(Solid::new(albedo)),
        }
    }

    pub fn new_texture(texture: SharedTexture) -> Self {
        Self { texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> MaterialRecord {
        let mut direction = hit_record.normal + Vec3::random_unit();

        // Catch degenerate scatter direction
        if direction.near_zero() {
            direction = hit_record.normal;
        }

        let mut material_record = MaterialRecord::default();
        material_record.scattered_ray = Ray::new(hit_record.point, direction, ray.time());
        material_record.attenuation =
            self.texture
                .color(hit_record.u, hit_record.v, &hit_record.point);
        material_record.scattered = true;

        material_record
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Lambertian::new(Color::black())
    }
}
