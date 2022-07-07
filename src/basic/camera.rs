use super::{
    VEC3::{Point3, Vec3},
    RAY::Ray,
};

#[derive(Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16. / 9.;
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;

        let origin = Point3::default();
        let horizontal = Vec3{x: viewport_width, y: 0., z: 0.};
        let vertical = Vec3{x: 0., y: viewport_height, z: 0.};
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner, 
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray { 
            orig: self.origin, 
            dir: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
        }
    }
}