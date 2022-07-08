use std::ops::{Neg, AddAssign, MulAssign, DivAssign};
use std::ops::{Add, Sub, Mul, Div};

use super::{random_double, random_range};

#[derive(Default, Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64, 
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;
pub type Point3 = Vec3;

const EPS: f64 = 1e-8;

fn min(x: f64, y: f64) -> f64 {
    if x < y { x } else { y }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }

    pub fn reflect(&self, n: &Vec3) -> Self {
        *self - 2. * Vec3::dot(self, n) * (*n)
    }
    pub fn refract(&self, n: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = min(-Vec3::dot(self, n), 1.);
        let r_out_perp = etai_over_etat * (*self + cos_theta * (*n));
        let r_out_parallel = -(1. - r_out_perp.len_square()).abs().sqrt() * (*n);
        r_out_perp + r_out_parallel
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    
    pub fn near_zero(&self) -> bool {
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }
    pub fn len_square(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn len(&self) -> f64 {
        self.len_square().sqrt()
    }
    pub fn unit_vector(&self) -> Vec3 {
        *self / (&self).len()
    }

    // ---- random ----
    pub fn random() -> Self {
        Self { x: random_double(), y: random_double(), z: random_double() }
    }
    pub fn random_range(min: f64, max: f64) -> Self {
        Self { 
            x: random_range(min, max), 
            y: random_range(min, max), 
            z: random_range(min, max),
        }
    }
    pub fn random_in_unit_sphere() -> Self {
        Vec3::random_range(-1., 1.).unit_vector() * random_double()
    }
    pub fn random_in_hemishpere(normal: &Vec3) -> Self {
        let p = Vec3::random_in_unit_sphere();
        if Vec3::dot(&p, normal) > 0. {
            p
        } else {
            -p
        }
    }
    pub fn random_unit_vector() -> Self {
        Vec3::random_range(-1., 1.).unit_vector()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul for Vec3 {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;
    
    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;
    
    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

// &mut
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}