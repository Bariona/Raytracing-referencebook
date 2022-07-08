pub use crate::basic::{
    VEC3::{Vec3, Point3, Color},
    RAY::Ray,
};

pub trait Material {
    pub fn scatter(r_in: &Ray, rec: HitRecord, attenuation: Color, scattered: &Ray) -> bool;
}