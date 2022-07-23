use super::super::Hit::{HitRecord, Hittable};
pub use crate::basic::{
    RAY::Ray,
    VEC3::{Point3, Vec3},
};
use crate::{
    bvh::aabb::{surrounding_box, AABB},
    Hit::Material, 
};

pub struct MoveSphere<M: Material> {
    pub center0: Point3,
    pub center1: Point3,
    pub radius: f64,
    pub time0: f64,
    pub time1: f64,
    pub mat: M,
    // pub hit: HitRecord,
}

impl<M: Material> MoveSphere<M> {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat: M,
    ) -> Self {
        Self {
            center0,
            center1,
            radius,
            time0,
            time1,
            mat,
        }
    }
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + (time - self.time0) / (self.time1 - self.time0) * (self.center1 - self.center0)
    }
}

impl<M: Material> Hittable for MoveSphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().len_square();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.len_square() - self.radius * self.radius;

        let discrim = half_b.powi(2) - a * c;
        if discrim < 0. {
            return None;
        }

        let sqrtd = discrim.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vec3::default(),
            front_face: bool::default(),
            mat: (self.mat).clone(),
            u: 0.,
            v: 0.,
        };
        let outward_normal = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let cub = Vec3::new(self.radius, self.radius, self.radius);
        let box0 = AABB::new(self.center(time0) - cub, self.center(time0) + cub);
        let box1 = AABB::new(self.center(time1) - cub, self.center(time1) + cub);

        Some(surrounding_box(box0, box1))
    }
}
