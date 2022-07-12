use crate::Hit::{Color, Point3};

use super::Texture;

pub struct SolidColor {
    pub color_val: Color,
}

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_val: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Option<Color> {
        Some(self.color_val)
    }
}
