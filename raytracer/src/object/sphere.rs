use std::{f64::consts::PI, f64::INFINITY};

use super::super::Hit::{HitRecord, Hittable};
pub use crate::basic::{
    RAY::Ray,
    VEC3::{Point3, Vec3},
};
use crate::{bvh::aabb::AABB, material::ONB, pdf::random_to_sphere, Hit::Material};

pub struct Sphere<M: Material> {
    pub center: Point3,
    pub radius: f64,
    pub mat: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, mat: M) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
    pub fn get_sphere_uv(p: &Point3) -> Option<[f64; 2]> {
        /*
            p: a given point on the sphere of radius one, centered at the origin.
            u: returned value [0,1] of angle around the Y axis from X=-1.
            v: returned value [0,1] of angle from Y=-1 to Y=+1.
               <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
               <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
               <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
        */

        let theta = (-p.y()).acos();
        let phi = p.x().atan2(-p.z()) + PI;

        Some([phi / 2. / PI, theta / PI])
    }
}

impl<M: Material> Hittable for Sphere<M> {
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

        let mut rec = HitRecord::new(
            root,
            r.at(root),
            Vec3::default(),
            bool::default(),
            &self.mat,
            tup[0],
            tup[1],
        );

        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        let cub = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(self.center - cub, self.center + cub))
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        if self.hit(&Ray::new(*o, *v, 0.), 0.001, INFINITY).is_none() {
            return 0.;
        }

        let cos_theta_max = (1. - self.radius.powi(2) / (self.center - *o).len_square()).sqrt();
        let solid_angle = 2. * PI * (1. - cos_theta_max);

        1. / solid_angle
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let direction = self.center - *o;
        let distance_squared = direction.len_square();
        let uvw = ONB::build(&direction);
        uvw.local(random_to_sphere(self.radius, distance_squared))
    }
}
