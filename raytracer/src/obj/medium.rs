#![allow(clippy::question_mark)]
use std::f64::INFINITY;

use crate::{
    material::isotropic::Isotropic,
    texture::{solid_color::SolidColor, Texture},
    Hit::{random_double, Color, HitRecord, Hittable, Material, Vec3},
};

pub struct ConstantMedium<H: Hittable, M: Material> {
    boundary: H,
    phase: M,
    neg_inv_density: f64,
}

impl<H: Hittable, T: Texture> ConstantMedium<H, Isotropic<T>> {
    pub fn new(
        boundary: H,
        density: f64,
        color: Color,
    ) -> ConstantMedium<H, Isotropic<SolidColor>> {
        ConstantMedium {
            boundary,
            phase: Isotropic::<SolidColor>::new_color(color),
            neg_inv_density: -1. / density,
        }
    }
    pub fn new_tx(boundary: H, density: f64, texture: T) -> Self {
        ConstantMedium {
            boundary,
            phase: Isotropic::new(texture),
            neg_inv_density: -1. / density,
        }
    }
}

impl<H: Hittable, M: Material> Hittable for ConstantMedium<H, M> {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::bvh::aabb::AABB> {
        self.boundary.bounding_box(time0, time1)
    }

    fn hit(&self, r: &crate::Hit::Ray, t_min: f64, t_max: f64) -> Option<crate::Hit::HitRecord> {
        let enableDebug = false;
        let debugging = enableDebug && random_double() < 0.00001;

        let rec1 = self.boundary.hit(r, -INFINITY, INFINITY);
        if rec1.is_none() {
            return None;
        }
        let mut rec1 = rec1.unwrap();

        let rec2 = self.boundary.hit(r, rec1.t + 0.0001, INFINITY);
        if rec2.is_none() {
            return None;
        }
        let mut rec2 = rec2.unwrap();

        if debugging {
            println!("t_min = {} t_max = {}", rec1.t, rec2.t);
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0. {
            rec1.t = 0.;
        }

        let ray_length = r.direction().len();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * (random_double().ln());

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);

        Some(HitRecord::new(
            t,
            p,
            Vec3::new(1., 0., 0.),
            true,
            &self.phase,
            0.,
            0.,
        ))
    }
}
