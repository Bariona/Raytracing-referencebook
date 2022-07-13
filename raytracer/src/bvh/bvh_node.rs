use core::panic;
use std::{cmp::Ordering, sync::Arc};

use rand::{thread_rng, Rng};

use crate::Hit::Hittable;

use super::aabb::{surrounding_box, AABB};

pub struct BvhNode {
    left: Arc<dyn Hittable>, // 指向 Hittable List
    right: Arc<dyn Hittable>,
    box_aabb: AABB,
}

impl BvhNode {
    pub fn new(left: Arc<dyn Hittable>, right: Arc<dyn Hittable>, box_aabb: AABB) -> Self {
        Self {
            left,
            right,
            box_aabb,
        }
    }

    // obj: has moved its ownership !!!
    // to-do: remove "start", "end"
    pub fn new_from_vec(
        mut obj: Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = thread_rng().gen_range(0..=2);
        let span = end - start;

        let left;
        let right;

        let compare = |x: &Arc<dyn Hittable>, y: &Arc<dyn Hittable>| {
            f64::partial_cmp(
                &x.bounding_box(0., 0.).unwrap().mini[axis],
                &y.bounding_box(0., 0.).unwrap().mini[axis],
            )
            .unwrap()
        };

        if span == 0 {
            panic!("src_objects are empty!");
        } else if span == 1 {
            let obj0 = obj.pop().unwrap();
            left = obj0;
            right = left.clone();
        } else if span == 2 {
            let obj0 = obj.pop().unwrap();
            let obj1 = obj.pop().unwrap();
            match compare(&obj0, &obj1) {
                Ordering::Less => {
                    left = obj0;
                    right = obj1;
                }
                _ => {
                    left = obj1;
                    right = obj0;
                }
            }
        } else {
            obj.sort_unstable_by(compare);

            let mid = start + span / 2;
            let mut obj_left = obj;
            let obj_rigt = obj_left.split_off(span / 2);

            left = Arc::new(Self::new_from_vec(obj_left, start, mid, time0, time1));
            right = Arc::new(Self::new_from_vec(obj_rigt, mid, end, time0, time1));
        }
        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();

        let box_cur = surrounding_box(box_left, box_right);

        Self::new(left, right, box_cur)
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.box_aabb)
    }
    fn hit(&self, r: &crate::Hit::Ray, t_min: f64, t_max: f64) -> Option<crate::Hit::HitRecord> {
        if !self.box_aabb.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self.right.hit(
            r,
            t_min,
            match &hit_left {
                None => t_max,
                Some(rec) => rec.t,
            },
        );

        match hit_left {
            None => hit_right,
            Some(_) => hit_left,
        }
    }
}
