use std::sync::Arc;

use crate::texture::{solid_color::SolidColor, Texture};

use super::{Color, Material, Ray, ScatterRecord, Vec3};

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new_color(c: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(c.x, c.y, c.z)),
        }
    }
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &super::Ray, rec: &super::HitRecord) -> Option<super::ScatterRecord> {
        let scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p).unwrap();
        Some(ScatterRecord {
            attenuation,
            scattered,
        })
    }
}
