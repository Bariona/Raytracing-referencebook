#![allow(clippy::many_single_char_names)]
use crate::{
    bvh::aabb::AABB,
    material::{HitRecord, Material, Point3, Vec3},
    Hit::Hittable,
};

const EPS: f64 = 1e-10;

pub struct Triangle<M: Material> {
    pub v0: Point3,
    pub v1: Point3,
    pub v2: Point3,
    pub mat: M,
}

impl<M: Material> Triangle<M> {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, mat: M) -> Self {
        Self { v0, v1, v2, mat }
    }
    pub fn get_normal(&self) -> Vec3 {
        (self.v1 - self.v0).cross(self.v2 - self.v0).unit_vector()
    }
    pub fn inside(&self, p: Point3) -> bool {
        let n = self.get_normal();
        Vec3::dot(&n, &(self.v1 - self.v0).cross(p - self.v0)) >= 0.
            && Vec3::dot(&n, &(self.v2 - self.v1).cross(p - self.v1)) >= 0.
            && Vec3::dot(&n, &(self.v0 - self.v2).cross(p - self.v2)) >= 0.
    }
}

impl<M: Material> Hittable for Triangle<M> {
    fn hit(&self, r: &crate::material::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let n = self.get_normal();
        let NdotRaydir = n.dot(&r.direction());
        if NdotRaydir.abs() < EPS {
            return None;
        }
        let t = Vec3::dot(&(self.v0 - r.orig), &n) / NdotRaydir;

        if t < t_min || t > t_max {
            return None;
        }

        let p = r.at(t);
        if !self.inside(p) {
            return None;
        }

        let a1 = self.v0.x - self.v1.x;
        let b1 = self.v0.x - self.v2.x;
        let c1 = self.v0.x - p.x;
        let a2 = self.v0.y - self.v1.y;
        let b2 = self.v0.y - self.v2.y;
        let c2 = self.v0.y - p.y;

        let u = (c1 * b2 - b1 * c2) / (a1 * b2 - b1 * a2);
        let v = (a1 * c2 - a2 * c1) / (a1 * b2 - b1 * a2);

        let mut rec = HitRecord::new(t, p, Vec3::default(), bool::default(), &self.mat, u, v);
        rec.set_face_normal(r, &n);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        let eps = Point3::new(EPS, EPS, EPS);
        Some(AABB::new(
            Point3::new(
                self.v0.x.min(self.v1.x.min(self.v2.x)),
                self.v0.y.min(self.v1.y.min(self.v2.y)),
                self.v0.z.min(self.v1.z.min(self.v2.z)),
            ) - eps,
            Point3::new(
                self.v0.x.max(self.v1.x.max(self.v2.x)),
                self.v0.y.max(self.v1.y.max(self.v2.y)),
                self.v0.z.max(self.v1.z.max(self.v2.z)),
            ) + eps,
        ))
    }
}
