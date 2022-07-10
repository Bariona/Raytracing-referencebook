use super::VEC3::{Vec3, Point3};

#[derive(Default, Copy, Clone, Debug)]
pub struct Ray {
    pub orig: Point3, 
    pub dir: Vec3,
}

impl Ray {
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig: orig, dir: dir }
    }
}