#![allow(non_snake_case)]
mod Hit;
mod obj;
mod basic;

// use std::os::windows::process;
use basic::{
    random_double,
    VEC3::{Vec3, Color, Point3},
    RAY::Ray, 
    camera::Camera,
};
use Hit::{HittableList};
use obj::sphere::Sphere;

pub const PI: f64 = 3.14159265358979323846264338327950288f64;
pub const INF: f64 = f64::INFINITY;

fn ray_color(r: Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(&r, 0., INF) {
        return 0.5 * (rec.normal + Color::new(1., 1., 1.));
    }
    let unit = r.direction().unit_vector();
    let t: f64 = 0.5 * (unit.y() + 1.);
    (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
    
}

fn main() {

    // Image
    const RATIO: f64 = 16.0 / 9.0;    
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;


    // World
    let mut world = HittableList::default();
    world.objects.push(Box::new(Sphere{
        center: Point3::new(0., 0., -1.),
        radius: 0.5,
    }));
    world.objects.push(Box::new(Sphere{
        center: Point3::new(0., -100.5, -1.),
        radius: 100.,
    }));

    // Camera
    let cam = Camera::default();
    // let viewport_height = 2.0;
    // let viewport_width = viewport_height * RATIO;
    // let focal_length = 1.0;

    // let origin = Point3::new(0., 0., 0.);
    // let horizontal = Vec3::new(viewport_width, 0., 0.);
    // let vertical = Vec3::new(0., viewport_height, 0.);

    // let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0., 0., focal_length);

    // eprintln!("{} {:?}", lower_left_corner.len(), lower_left_corner.unit_vector().len());
    // std::process::exit(0);
    // Render
    println!("P3");
    print!("{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);


    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color: Color = Color::new(0., 0., 0.);
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH as f64 - 1.);
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT as f64 - 1.);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
            // write_color(pixel_color);
        }
    }
}


fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

// 在每个像素点周围(小范围)内采样sample_per_pixel次, 暴力取平均值 
fn write_color(pixel_color: Color, samples_per_pixel: usize) { 
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / (samples_per_pixel as f64);
    r *= scale;
    g *= scale;
    b *= scale;

    println!("{} {} {}", 
        (256. * clamp(r, 0., 0.999)) as usize, 
        (256. * clamp(g, 0., 0.999)) as usize, 
        (256. * clamp(b, 0., 0.999)) as usize,
    );
}

