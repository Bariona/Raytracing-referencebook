use std::{f64::consts::PI, sync::Arc};

use crate::texture::{solid_color::SolidColor, Texture};

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
        let direction = basis.local(Vec3::random_cosine_direction());
        let scattered = Ray::new(rec.p, direction.unit_vector(), r_in.time());
        Some(ScatterRecord {
            scattered,
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p).unwrap(),
            pdf: Vec3::dot(&basis.w(), &scattered.direction()) / PI,
        })

        // let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // if scatter_direction.near_zero() {
        //     scatter_direction = rec.normal;
        // }

        // let scattered = Ray::new(rec.p, scatter_direction.unit_vector(), r_in.time());
        // let attenuation = (self.albedo).value(rec.u, rec.v, &rec.p).unwrap();

        // Some(ScatterRecord {
        //     attenuation,
        //     scattered,
        //     //pdf: Vec3::dot(&rec.normal, &scattered.direction()) / PI,
        //     pdf: 0.5 / PI,
        // })
    }
    fn scatter_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> Option<f64> {
        let cosine = Vec3::dot(&rec.normal, &scattered.direction().unit_vector());
        let consine = cosine.max(0.);

        // println!("{}",cosine);
        Some(consine / PI)
    }
}
