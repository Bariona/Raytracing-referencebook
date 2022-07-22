use std::f64::consts::PI;

use crate::{material::ONB, Hit::Vec3};

use super::{random_cosine_direction, PDF};

pub struct CosPDF {
    uvw: ONB,
}

impl CosPDF {
    pub fn new(w: &Vec3) -> Self {
        Self { uvw: ONB::build(w) }
    }
}

impl PDF for CosPDF {
    fn value(&self, direction: &crate::Hit::Vec3) -> f64 {
        // 使用蒙特卡罗方法, 计算相应的pdf
        let cos = Vec3::dot(&direction.unit_vector(), &self.uvw.w());
        let cos = cos.max(0.);
        cos / PI
    }
    fn generate(&self) -> Vec3 {
        // 在半球上生成随机
        self.uvw.local(random_cosine_direction())
    }
}
