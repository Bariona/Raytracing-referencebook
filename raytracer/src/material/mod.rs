pub mod dielectric;
pub mod diffuse;
pub mod isotropic;
pub mod lambertian;
pub mod matel;
pub mod rectangle;
pub mod staticmaterial;

pub use crate::{
    basic::{
        RAY::Ray,
        VEC3::{Color, Point3, Vec3},
    },
    Hit::HitRecord,
};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
    pub pdf: f64,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
    fn scatter_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> Option<f64> {
        None
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Option<Color> {
        Some(Color::new(0., 0., 0.))
    }
}
