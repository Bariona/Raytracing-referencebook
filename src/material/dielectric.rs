use super::{Material, HitRecord, Vec3, Color, Ray, ScatterRecord};

pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(index: f64) -> Self {
        Self { ir: index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let att = Color::new(1., 1., 1.);
        let refraction_ratio = if rec.front_face { 1. / self.ir } else { self.ir }; // 折射率之比

        let unit_direction = r_in.direction().unit_vector();
        let refracted = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);

        let sca = Ray::new(rec.p, refracted);

        Some(ScatterRecord{ attenuation: att, scattered: sca })
    }
}