use super::{Color, HitRecord, Material, Ray, ScatterRecord, Vec3};
use crate::basic::random_double;

pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(index: f64) -> Self {
        Self { ir: index }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
        r + (1. - r) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::new(1., 1., 1.); // those kind of material absorbs nothing !

        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        }; // 折射率之比(要判断光线是在光密还是在光疏部分)

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = Vec3::dot(&-unit_direction, &rec.normal).min(1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let judnot = refraction_ratio * sin_theta > 1.; // 直接是全反射的情况

        let dir: Vec3;
        if judnot || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double() { // 比较二者的光强大小决定选哪条射线
            dir = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            dir = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        // let scattered = Ray::new(rec.p, dir, r_in.time());

        Some(ScatterRecord {
            attenuation,
            is_specular: true,
            specular_ray: Ray::new(rec.p, dir, r_in.time()),
            pdf_ptr: None,
        })
    }
}

// reference: https://blog.csdn.net/masilejfoaisegjiae/article/details/104614953