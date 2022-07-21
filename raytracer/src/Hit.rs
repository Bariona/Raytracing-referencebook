use std::sync::Arc;

use crate::{
    basic::{self, random_range},
    bvh::{
        aabb::{surrounding_box, AABB},
        bvh_node::BvhNode,
    },
    material::{
        diffuse::DiffuseLight,
        rectangle::{Rectanglexy, Rectanglexz, Rectangleyz},
    },
    obj::{
        cube::Cube, medium::ConstantMedium, move_sphere::MoveSphere, rotate::Rotatey,
        translate::Translate,
    },
    texture::{checker::Checker, image_texture::ImageTexture, perlin::NoiseTexture},
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
    pub u: f64, // u, v 物体表面 surface的coordinates
    pub v: f64, // u, v \in [0, 1]
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

        let mut first_box = true;
        let mut tmp_box = AABB::default();

        for obj in &self.objects {
            match obj.bounding_box(time0, time1) {
                Some(tmp_AABB) => {
                    if first_box {
                        tmp_box = tmp_AABB;
                        first_box = false;
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
    pub fn final_scene() -> Self {
        let mut world = HittableList::default();

        let mut boxes1 = HittableList::default();
        let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

        let boxes_per_side = 20;

        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let w = 100.;
                let x0 = -1000. + (i as f64) * w;
                let z0 = -1000. + (j as f64) * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = random_range(1., 101.);
                let z1 = z0 + w;

                boxes1.objects.push(Arc::new(Cube::new(
                    Point3::new(x0, y0, z0),
                    Point3::new(x1, y1, z1),
                    ground.clone(),
                )));
            }
        }

        world.objects.push(Arc::new(BvhNode::new(boxes1, 0., 1.)));

        let light = Arc::new(DiffuseLight::new(Color::new(7., 7., 7.)));
        world.objects.push(Arc::new(Rectanglexz::new(
            123., 423., 147., 412., 554., light,
        )));

        // return world;

        let center1 = Point3::new(400., 400., 200.);
        let center2 = center1 + Vec3::new(30., 0., 0.);

        let moving_sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
        world.objects.push(Arc::new(MoveSphere::new(
            center1,
            center2,
            0.,
            1.,
            50.,
            moving_sphere_material,
        )));

        world.objects.push(Arc::new(Sphere::new(
            Point3::new(260., 150., 45.),
            50.,
            Arc::new(Dielectric::new(1.5)),
        )));

        world.objects.push(Arc::new(Sphere::new(
            Point3::new(0., 150., 145.),
            50.,
            Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.)),
        )));

        let boundary = Arc::new(Sphere::new(
            Point3::new(360., 150., 145.),
            70.,
            Arc::new(Dielectric::new(1.5)),
        ));

        world.objects.push(boundary.clone());
        world.objects.push(Arc::new(ConstantMedium::new(
            boundary,
            0.2,
            Color::new(0.2, 0.4, 0.9),
        )));

        let boundary = Arc::new(Sphere::new(
            Point3::new(0., 0., 0.),
            5000.,
            Arc::new(Dielectric::new(1.5)),
        ));
        world.objects.push(Arc::new(ConstantMedium::new(
            boundary,
            0.0001,
            Color::new(1., 1., 1.),
        )));

        let texture = Arc::new(ImageTexture::new("raytracer/earthmap.jpg"));
        world.objects.push(Arc::new(Sphere {
            center: Point3::new(400., 200., 400.),
            radius: 100.,
            mat: Arc::new(Lambertian::new_texture(texture)),
        }));

        let pertext = Arc::new(NoiseTexture::new(0.1));
        world.objects.push(Arc::new(Sphere {
            center: Point3::new(220., 280., 300.),
            radius: 80.,
            mat: Arc::new(Lambertian::new_texture(pertext)),
        }));

        let mut box2 = HittableList::default();
        let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
        let ns = 1000;
        for _j in 0..ns {
            box2.objects.push(Arc::new(Sphere::new(
                Point3::random_range(0., 165.),
                10.,
                white.clone(),
            )));
        }

        world.objects.push(Arc::new(Translate::new(
            Arc::new(Rotatey::new(Arc::new(BvhNode::new(box2, 0., 1.)), 15.)),
            Vec3::new(-100., 270., 395.),
        )));

        world
    }

    pub fn cornell_box_smoke() -> Self {
        let mut world = HittableList::default();
        let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
        let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
        let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
        let light = Arc::new(DiffuseLight::new(Color::new(7., 7., 7.)));

        world
            .objects
            .push(Arc::new(Rectangleyz::new(0., 555., 0., 555., 555., green)));
        world
            .objects
            .push(Arc::new(Rectangleyz::new(0., 555., 0., 555., 0., red)));
        world.objects.push(Arc::new(Rectanglexz::new(
            113., 443., 127., 432., 554., light,
        )));
        world.objects.push(Arc::new(Rectanglexz::new(
            0.,
            555.,
            0.,
            555.,
            0.,
            white.clone(),
        )));
        world.objects.push(Arc::new(Rectanglexz::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
        )));
        world.objects.push(Arc::new(Rectanglexy::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
        )));

        let box1 = Arc::new(Cube::new(
            Point3::new(0., 0., 0.),
            Point3::new(165., 330., 165.),
            white.clone(),
        ));
        let box1 = Arc::new(Rotatey::new(box1, 15.));
        let box1 = Arc::new(Translate::new(box1, Vec3::new(265., 0., 295.)));

        let box2 = Arc::new(Cube::new(
            Point3::new(0., 0., 0.),
            Point3::new(165., 165., 165.),
            white,
        ));
        let box2 = Arc::new(Rotatey::new(box2, -18.));
        let box2 = Arc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));

        world.objects.push(Arc::new(ConstantMedium::new(
            box1,
            0.01,
            Color::new(0., 0., 0.),
        )));

        world.objects.push(Arc::new(ConstantMedium::new(
            box2,
            0.01,
            Color::new(1., 1., 1.),
        )));
        // world.objects.push(Arc::new(Cube::new(Point3::new(130., 0., 65.), Point3::new(295., 165., 230.), white.clone())));
        // world.objects.push(Arc::new(Cube::new(Point3::new(265., 0., 295.), Point3::new(430., 330., 460.), white.clone())));
        world
    }
    pub fn cornell_box() -> Self {
        let mut world = HittableList::default();
        let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
        let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
        let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
        let light = Arc::new(DiffuseLight::new(Color::new(15., 15., 15.)));

        world
            .objects
            .push(Arc::new(Rectangleyz::new(0., 555., 0., 555., 555., green)));
        world
            .objects
            .push(Arc::new(Rectangleyz::new(0., 555., 0., 555., 0., red)));
        world.objects.push(Arc::new(Rectanglexz::new(
            213., 343., 227., 332., 554., light,
        )));
        world.objects.push(Arc::new(Rectanglexz::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
        )));
        world.objects.push(Arc::new(Rectanglexz::new(
            0.,
            555.,
            0.,
            555.,
            0.,
            white.clone(),
        )));
        world.objects.push(Arc::new(Rectanglexy::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
        )));

        let box1 = Arc::new(Cube::new(
            Point3::new(0., 0., 0.),
            Point3::new(165., 330., 165.),
            white.clone(),
        ));
        let box1 = Arc::new(Rotatey::new(box1, 15.));
        let box1 = Arc::new(Translate::new(box1, Vec3::new(265., 0., 295.)));

        let box2 = Arc::new(Cube::new(
            Point3::new(0., 0., 0.),
            Point3::new(165., 165., 165.),
            white,
        ));
        let box2 = Arc::new(Rotatey::new(box2, -18.));
        let box2 = Arc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));

        world.objects.push(box1);
        world.objects.push(box2);

        world
    }

    pub fn simple_light() -> Self {
        let mut world = HittableList::default();
        let pertext = Arc::new(NoiseTexture::new(4.));
        world.objects.push(Arc::new(Sphere {
            center: Point3::new(0., -1000., 0.),
            radius: 1000.,
            mat: Arc::new(Lambertian::new_texture(pertext.clone())),
        }));
        world.objects.push(Arc::new(Sphere {
            center: Point3::new(0., 2., 0.),
            radius: 2.,
            mat: Arc::new(Lambertian::new_texture(pertext)),
        }));

        let difflight = Arc::new(DiffuseLight::new(Color::new(4., 4., 4.)));
        world
            .objects
            .push(Arc::new(Rectanglexy::new(3., 5., 1., 3., -2., difflight)));
        world
    }
    pub fn load_image() -> Self {
        let mut world = HittableList::default();
        let texture = Arc::new(ImageTexture::new("raytracer/earthmap.jpg"));
        world.objects.push(Arc::new(Sphere {
            center: Point3::new(0., 0., 0.),
            radius: 2.,
            mat: Arc::new(Lambertian::new_texture(texture)),
        }));
        world
    }
    pub fn two_perlin_sphere() -> Self {
        let mut world = HittableList::default();

        let pertext = Arc::new(NoiseTexture::new(4.));

        world.objects.push(Arc::new(Sphere {
            center: Point3::new(0., -1000., 0.),
            radius: 1000.,
            mat: Arc::new(Lambertian::new_texture(pertext.clone())),
        }));
        world.objects.push(Arc::new(Sphere {
            center: Point3::new(0., 2., 0.),
            radius: 2.,
            mat: Arc::new(Lambertian::new_texture(pertext)),
        }));
        world
    }

    pub fn two_sphere() -> Self {
        let mut world = HittableList::default();
        let checker = Arc::new(Checker::new(
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
        ));
        world.objects.push(Arc::new(Sphere {
            center: Point3::new(0., -10., 0.),
            radius: 10.,
            mat: Arc::new(Lambertian::new_texture(checker.clone())),
        }));
        world.objects.push(Arc::new(Sphere {
            center: Point3::new(0., 10., 0.),
            radius: 10.,
            mat: Arc::new(Lambertian::new_texture(checker)),
        }));
        world
    }

    pub fn random_scene() -> Self {
        let mut world = HittableList::default();

        let checker = Arc::new(Checker::new(
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
        ));

        // let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        world.objects.push(Arc::new(Sphere {
            center: Point3::new(0., -1000., 0.),
            radius: 1000.,
            mat: Arc::new(Lambertian::new_texture(checker)),
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
