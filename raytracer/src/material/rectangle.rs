#![allow(clippy::many_single_char_names)]
use std::sync::Arc;

use crate::{bvh::aabb::AABB, Hit::Hittable};

use super::{HitRecord, Material, Point3, Vec3};

pub struct Rectanglexy {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl Rectanglexy {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self {
            mp,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for Rectanglexy {
    fn hit(&self, r: &super::Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let t = t;
        let outward_normal = Vec3::new(0., 0., 1.);
        let mut rec = HitRecord {
            u,
            v,
            t,
            mat: self.mp.clone(),
            p: r.at(t),
            normal: Vec3::default(),
            front_face: bool::default(),
        };
        rec.set_face_normal(r, &outward_normal);
        // rec.mat = self.mp;
        // rec.p = r.at(t);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::bvh::aabb::AABB> {
        Some(AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

pub struct Rectanglexz {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl Rectanglexz {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self {
            mp,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for Rectanglexz {
    fn hit(&self, r: &super::Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let t = t;
        let outward_normal = Vec3::new(0., 1., 0.);
        let mut rec = HitRecord {
            u,
            v,
            t,
            mat: self.mp.clone(),
            p: r.at(t),
            normal: Vec3::default(),
            front_face: bool::default(),
        };
        rec.set_face_normal(r, &outward_normal);
        // rec.mat = self.mp;
        // rec.p = r.at(t);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::bvh::aabb::AABB> {
        Some(AABB::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

pub struct Rectangleyz {
    mp: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl Rectangleyz {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self {
            mp,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for Rectangleyz {
    fn hit(&self, r: &super::Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        let t = t;
        let outward_normal = Vec3::new(1., 0., 0.);
        let mut rec = HitRecord {
            u,
            v,
            t,
            mat: self.mp.clone(),
            p: r.at(t),
            normal: Vec3::default(),
            front_face: bool::default(),
        };
        rec.set_face_normal(r, &outward_normal);
        // rec.mat = self.mp;
        // rec.p = r.at(t);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::bvh::aabb::AABB> {
        Some(AABB::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
