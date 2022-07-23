use crate::texture::{solid_color::SolidColor, Texture};

use super::{Color, Material, Ray, ScatterRecord, Vec3};

pub struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new_color(c: Color) -> Isotropic<SolidColor> {
        Isotropic {
            albedo: SolidColor::new(c.x, c.y, c.z),
        }
    }
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r_in: &super::Ray, rec: &super::HitRecord) -> Option<super::ScatterRecord> {
        let scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p).unwrap();
        Some(ScatterRecord {
            attenuation,
            is_specular: true,
            specular_ray: scattered,
            pdf_ptr: None,
        })
    }
}
