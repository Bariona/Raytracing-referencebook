use crate::Hit::{Hittable, Point3, Vec3};

use super::PDF;

#[derive(Clone)]
pub struct HittablePDF<'a, H: Hittable> {
    origin: Point3,
    ptr: &'a H,
}

impl<'a, H: Hittable> HittablePDF<'a, H> {
    pub fn new(ptr: &'a H, origin: Vec3) -> Self {
        Self { origin, ptr }
    }
}

impl<'a, H: Hittable> PDF for HittablePDF<'a, H> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.origin, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.origin)
    }
}
