use crate::Hit::random_double;

use super::PDF;

pub struct MixturePDF<T1: PDF, T2: PDF> {
    p0: T1,
    p1: T2,
}

impl<T1: PDF, T2: PDF> MixturePDF<T1, T2> {
    pub fn new(p0: T1, p1: T2) -> Self {
        Self { p0, p1 }
    }
}

impl<T1: PDF, T2: PDF> PDF for MixturePDF<T1, T2> {
    fn value(&self, direction: &crate::Hit::Vec3) -> f64 {
        // println!("{}", self.p[0].value(direction));
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }
    fn generate(&self) -> crate::Hit::Vec3 {
        if random_double() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
