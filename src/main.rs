// use std::io::Write;
#![allow(non_snake_case)]
mod VEC3;
mod RAY;
mod Hit;
mod sphere;

// use std::os::windows::process;
use VEC3::{Vec3, Color, Point3};
use RAY::Ray;
use Hit::{HittableList};
use sphere::Sphere;

pub const PI: f64 = 3.14159265358979323846264338327950288f64;
pub const INF: f64 = f64::INFINITY;

fn ray_color(r: Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(&r, 0., INF) {
        return 0.5 * (rec.normal + Color{x: 1., y: 1., z: 1.});
    }
    let unit = r.direction().unit_vector();
    let t: f64 = 0.5 * (unit.y() + 1.);
    (1.0 - t) * Color{x: 1., y: 1., z: 1.} + t * Color{x: 0.5, y: 0.7, z: 1.}
    
}

fn main() {

    // Image
    const RATIO: f64 = 16.0 / 9.0;    
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / RATIO) as u32;

    // World
    let mut world = HittableList::default();
    world.objects.push(Box::new(Sphere{
        center: Point3{x: 0., y: 0., z: -1.},
        radius: 0.5,
    }));
    world.objects.push(Box::new(Sphere{
        center: Point3{x: 0., y: -100.5, z: -1.},
        radius: 100.,
    }));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * RATIO;
    let focal_length = 1.0;

    let origin = Point3 {x: 0., y: 0., z: 0.};
    let horizontal = Vec3 {x: viewport_width, y: 0., z: 0.};
    let vertical = Vec3 {x: 0., y: viewport_height, z: 0.};

    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3{x: 0., y: 0., z: focal_length};

    // eprintln!("{} {:?}", lower_left_corner.len(), lower_left_corner.unit_vector().len());
    // std::process::exit(0);
    // Render
    println!("P3");
    print!("{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);


    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let r = Ray{orig: origin, dir: lower_left_corner + u * horizontal + v * vertical - origin};
            let pixel_color = ray_color(r, &world);
            write_color(pixel_color);
        }
    }
}

fn write_color(col: Color) {
    println!("{} {} {}", 
        col.x() * 255.999, 
        col.y() * 255.999,
        col.z() * 255.999
    );
}

// fn degree_to_radians(degree: f64) -> f64 {
//     degree * pi / 180.
// }