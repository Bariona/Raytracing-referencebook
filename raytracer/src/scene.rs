#![allow(unused_imports)]
use rand::{thread_rng, Rng};
use std::sync::Arc;

use crate::{
    basic::{self, random_range},
    bvh::{
        aabb::{surrounding_box, AABB},
        bvh_node::BvhNode,
    },
    material::diffuse::DiffuseLight,
    obj::{
        cube::Cube,
        medium::ConstantMedium,
        move_sphere::MoveSphere,
        rectangle::{Rectanglexy, Rectanglexz, Rectangleyz},
        rotate::Rotatey,
        translate::Translate,
        triangle::Triangle,
    },
    texture::{
        checker::Checker, image_texture::ImageTexture, obj_texture::ObjTexture,
        perlin::NoiseTexture, solid_color::SolidColor, Texture,
    },
    Hit::{self, HittableList},
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

pub fn load_obj(world: &mut HittableList) {
    let rate = 20.; // 物体放大倍数
    
    // let filejpg = "obj_material/Char_Patrick.png";
    let fileimg = "obj_material/10483_baseball_diffuse.jpg";
    let img_ptr = Arc::new(image::open(fileimg).expect("load image failed").into_rgb8());
    
    let offset = Vec3::new(250., 150., 500.);

    let obj = tobj::load_obj(
        "obj_material/10483_baseball_v1_L3.obj",
        //"obj_material/patrick.obj",
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    );
    
    // panic!();
    assert!(obj.is_ok());

    let (models, _materials) = obj.expect("Failed to load OBJ file");

    // Materials might report a separate loading error if the MTL file wasn't found.
    // If you don't need the materials, you can generate a default here and use that instead.
    // let materials = materials.expect("Failed to load MTL file");

    for m in models.iter() {
        let mesh = &m.mesh;

        assert!(!mesh.texcoords.is_empty());

        let mut vertices: Vec<Point3> = Vec::default(); // 存储所用到的点集
        for id in 0..mesh.positions.len() / 3 {
            let x = mesh.positions[3 * id] as f64;
            let y = mesh.positions[3 * id + 1] as f64;
            let z = mesh.positions[3 * id + 2] as f64;
            vertices.push(Point3::new(x, y, z));
           
        }
        
        let mut object = HittableList::default();
        for i in 0..mesh.indices.len() / 3 {
            // [idx_x, idx_y, idx_z, ... ] 三个点为一个triangle

            let idx_x = mesh.indices[i * 3] as usize;
            let idx_y = mesh.indices[i * 3 + 1] as usize;
            let idx_z = mesh.indices[i * 3 + 2] as usize;

            let u1 = mesh.texcoords[2 * idx_x] as f64;
            let v1 = mesh.texcoords[2 * idx_x + 1] as f64;

            let u2 = mesh.texcoords[2 * idx_y] as f64;
            let v2 = mesh.texcoords[2 * idx_y + 1] as f64;

            let u3 = mesh.texcoords[2 * idx_z] as f64;
            let v3 = mesh.texcoords[2 * idx_z + 1] as f64;

            let mat = ObjTexture::new(img_ptr.clone(), u1, v1, u2, v2, u3, v3);
            //let mut col = mat1.value(0.5, 0., &Point3::default()).unwrap();

            let tri = Triangle::new(
                rate * vertices[idx_x],
                rate * vertices[idx_y],
                rate * vertices[idx_z],
                Lambertian::new_texture(mat),
            );
            object.add(Arc::new(tri));
            // println!("{}", i);
        }
        
        //std::process::exit(0);
        println!("{}", object.objects.len());
        let object = BvhNode::new_from_vec(object.objects, 0., 1.);
        let object = Rotatey::new(object, 180.);
        let object = Translate::new(object, offset);
        world.add(Arc::new(object));
    }
}

pub fn cornell_box() -> (HittableList, HittableList) {
    let mut world = HittableList::default();
    let mut lights = HittableList::default();

    let red = Lambertian::<SolidColor>::new(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::<SolidColor>::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::<SolidColor>::new(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::<SolidColor>::new(Color::new(15., 15., 15.));

    world.add(Arc::new(Rectangleyz::new(0., 555., 0., 555., 555., green)));
    world.add(Arc::new(Rectangleyz::new(0., 555., 0., 555., 0., red)));
    world.add(Arc::new(Rectanglexz::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    world.add(Arc::new(Rectanglexz::new(
        0.,
        555.,
        0.,
        555.,
        0.,
        white.clone(),
    )));
    world.add(Arc::new(Rectanglexy::new(0., 555., 0., 555., 555., white)));

    let light1 = Arc::new(Rectanglexz::new(213., 343., 227., 332., 554., light));
    world.add(light1.clone());
    lights.add(light1);

    // let aluminum = Metal::new(Color::new(0.8, 0.85, 0.88), 0.);
    // let box1 = Cube::new(
    //     Point3::new(0., 0., 0.),
    //     Point3::new(165., 330., 165.),
    //     aluminum,
    // );
    // let box1 = Rotatey::new(box1, 15.);
    // let box1 = Translate::new(box1, Vec3::new(265., 0., 295.));
    // world.add(Arc::new(box1));

    // let glass = Dielectric::new(1.5);
    // let ball1 = Arc::new(Sphere::new(Point3::new(190., 90., 190.), 90., glass));
    // world.add(ball1.clone());
    // lights.add(ball1);

    // let tri = Triangle::new(
    //     Point3::new(170., 50., 0.),
    //     Point3::new(300., 50., -50.),
    //     Point3::new(250., 205., 200.),
    //     red,
    // );
    // world.add(Arc::new(tri));

    // let box2 = Arc::new(Cube::new(
    //     Point3::new(0., 0., 0.),
    //     Point3::new(165., 165., 165.),
    //     white,
    // ));
    // let box2 = Arc::new(Rotatey::new(box2, -18.));
    // let box2 = Arc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));
    // world.add(box2);

    load_obj(&mut world);

    (world, lights)
}

/*
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

            boxes1.add(Arc::new(Cube::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    world.add(Arc::new(BvhNode::new(boxes1, 0., 1.)));

    let light = Arc::new(DiffuseLight::new(Color::new(7., 7., 7.)));
    world.add(Arc::new(Rectanglexz::new(
        123., 423., 147., 412., 554., light,
    )));

    // return world;

    let center1 = Point3::new(400., 400., 200.);
    let center2 = center1 + Vec3::new(30., 0., 0.);

    let moving_sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(MoveSphere::new(
        center1,
        center2,
        0.,
        1.,
        50.,
        moving_sphere_material,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(260., 150., 45.),
        50.,
        Arc::new(Dielectric::new(1.5)),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0., 150., 145.),
        50.,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.)),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(360., 150., 145.),
        70.,
        Arc::new(Dielectric::new(1.5)),
    ));

    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(0., 0., 0.),
        5000.,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new(
        boundary,
        0.0001,
        Color::new(1., 1., 1.),
    )));

    let texture = Arc::new(ImageTexture::new("raytracer/earthmap.jpg"));
    world.add(Arc::new(Sphere {
        center: Point3::new(400., 200., 400.),
        radius: 100.,
        mat: Arc::new(Lambertian::new_texture(texture)),
    }));

    let pertext = Arc::new(NoiseTexture::new(0.1));
    world.add(Arc::new(Sphere {
        center: Point3::new(220., 280., 300.),
        radius: 80.,
        mat: Arc::new(Lambertian::new_texture(pertext)),
    }));

    let mut box2 = HittableList::default();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        box2.add(Arc::new(Sphere::new(
            Point3::random_range(0., 165.),
            10.,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
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
    world.add(Arc::new(Rectanglexz::new(
        113., 443., 127., 432., 554., light,
    )));
    world.add(Arc::new(Rectanglexz::new(
        0.,
        555.,
        0.,
        555.,
        0.,
        white.clone(),
    )));
    world.add(Arc::new(Rectanglexz::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    world.add(Arc::new(Rectanglexy::new(
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

    world.add(Arc::new(ConstantMedium::new(
        box1,
        0.01,
        Color::new(0., 0., 0.),
    )));

    world.add(Arc::new(ConstantMedium::new(
        box2,
        0.01,
        Color::new(1., 1., 1.),
    )));
    // world.add(Arc::new(Cube::new(Point3::new(130., 0., 65.), Point3::new(295., 165., 230.), white.clone())));
    // world.add(Arc::new(Cube::new(Point3::new(265., 0., 295.), Point3::new(430., 330., 460.), white.clone())));
    world
}

pub fn simple_light() -> Self {
    let mut world = HittableList::default();
    let pertext = Arc::new(NoiseTexture::new(4.));
    world.add(Arc::new(Sphere {
        center: Point3::new(0., -1000., 0.),
        radius: 1000.,
        mat: Arc::new(Lambertian::new_texture(pertext.clone())),
    }));
    world.add(Arc::new(Sphere {
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
    world.add(Arc::new(Sphere {
        center: Point3::new(0., 0., 0.),
        radius: 2.,
        mat: Arc::new(Lambertian::new_texture(texture)),
    }));
    world
}
pub fn two_perlin_sphere() -> Self {
    let mut world = HittableList::default();

    let pertext = Arc::new(NoiseTexture::new(4.));

    world.add(Arc::new(Sphere {
        center: Point3::new(0., -1000., 0.),
        radius: 1000.,
        mat: Arc::new(Lambertian::new_texture(pertext.clone())),
    }));
    world.add(Arc::new(Sphere {
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
    world.add(Arc::new(Sphere {
        center: Point3::new(0., -10., 0.),
        radius: 10.,
        mat: Arc::new(Lambertian::new_texture(checker.clone())),
    }));
    world.add(Arc::new(Sphere {
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
    world.add(Arc::new(Sphere {
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
                    world.add(Arc::new(MoveSphere {
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
                    world.add(Arc::new(Sphere {
                        center: cen,
                        radius: 0.2,
                        mat,
                    }));
                } else {
                    // glass
                    let mat = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere {
                        center: cen,
                        radius: 0.2,
                        mat,
                    }));
                }
            }
        }
    }
    let sph_mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere {
        center: Point3::new(0., 1., 0.),
        radius: 1.,
        mat: sph_mat1,
    }));
    let sph_mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere {
        center: Point3::new(-4., 1., 0.),
        radius: 1.,
        mat: sph_mat2,
    }));
    let sph_mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    world.add(Arc::new(Sphere {
        center: Point3::new(4., 1., 0.),
        radius: 1.,
        mat: sph_mat3,
    }));
    world
}
*/
