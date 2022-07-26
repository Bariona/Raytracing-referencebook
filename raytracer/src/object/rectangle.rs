#![allow(clippy::many_single_char_names)]
use std::f64::INFINITY;

use crate::{
    basic::random_range,
    bvh::aabb::AABB,
    Hit::{HitRecord, Hittable, Material, Point3, Ray, Vec3},
};

pub struct Rectanglexy<M: Material> {
    mat: M,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl<M: Material> Rectanglexy<M> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: M) -> Self {
        Self {
            mat,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl<M: Material> Hittable for Rectanglexy<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max || t.is_nan() {
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
        let mut rec = HitRecord::new(
            t,
            r.at(t),
            Vec3::default(),
            bool::default(),
            &self.mat,
            u,
            v,
        );
        rec.set_face_normal(r, &outward_normal);
        // rec.mat = self.mat;
        // rec.p = r.at(t);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::bvh::aabb::AABB> {
        Some(AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }

    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        let getn = self.hit(&Ray::new(*origin, *v, 0.), 0.001, INFINITY);

        if getn.is_none() {
            return 0.;
        }
        let rec = getn.unwrap();
        let area = (self.x1 - self.x0) * (self.y1 - self.y0);
        let distance_squared = rec.t.powi(2) * v.len_square();
        let cos = Vec3::dot(v, &rec.normal).abs() / v.len();

        // println!("{} {} {} {}", rec.t, distance_squared, cos, area);
        distance_squared / (cos * area)
    }
    fn random(&self, origin: &Vec3) -> Vec3 {
        let point = Point3::new(
            random_range(self.x0, self.x1),
            random_range(self.y0, self.y1),
            self.k,
        );
        point - *origin
    }
}

pub struct Rectanglexz<M: Material> {
    mat: M,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl<M: Material> Rectanglexz<M> {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: M) -> Self {
        Self {
            mat,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl<M: Material> Hittable for Rectanglexz<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max || t.is_nan() {
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

        let mut rec = HitRecord::new(
            t,
            r.at(t),
            Vec3::default(),
            bool::default(),
            &self.mat,
            u,
            v,
        );

        rec.set_face_normal(r, &outward_normal);
        // rec.mat = self.mat;
        // rec.p = r.at(t);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::bvh::aabb::AABB> {
        Some(AABB::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }

    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        let getn = self.hit(&Ray::new(*origin, *v, 0.), 0.001, INFINITY);

        if getn.is_none() {
            return 0.;
        }
        let rec = getn.unwrap();
        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared = rec.t.powi(2) * v.len_square();
        let cos = Vec3::dot(v, &rec.normal).abs() / v.len();

        // println!("{} {} {} {}", rec.t, distance_squared, cos, area);
        distance_squared / (cos * area)
    }
    fn random(&self, origin: &Vec3) -> Vec3 {
        let point = Point3::new(
            random_range(self.x0, self.x1),
            self.k,
            random_range(self.z0, self.z1),
        );
        point - *origin
    }
}

pub struct Rectangleyz<M: Material> {
    mat: M,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl<M: Material> Rectangleyz<M> {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: M) -> Self {
        Self {
            mat,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl<M: Material> Hittable for Rectangleyz<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max || t.is_nan() {
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

        let mut rec = HitRecord::new(
            t,
            r.at(t),
            Vec3::default(),
            bool::default(),
            &self.mat,
            u,
            v,
        );

        rec.set_face_normal(r, &outward_normal);
        // rec.mat = self.mat;
        // rec.p = r.at(t);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::bvh::aabb::AABB> {
        Some(AABB::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        let getn = self.hit(&Ray::new(*origin, *v, 0.), 0.001, INFINITY);

        if getn.is_none() {
            return 0.;
        }
        let rec = getn.unwrap();
        let area = (self.y1 - self.y0) * (self.z1 - self.z0);
        let distance_squared = rec.t.powi(2) * v.len_square();
        let cos = Vec3::dot(v, &rec.normal).abs() / v.len();

        // println!("{} {} {} {}", rec.t, distance_squared, cos, area);
        distance_squared / (cos * area)
    }
    fn random(&self, origin: &Vec3) -> Vec3 {
        let point = Point3::new(
            self.k,
            random_range(self.y0, self.y1),
            random_range(self.z0, self.z1),
        );
        point - *origin
    }
}
