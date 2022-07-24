#![allow(clippy::too_many_arguments)]
use super::{
    degree_to_radians, random_range,
    RAY::Ray,
    VEC3::{Point3, Vec3},
};

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        lfrom: Point3,
        lat: Point3,
        vup: Vec3,         // view up vector
        vfov: f64,         // 视野范围 [0, 180']
        aspect_ratio: f64, // 比例
        aperture: f64,     // 光圈大小
        focus_dist: f64,   // object distance
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = degree_to_radians(vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        // let focal_length = 1.;

        let w = (lfrom - lat).unit_vector();
        let u = Vec3::cross(&vup, w).unit_vector();
        let v = Vec3::cross(&w, u);

        let ori = lfrom;
        let hori = focus_dist * viewport_width * u;
        let vert = focus_dist * viewport_height * v;
        let llc = ori - hori / 2. - vert / 2. - focus_dist * w;

        Camera {
            origin: ori,
            horizontal: hori,
            vertical: vert,
            lower_left_corner: llc,
            u,
            v,
            w,
            lens_radius: aperture / 2.,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y(); // defocus的偏移量

        Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
            tm: random_range(self.time0, self.time1),
        }
    }
}
