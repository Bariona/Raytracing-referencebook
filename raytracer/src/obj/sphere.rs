use std::{f64::consts::PI, sync::Arc};

use super::super::Hit::{HitRecord, Hittable};
pub use crate::basic::{
    RAY::Ray,
    VEC3::{Point3, Vec3},
};
use crate::{bvh::aabb::AABB, Hit::Material};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn get_sphere_uv(p: &Point3) -> Option<[f64; 2]> {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y()).acos();
        let phi = p.x().atan2(-p.z()) + PI;

        Some([phi / 2. / PI, theta / PI])
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
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

        let outward_normal = (r.at(root) - self.center) / self.radius;
        let tup = Self::get_sphere_uv(&outward_normal).unwrap();

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vec3::default(),
            front_face: bool::default(),
            mat: (self.mat).clone(),
            u: tup[0],
            v: tup[1],
        };

        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        let cub = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(self.center - cub, self.center + cub))
    }
}
