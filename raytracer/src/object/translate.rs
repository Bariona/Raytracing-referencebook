use crate::{
    bvh::aabb::AABB,
    Hit::{Hittable, Ray, Vec3},
};

// 物体空间平移 offset
#[derive(Debug)]
pub struct Translate<H: Hittable> {
    ptr: H,
    offset: Vec3,
}

impl<H: Hittable> Translate<H> {
    pub fn new(ptr: H, offset: Vec3) -> Self {
        Self { ptr, offset }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, r: &crate::Hit::Ray, t_min: f64, t_max: f64) -> Option<crate::Hit::HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if let Some(mut rec) = self.ptr.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            let outward = rec.normal;
            rec.set_face_normal(&moved_r, &outward);

            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::bvh::aabb::AABB> {
        if let Some(output_box) = self.ptr.bounding_box(time0, time1) {
            let output_box =
                AABB::new(output_box.mini + self.offset, output_box.maxi + self.offset);
            Some(output_box)
        } else {
            None
        }
    }
    fn random(&self, origin: &Vec3) -> Vec3 {
        self.ptr.random(&(*origin - self.offset))
    }
    fn pdf_value(&self, o: &crate::Hit::Point3, v: &Vec3) -> f64 {
        self.ptr.pdf_value(&(*o - self.offset), v)
        // println!("{:?} {:?} res = {}", *o - self.offset, v, res);
    }
}
