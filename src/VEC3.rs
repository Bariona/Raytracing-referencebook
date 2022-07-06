use std::ops::{Neg, AddAssign, MulAssign, DivAssign};
use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone, Debug)]
pub struct vec3 {
    pub x : f64, 
    pub y : f64,
    pub z : f64,
}

impl vec3 {
    // fn x(&self) -> f64 { self.x }
    // fn y(&self) -> f64 { self.y }
    // fn z(&self) -> f64 { self.z }
    fn len_square(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn len(&self) -> f64 {
        self.len_square().sqrt()
    }
    fn dot(&self, rhs: &mut vec3) -> vec3 {
        vec3 {
            x : self.x * rhs.x,
            y : self.y * rhs.y,
            z : self.z * rhs.z,
        }
    }
    fn unit_length(self) -> vec3 {
        self / self.len()
    }
}

impl Add for vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul for vec3 {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Mul<f64> for vec3 {
    type Output = Self;
    
    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl Div<f64> for vec3 {
    type Output = Self;
    
    fn div(self, other: f64) -> Self::Output {
        self * (1.0 / other)
    }
}

impl Neg for vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

// &mut
impl AddAssign for vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl MulAssign<f64> for vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl DivAssign<f64> for vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}
