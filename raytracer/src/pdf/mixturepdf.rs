use std::sync::Arc;

use crate::Hit::random_double;

use super::PDF;

pub struct MixturePDF {
    p: [Arc<dyn PDF>; 2],
}

impl MixturePDF {
    pub fn new(p0: Arc<dyn PDF>, p1: Arc<dyn PDF>) -> Self {
        Self {
            p: [p0, p1],
        }
    }
}

impl PDF for MixturePDF {
    fn value(&self, direction: &crate::Hit::Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
    fn generate(&self) -> crate::Hit::Vec3 {
        if random_double() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}