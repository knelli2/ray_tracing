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
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> MaterialRecord {
        let mut reflected = reflect(ray.direction(), &hit_record.normal);
        reflected = reflected.unit() + Vec3::random_unit() * self.fuzz;

        let mut material_record = MaterialRecord::default();
        material_record.scattered_ray = Ray::new(hit_record.point, reflected, ray.time());
        material_record.attenuation = self.albedo;
        material_record.scattered = material_record
            .scattered_ray
            .direction()
            .dot(&hit_record.normal)
            > 0.;

        material_record
    }
}
