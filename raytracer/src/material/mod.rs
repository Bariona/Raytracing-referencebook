pub mod dielectric;
pub mod diffuse;
pub mod isotropic;
pub mod lambertian;
pub mod matel;

use crate::pdf::cospdf::CosPDF;
pub use crate::{
    basic::{
        RAY::Ray,
        VEC3::{Color, Point3, Vec3},
    },
    Hit::HitRecord,
};

pub struct ScatterRecord {
    // 1 - attenuation := 光线的被吸收量, attenuation在某种意义上和材料的albedo(反射率)等价
    // 理解: 物体呈现颜色(比如红色) 不是因为本身发光, 而是善于吸收其他颜色光, 而不怎么吸收红光
    //       所以红颜色材质for example: attenuation = (0.8, 0.3, 0.3), i.e. 吸收70%的G和B, 仅吸收20%的R
    pub attenuation: Color,
    pub is_specular: bool,
    pub specular_ray: Ray,
    pub pdf_ptr: Option<CosPDF>,
}

pub struct ONB { // 一组正交基
    pub axis: [Vec3; 3],
}

impl ONB {
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }
    // pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
    //     a * self.axis[0] + b * self.axis[1] + c * self.axis[2]
    // }
    pub fn local(&self, a: Vec3) -> Vec3 {
        a.x() * self.axis[0] + a.y() * self.axis[1] + a.z() * self.axis[2]
    }
    pub fn build(n: &Vec3) -> Self {
        let mut axis = [Vec3::default(); 3];
        axis[2] = n.unit_vector();
        let a = if axis[2].x().abs() > 0.9 {
            Vec3::new(0., 1., 0.)
        } else {
            Vec3::new(1., 0., 0.)
        };
        axis[1] = Vec3::cross(&axis[2], a).unit_vector();
        axis[0] = Vec3::cross(&axis[2], axis[1]);
        Self { axis }
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;

    fn scatter_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> Option<f64> {
        None
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Option<Color> {
        Some(Color::new(0., 0., 0.))
    }
}
