pub use super::VEC3::{Point3, Vec3};
pub use super::RAY::Ray;

#[derive(Default, Copy, Clone, Debug)]
pub struct HitRecord {
    pub p: Point3, // 碰撞点 
    pub normal: Vec3, // 法向量
    pub t: f64, // 表示 p = Ray(t) 
    pub front_face: bool, // 是否Ray来自外侧
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) -> () {
        self.front_face = Vec3::dot(r.direction(), *outward_normal) < 0.;
        self.normal = if self.front_face { 
            *outward_normal 
        } else { 
            -*outward_normal 
        };
    }
}

// ---- trait ----
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}