pub mod solid_color;
pub mod checker;

use crate::Hit::{Color, Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Option<Color>;
}

