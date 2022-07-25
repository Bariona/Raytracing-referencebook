use std::sync::Arc;

use image::RgbImage;

use crate::Hit::Color;

use super::Texture;

pub struct ObjTexture {
    pub u1: f64,
    pub v1: f64,
    pub u2: f64,
    pub v2: f64,
    pub u3: f64,
    pub v3: f64,
    pub img: Arc<RgbImage>,
}

impl ObjTexture {
    pub fn new(img: Arc<RgbImage>, u1: f64, v1: f64, u2: f64, v2: f64, u3: f64, v3: f64) -> Self {
        Self {
            u1,
            v1,
            u2,
            v2,
            u3,
            v3,
            img
        }
    }
}

impl Texture for ObjTexture {
    fn value(&self, beta: f64, gamma: f64, _p: &crate::Hit::Point3) -> Option<crate::Hit::Color> {
        // if self.u > 1. || self.u < 0. || self.v > 1. || self.v < 0. {
        //     panic!();
        // }
        // let u = clamp(self.u, 0., 1.);
        // let v = clamp(self.v, 0., 1.);

        let alpha = 1. - beta - gamma;
        let u = self.u1 * alpha + self.u2 * beta + self.u3 * gamma;
        let v = self.v1 * alpha + self.v2 * beta + self.v3 * gamma;
        let mut i = (u * ((self.img.width()) as f64)) as u32;
        let mut j = ((1. - v) * ((self.img.height()) as f64)) as u32;

        // let mut i = (self.u * self.img.width() as f64) as u32;
        // let mut j = ((1. - self.v) * self.img.height() as f64) as u32;

        if i >= self.img.width() {
            i = self.img.width() - 1;
        }
        if j >= self.img.height() {
            j = self.img.height() - 1;
        }

        let color_scale = 1. / 255.;
        let pixel = self.img.get_pixel(i, j);

        Some(Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        ))
    }
}
