use crate::texture::{solid_color::SolidColor, Texture};

use super::{Color, Material};

pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(c: Color) -> DiffuseLight<SolidColor> {
        DiffuseLight {
            emit: SolidColor::new(c.x, c.y, c.z),
        }
    }
}
impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _r_in: &super::Ray, _rec: &super::HitRecord) -> Option<super::ScatterRecord> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: &super::Point3) -> Option<super::Color> {
        self.emit.value(u, v, p) // 其实本质是直接返回一个solidcolor的颜色
    }
}
