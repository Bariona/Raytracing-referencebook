pub mod dielectric;
pub mod lambertian;
pub mod matel;

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
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}
