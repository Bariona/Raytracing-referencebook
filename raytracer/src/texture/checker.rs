use std::sync::Arc;

use crate::Hit::{Color, Point3};

use super::{solid_color::SolidColor, Texture};

pub struct Checker {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl Checker {
    pub fn new(c1: Color, c2: Color) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(c1.x, c1.y, c1.z)),
            even: Arc::new(SolidColor::new(c2.x, c2.y, c2.z)),
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Option<crate::Hit::Color> {
        let sines = (p.x() * 10.).sin() * (p.y() * 10.).sin() * (p.z() * 10.).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
