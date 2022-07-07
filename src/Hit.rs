pub use crate::basic::{
    VEC3::{Point3, Vec3},
    RAY::Ray,
};

#[derive(Default, Copy, Clone, Debug)]
pub struct HitRecord {
    pub p: Point3, // 碰撞点 
    pub normal: Vec3, // 碰撞点的法向量(与Ray的方向相反)
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

// ---- Hittable trait ----
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl  HittableList {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_rec = None;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if let Some(tmp_rec) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = tmp_rec.t;
                hit_rec = Some(tmp_rec);
            }
        }

        hit_rec
    }
}