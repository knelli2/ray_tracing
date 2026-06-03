use crate::{
    color::Color, hittable::HitRecord, materials::material::{Material, MaterialRecord}, ray::Ray, utils::random_float, vec3::{Vec3, reflect, refract}
};

#[derive(Default, Debug)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Dielectric { refraction_index }
    }
}

fn reflectance(cos: f32, refraction_index: f32) -> f32 {
    let r0 = (1. - refraction_index) / (1. + refraction_index);
    let r0 = r0 * r0;

    r0 + (1. - r0) * (1. - cos).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> MaterialRecord {
        let index = if hit_record.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction().unit();
        let cos_theta = -unit_direction.dot(&hit_record.normal).min(1.0);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = index * sin_theta > 1.0;

        if cannot_refract || reflectance(cos_theta, index) > random_float() {
            MaterialRecord::new(
                true,
                Color::white(),
                Ray::new(
                    hit_record.point,
                    reflect(&unit_direction, &hit_record.normal),
                    ray.time(),
                ),
            )
        } else {
            MaterialRecord::new(
                true,
                Color::white(),
                Ray::new(
                    hit_record.point,
                    refract(&unit_direction, &hit_record.normal, index),
                    ray.time(),
                ),
            )
        }
    }
}
