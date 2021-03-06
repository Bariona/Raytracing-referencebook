use core::panic;
use std::{cmp::Ordering, sync::Arc};

use rand::{thread_rng, Rng};

use crate::Hit::{Hittable, HittableList};

use super::aabb::{surrounding_box, AABB};

pub struct BvhNode {
    left: Arc<dyn Hittable>, // 指向 Hittable List
    right: Arc<dyn Hittable>,
    box_aabb: AABB,
}

impl BvhNode {
    pub fn new(list: HittableList, time0: f64, time1: f64) -> Self {
        // println!("length = {}", list.objects.len());
        Self::new_from_vec(list.objects, time0, time1)
    }
    pub fn new_node_macro(
        left: Arc<dyn Hittable>,
        right: Arc<dyn Hittable>,
        time0: f64,
        time1: f64,
    ) -> Self {
        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();

        let box_aabb = surrounding_box(box_left, box_right);

        Self {
            left,
            right,
            box_aabb,
        }
    }
    pub fn new_node(left: Arc<dyn Hittable>, right: Arc<dyn Hittable>, box_aabb: AABB) -> Self {
        Self {
            left,
            right,
            box_aabb,
        }
    }

    // obj: has moved its ownership !!!
    // to-do: remove "start", "end"
    pub fn new_from_vec(mut obj: Vec<Arc<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        let axis = thread_rng().gen_range(0..=2);
        let span = obj.len();

        let left;
        let right;

        let compare = |x: &Arc<dyn Hittable>, y: &Arc<dyn Hittable>| {
            f64::partial_cmp(
                &x.bounding_box(0., 0.).unwrap().mini[axis],
                &y.bounding_box(0., 0.).unwrap().mini[axis],
            )
            .unwrap()
        };

        // println!("{}", span);
        // for item in obj {
        //     print!("{} ", item);
        // }
        // println!("");

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

            let mut obj_left = obj;
            let obj_rigt = obj_left.split_off(span / 2);

            // let mut flag = 0;
            // if span > 390 {
            //     println!("@");
            //     flag = 1;
            // }
            // println!("{}", flag);
            left = Arc::new(BvhNode::new_from_vec(obj_left, time0, time1));
            right = Arc::new(BvhNode::new_from_vec(obj_rigt, time0, time1));
            // if flag == 1 || span > 390 {
            //     println!("%");
            // }
        }
        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();

        let box_cur = surrounding_box(box_left, box_right);

        // if span > 390 {
        //     println!("{} {:?}", span, box_cur);
        //     exit(0);
        // }

        Self::new_node(left, right, box_cur)
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

        // 注意这里应该能return就先return hit_right 而不是hit_left
        match hit_right {
            Some(_) => hit_right,
            None => hit_left,
        }
    }
}
