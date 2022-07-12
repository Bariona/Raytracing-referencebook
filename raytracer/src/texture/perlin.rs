use rand::{thread_rng, Rng};

use crate::Hit::{random_double, Color, Point3};

use super::Texture;

const POINT_COUNT: usize = 256;

#[derive(Default)]
pub struct Perlin {
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
    pub ranfloat: Vec<f64>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranfloat = vec![0.; POINT_COUNT];
        for item in ranfloat.iter_mut().take(POINT_COUNT) {
            *item = random_double();
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        // let mut cnt = 0;
        // for i in 0..POINT_COUNT {
        //     if perm_x[i] == perm_y[i] {
        //         cnt += 1;
        //     }
        // }
        // println!("{}", cnt);

        Self {
            perm_x,
            perm_y,
            perm_z,
            ranfloat,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        // hash: get a double \in [0, 1)
        let i = (4. * p.x().abs()) as usize & 255;
        let j = (4. * p.y().abs()) as usize & 255;
        let k = (4. * p.z().abs()) as usize & 255;

        // 对于一个固定的p, 一定要返回固定的noise, 不然就是全一个基调的灰色(白色+黑色的混合结果)
        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
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

impl Texture for Perlin {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Option<crate::Hit::Color> {
        Some(Color::new(1., 1., 1.) * self.noise(p))
    }
}
