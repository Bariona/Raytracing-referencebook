use std::sync::Arc;

use super::{Color, HitRecord, Material, Ray, ScatterRecord, Vec3};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(al: Color, fuz: f64) -> Self {
        Self {
            albedo: al,
            fuzz: if fuz < 1. { fuz } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);
        Some(ScatterRecord {
            attenuation: self.albedo,
            is_specular: true,
            specular_ray: Ray::new(
                rec.p, 
                reflected + self.fuzz * Vec3::random_in_unit_sphere(),
                0.,
            ),
            pdf_ptr: None,
        })
    }
}
