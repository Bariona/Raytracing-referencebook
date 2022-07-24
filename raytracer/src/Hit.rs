use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::bvh::aabb::{surrounding_box, AABB};
pub use crate::{
    basic::{
        random_double,
        RAY::Ray,
        VEC3::{Color, Point3, Vec3},
    },
    material::{dielectric::Dielectric, lambertian::Lambertian, matel::Metal, Material},
    obj::sphere::Sphere,
};

pub struct HitRecord<'a> {
    pub p: Point3,        // 碰撞点
    pub normal: Vec3,     // 碰撞点的单 位 法 向 量(与Ray的方向相反)
    pub t: f64,           // 表示 p = Ray(t)
    pub front_face: bool, // 是否Ray来自外侧
    pub mat: &'a dyn Material,
    pub u: f64, // u, v 物体表面 surface的coordinates
    pub v: f64, // u, v \in [0, 1]
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f64,
        p: Point3,
        normal: Vec3,
        front_face: bool,
        mat: &'a dyn Material,
        u: f64,
        v: f64,
    ) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
            mat,
            u,
            v,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

// ---- Hittable trait ----
pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
    fn pdf_value(&self, _o: &Point3, _v: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, _o: &Vec3) -> Vec3 {
        Vec3::new(1., 0., 0.)
    }
}

// ---- Hittable List ----
// 用于存储 Hittable 的 struct

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    // 这里必须要用Arc来实现多态(因为有Vec的存在, 需要实现一个Vector中存多种类型)
}

impl HittableList {
    pub fn add(&mut self, element: Arc<dyn Hittable>) {
        self.objects.push(element);
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut first_box = true;
        let mut tmp_box = AABB::default();

        for obj in &self.objects {
            match obj.bounding_box(time0, time1) {
                Some(tmp_AABB) => {
                    if first_box {
                        tmp_box = tmp_AABB;
                        first_box = false;
                    } else {
                        tmp_box = surrounding_box(tmp_box, tmp_AABB);
                    }
                }
                None => {
                    return None;
                }
            }
        }
        Some(tmp_box)
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let weigh = 1. / self.objects.len() as f64;
        let mut sum = 0.;

        for object in self.objects.iter() {
            sum += weigh * object.pdf_value(o, v);
        }

        sum
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        self.objects[thread_rng().gen_range(0..self.objects.len())].random(o)
    }
}
