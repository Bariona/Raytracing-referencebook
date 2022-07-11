#![allow(clippy::redundant_field_names)]
use std::sync::Arc;

use crate::basic;
pub use crate::{
    basic::{
        random_double,
        RAY::Ray,
        VEC3::{Color, Point3, Vec3},
    },
    material::{dielectric::Dielectric, lambertian::Lambertian, matel::Metal, Material},
    obj::sphere::Sphere,
};

pub struct HitRecord {
    pub p: Point3,        // 碰撞点
    pub normal: Vec3,     // 碰撞点的单 位 法 向 量(与Ray的方向相反)
    pub t: f64,           // 表示 p = Ray(t)
    pub front_face: bool, // 是否Ray来自外侧
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

// impl Material for HitRecord {

// }

// ---- Hittable trait ----
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

// ---- Hittable List ----
// 用于存储 Hittable 的 struct

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_rec = None;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if let Some(tmp_rec) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = tmp_rec.t;
                hit_rec = Some(tmp_rec);
            }
        }

        hit_rec
    }
    pub fn random_scene() -> Self {
        let mut world = HittableList::default();
        let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        world.objects.push(Arc::new(Sphere {
            center: Point3::new(0., -1000., 0.),
            radius: 1000.,
            mat: ground_material,
        }));
        for a in -11..11 {
            for b in -11..11 {
                let mat = random_double();
                let center = Point3::new(
                    a as f64 + 0.9 * random_double(),
                    0.2,
                    b as f64 + 0.9 * random_double(),
                );

                if (center - Vec3::new(4., 0.2, 0.)).len() > 0.9 {
                    if mat < 0.8 {
                        // disffuse
                        let albedo = Color::random();
                        let sph_mat = Arc::new(Lambertian::new(albedo));
                        world.objects.push(Arc::new(Sphere {
                            center: center,
                            radius: 0.2,
                            mat: sph_mat,
                        }));
                    } else if mat < 0.95 {
                        // metal
                        let albedo = Color::random_range(0.5, 1.);
                        let fuzz = basic::random_range(0., 0.5);
                        let sph_mat = Arc::new(Metal::new(albedo, fuzz));
                        world.objects.push(Arc::new(Sphere {
                            center: center,
                            radius: 0.2,
                            mat: sph_mat,
                        }));
                    } else {
                        // glass
                        let sph_mat = Arc::new(Dielectric::new(1.5));
                        world.objects.push(Arc::new(Sphere {
                            center: center,
                            radius: 0.2,
                            mat: sph_mat,
                        }));
                    }
                }
            }
        }
        let sph_mat1 = Arc::new(Dielectric::new(1.5));
        world.objects.push(Arc::new(Sphere {
            center: Point3::new(0., 1., 0.),
            radius: 1.,
            mat: sph_mat1,
        }));
        let sph_mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
        world.objects.push(Arc::new(Sphere {
            center: Point3::new(-4., 1., 0.),
            radius: 1.,
            mat: sph_mat2,
        }));
        let sph_mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
        world.objects.push(Arc::new(Sphere {
            center: Point3::new(4., 1., 0.),
            radius: 1.,
            mat: sph_mat3,
        }));
        world
    }
}
