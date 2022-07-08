pub mod lambertian;
pub mod matel;

pub use crate::{
    basic::{
        VEC3::{Vec3, Point3, Color},
        RAY::Ray,
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
