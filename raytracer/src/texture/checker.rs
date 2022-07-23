use crate::Hit::{Color, Point3};

use super::{solid_color::SolidColor, Texture};

pub struct Checker<T: Texture> {
    pub odd: T,
    pub even: T,
}

impl<T: Texture> Checker<T> {
    pub fn new(c1: Color, c2: Color) -> Checker<SolidColor> {
        Checker {
            odd: SolidColor::new(c1.x, c1.y, c1.z),
            even: SolidColor::new(c2.x, c2.y, c2.z),
        }
    }
}

impl<T: Texture> Texture for Checker<T> {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Option<crate::Hit::Color> {
        let sines = (p.x() * 10.).sin() * (p.y() * 10.).sin() * (p.z() * 10.).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
