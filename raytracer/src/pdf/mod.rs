use std::f64::consts::PI;

use crate::{Hit::{Vec3, random_double}, material::ONB};

pub mod cospdf;
pub mod hittablepdf;
pub mod mixturepdf;

pub trait PDF {
    fn generate(&self) -> Vec3;
    fn value(&self, direction: &Vec3) -> f64;
}

pub fn random_cosine_direction() -> Vec3 { // 在半球内随机, 关于cos(theta)分布, theta为与normal所成角
    let r1 = random_double();
    let r2 = random_double();
    let z = (1. - r2).sqrt();

    let phi = 2. * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

