use std::{f64::consts::PI, sync::Arc};

use crate::{texture::{solid_color::SolidColor, Texture}, pdf::{random_cosine_direction, cospdf::{self, CosPDF}}};

use super::{Color, HitRecord, Material, Ray, ScatterRecord, Vec3, ONB};

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new_texture(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
    pub fn new(al: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(al.x, al.y, al.z)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let basis = ONB::build(&rec.normal);
        let direction = basis.local(random_cosine_direction());
        let scattered = Ray::new(rec.p, direction.unit_vector(), r_in.time());
        // println!("{}", Vec3::dot(&basis.w(), &scattered.direction()) / PI);
        Some(ScatterRecord {
            is_specular: false,
            specular_ray: Ray::default(),
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p).unwrap(),
            pdf_ptr: Some(Arc::new(CosPDF::new(&rec.normal))),
        })
    }
    fn scatter_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> Option<f64> {
        let cosine = Vec3::dot(&rec.normal, &scattered.direction().unit_vector());
        let consine = cosine.max(0.);

        // println!("{}",cosine);
        Some(consine / PI)
    }
}
