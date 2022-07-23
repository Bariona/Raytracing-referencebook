use std::f64::consts::PI;

use crate::{
    pdf::cospdf::CosPDF,
    texture::{solid_color::SolidColor, Texture},
};

use super::{Color, HitRecord, Material, Ray, ScatterRecord, Vec3};

pub struct Lambertian<T: Texture> {
    pub albedo: T, // albedo 为实现了Texture的一个泛型
}

impl<T: Texture> Lambertian<T> {
    pub fn new_texture(albedo: T) -> Self {
        Self { albedo }
    }
    pub fn new(al: Color) -> Lambertian<SolidColor> {
        Lambertian {
            albedo: SolidColor::new(al.x, al.y, al.z),
        }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            is_specular: false,
            specular_ray: Ray::default(),
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p).unwrap(),
            pdf_ptr: Some(CosPDF::new(&rec.normal)),
        })
    }
    fn scatter_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> Option<f64> {
        let cosine = Vec3::dot(&rec.normal, &scattered.direction().unit_vector());
        let consine = cosine.max(0.);

        Some(consine / PI)
    }
}
