use image::{DynamicImage, GenericImageView, Pixel};

use crate::{clamp, Hit::Color};

use super::Texture;

pub struct ImageTexture {
    pub width: u32,
    pub height: u32,
    pub data: DynamicImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let data = image::open(filename).unwrap();
        let width = data.width();
        let height = data.height();
        Self {
            width,
            height,
            data,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &crate::Hit::Point3) -> Option<crate::Hit::Color> {
        let u = clamp(u, 0., 1.);
        let v = 1. - clamp(v, 0., 1.);

        let mut i = (u * self.width as f64) as u32;
        let mut j = (v * self.height as f64) as u32;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1. / 255.;
        let pixel = self.data.get_pixel(i, j).to_rgb();

        Some(Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        ))
    }
}
