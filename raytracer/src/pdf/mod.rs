use std::f64::consts::PI;

use crate::Hit::{random_double, Vec3};

pub mod cospdf;
pub mod hittablepdf;
pub mod mixturepdf;

pub trait PDF {
    fn generate(&self) -> Vec3;
    fn value(&self, direction: &Vec3) -> f64;
}

pub fn random_cosine_direction() -> Vec3 {
    // 在半球内随机, 关于cos(theta)分布, theta为与normal所成角
    let r1 = random_double();
    let r2 = random_double();
    let z = (1. - r2).sqrt();

    let phi = 2. * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1 = random_double();
    let r2 = random_double();
    let z = 1. + r2 * ((1. - radius.powi(2) / distance_squared).sqrt() - 1.);

    let phi = 2. * PI * r1;
    let x = phi.cos() * (1. - z.powi(2)).sqrt();
    let y = phi.sin() * (1. - z.powi(2)).sqrt();

    Vec3::new(x, y, z)
}
