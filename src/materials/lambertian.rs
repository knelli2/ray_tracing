use crate::{
    color::Color,
    hittable::HitRecord,
    materials::material::{Material, MaterialRecord},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Default, Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
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
        material_record.attenuation = self.albedo;
        material_record.scattered = true;

        material_record
    }
}
