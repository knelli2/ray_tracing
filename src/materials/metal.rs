use crate::{
    color::Color,
    hittable::HitRecord,
    materials::material::{Material, MaterialRecord},
    ray::Ray,
    vec3::{Vec3, reflect},
};

#[derive(Default, Debug)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> MaterialRecord {
        let reflected = reflect(ray.direction(), &hit_record.normal);

        let mut material_record = MaterialRecord::default();
        material_record.scattered_ray = Ray::new(hit_record.point, reflected);
        material_record.attenuation = self.albedo;
        material_record.scattered = true;

        material_record
    }
}
