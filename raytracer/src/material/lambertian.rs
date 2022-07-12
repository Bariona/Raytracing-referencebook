use std::sync::Arc;

use crate::texture::{Texture, solid_color::SolidColor};

use super::{Color, HitRecord, Material, Ray, ScatterRecord, Vec3};

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new_tx(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
    pub fn new(al: Color) -> Self {
        Self { 
            albedo: Arc::new(SolidColor::new(al.x, al.y, al.z))
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        let attenuation = (self.albedo).value(rec.u, rec.v, &rec.p).unwrap();

        Some(ScatterRecord {
            attenuation,
            scattered,
        })
    }
}
