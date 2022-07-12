use std::sync::Arc;

use crate::{
    basic::{self, random_range},
    bvh::aabb::{surrounding_box, AABB},
    obj::move_sphere::MoveSphere,
};
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

// ---- Hittable trait ----
pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}

// ---- Hittable List ----
// 用于存储 Hittable 的 struct

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let first_box = true;
        let mut tmp_box = AABB::default();

        for obj in &self.objects {
            match obj.bounding_box(time0, time1) {
                Some(tmp_AABB) => {
                    if first_box {
                        tmp_box = tmp_AABB;
                    } else {
                        tmp_box = surrounding_box(tmp_box, tmp_AABB);
                    }
                }
                None => {
                    return None;
                }
            }
        }
        Some(tmp_box)
    }
}

impl HittableList {
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
                let cen = Point3::new(
                    a as f64 + 0.9 * random_double(),
                    0.2,
                    b as f64 + 0.9 * random_double(),
                );

                if (cen - Vec3::new(4., 0.2, 0.)).len() > 0.9 {
                    if mat < 0.8 {
                        // disffuse
                        let albedo = Color::random();
                        let mat = Arc::new(Lambertian::new(albedo));
                        let center2 = cen + Vec3::new(0., random_range(0., 0.5), 0.);
                        world.objects.push(Arc::new(MoveSphere {
                            center0: cen,
                            center1: center2,
                            time0: 0.,
                            time1: 1.,
                            radius: 0.2,
                            mat,
                        }));
                    } else if mat < 0.95 {
                        // metal
                        let albedo = Color::random_range(0.5, 1.);
                        let fuzz = basic::random_range(0., 0.5);
                        let mat = Arc::new(Metal::new(albedo, fuzz));
                        world.objects.push(Arc::new(Sphere {
                            center: cen,
                            radius: 0.2,
                            mat,
                        }));
                    } else {
                        // glass
                        let mat = Arc::new(Dielectric::new(1.5));
                        world.objects.push(Arc::new(Sphere {
                            center: cen,
                            radius: 0.2,
                            mat,
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
