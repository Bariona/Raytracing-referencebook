pub mod checker;
pub mod image_texture;
pub mod obj_texture;
pub mod perlin;
pub mod solid_color;

use crate::Hit::{Color, Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Option<Color>;
}
