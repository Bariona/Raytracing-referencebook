pub mod RAY;
pub mod VEC3;
pub mod camera;

use rand::Rng;

// use crate::{PI, INF};

// pub fn degree_to_radians(degree: f64) -> f64 {
//     degree * PI / 180.
// }

pub fn random_double() -> f64 {
    rand::thread_rng().gen::<f64>()
}

// pub fn random_range(min: f64, max: f64) -> f64 {
//     min + (max - min) * random_double()
// }