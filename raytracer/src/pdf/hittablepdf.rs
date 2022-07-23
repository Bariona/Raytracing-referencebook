use crate::Hit::{Hittable, Point3, Vec3};

use super::PDF;

pub struct HittablePDF<H: Hittable> {
    origin: Point3,
    ptr: H,
}

impl<H: Hittable> HittablePDF<H> {
    pub fn new(ptr: H, origin: Vec3) -> Self {
        Self { origin, ptr }
    }
}

impl<H: Hittable> PDF for HittablePDF<H> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.origin, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.origin)
    }
}
