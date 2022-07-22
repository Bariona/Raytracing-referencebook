use std::sync::Arc;

use crate::Hit::{Hittable, Point3, Vec3};

use super::PDF;

pub struct HittablePDF {
    origin: Point3,
    ptr: Arc<dyn Hittable>,
}

impl HittablePDF {
    pub fn new(ptr: Arc<dyn Hittable>, origin: Vec3) -> Self {
        Self { origin, ptr }
    }
}

impl PDF for HittablePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.origin, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.origin)
    }
}
