pub use crate::basic::{
    VEC3::{Vec3, Point3},
    RAY::Ray,
    
};
use super::super::Hit::{HitRecord, Hittable};

#[derive(Default, Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Point3, 
    pub radius: f64,
    // pub hit: HitRecord,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().len_square();
        let half_b = Vec3::dot(oc, r.direction());
        let c = oc.len_square() - self.radius * self.radius;

        let discrim = half_b * half_b - a * c;
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
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}