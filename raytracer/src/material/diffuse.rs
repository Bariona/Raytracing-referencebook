use std::sync::Arc;

use crate::texture::{solid_color::SolidColor, Texture};

use super::{Color, Material};

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(c: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(c.x, c.y, c.z)),
        }
    }
}
impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &super::Ray, _rec: &super::HitRecord) -> Option<super::ScatterRecord> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: &super::Point3) -> Option<super::Color> {
        self.emit.value(u, v, p)
    }
}
