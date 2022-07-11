use super::VEC3::{Point3, Vec3};

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

    pub fn new(ori: Point3, di: Vec3) -> Self {
        Self { orig: ori, dir: di }
    }
}
