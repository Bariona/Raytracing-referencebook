#![allow(clippy::many_single_char_names)]
use ndarray::Array3;
use rand::{thread_rng, Rng};

use crate::Hit::{Color, Point3};

use super::Texture;

const POINT_COUNT: usize = 256;

#[derive(Default)]
pub struct Perlin {
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
    pub ranvec: Vec<Point3>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec = vec![Point3::default(); POINT_COUNT];
        for item in ranvec.iter_mut().take(POINT_COUNT) {
            *item = Point3::random_range(-1., 1.);
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Self {
            perm_x,
            perm_y,
            perm_z,
            ranvec,
        }
    }

    fn trilinear_inerp(a3: Array3<Point3>, u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1 - i) as f64 * (1. - u))
                        * (j as f64 * v + (1 - j) as f64 * (1. - v))
                        * (k as f64 * w + (1 - k) as f64 * (1. - w))
                        * Point3::dot(&a3[[i, j, k]], &Point3::new(u - i as f64, v - j as f64, w - k as f64));
                }
            }
        }
        accum
    }

    pub fn turb(&self, p: &Point3, depth: isize) -> f64 {
        let mut accum = 0.;
        let mut temp = p.clone();
        let mut weigth = 1.;

        for _i in 0..depth {
            accum += weigth * Self::noise(&self, &temp);
            weigth *= 0.5;
            temp *= 2.;
        }

        accum.abs()
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        u = u.powi(2) * (3. - 2. * u);
        v = v.powi(2) * (3. - 2. * v);
        w = w.powi(2) * (3. - 2. * w);

        let i = p.x().floor() as isize;
        let j = p.y().floor() as isize;
        let k = p.z().floor() as isize;

        let mut a3 = Array3::<Point3>::default((2, 2, 2));

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    a3[[di, dj, dk]] = self.ranvec[self.perm_x
                        [((i + di as isize) & 255) as usize]
                        ^ self.perm_y[((j + dj as isize) & 255) as usize]
                        ^ self.perm_z[((k + dk as isize) & 255) as usize]];
                }
            }
        }
        Self::trilinear_inerp(a3, u, v, w)
    }

    pub fn perlin_generate_perm() -> Vec<usize> {
        let mut p = vec![0; POINT_COUNT];

        for (i, item) in p.iter_mut().enumerate().take(POINT_COUNT) {
            *item = i as usize;
        }

        Perlin::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut Vec<usize>, n: usize) {
        for i in (1..n).rev() {
            let j = thread_rng().gen_range(0..i);
            p.swap(i, j);
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Option<Color> {
        Some(Color::new(1., 1., 1.) * self.noise.turb(&(self.scale * (*p)), 7))
    }
}