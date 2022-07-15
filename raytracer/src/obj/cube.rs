use std::sync::Arc;

use crate::{
    bvh::aabb::AABB,
    material::rectangle::{Rectanglexy, Rectanglexz, Rectangleyz},
    Hit::{Hittable, HittableList, Material, Point3},
};

pub struct Cube {
    box_min: Point3,
    box_max: Point3,
    side: HittableList,
}

impl Cube {
    pub fn new(p0: Point3, p1: Point3, ptr: Arc<dyn Material>) -> Self {
        let mut side = HittableList::default();

        side.objects.push(Arc::new(Rectanglexy::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            ptr.clone(),
        )));
        side.objects.push(Arc::new(Rectanglexy::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            ptr.clone(),
        )));

        side.objects.push(Arc::new(Rectanglexz::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            ptr.clone(),
        )));
        side.objects.push(Arc::new(Rectanglexz::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            ptr.clone(),
        )));

        side.objects.push(Arc::new(Rectangleyz::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            ptr.clone(),
        )));
        side.objects.push(Arc::new(Rectangleyz::new(
            p0.y, p1.y, p0.z, p1.z, p0.x, ptr,
        )));

        Self {
            box_min: p0,
            box_max: p1,
            side,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &crate::Hit::Ray, t_min: f64, t_max: f64) -> Option<crate::Hit::HitRecord> {
        self.side.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::bvh::aabb::AABB> {
        let output_box = AABB::new(self.box_min, self.box_max);
        Some(output_box)
    }
}
