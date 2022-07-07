// use std::io::Write;
mod VEC3;
mod RAY;
mod Hit;
mod hit_base;

// use std::os::windows::process;
use VEC3::{Vec3, Color, Point3};
use RAY::Ray;
use hit_base::Hittable;
use crate::Hit::{HittableList};

pub const pi: f64 = 3.14159265358979323846264338327950288f64;
pub const inf: f64 = f64::INFINITY;

fn hit_shpere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(oc, r.direction());
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discrim = b * b - 4.0 * a * c;
    if discrim < 0. {
        -1.0 
    } else {
        (-b - discrim.sqrt()) / (2.0 * a)
    }
}
fn Ray_Color(r: Ray) -> Color {
    let t = hit_shpere(Point3{x: 0., y: 0., z: -1.0}, 0.5, r);
    if t > 0. {
        let n = r.at(t) - Vec3{x: 0., y: 0., z: -1.0};
        // if N.len() != 1.0 {
        //     eprint!("erro");
        //     process::exit(0);
        // }
        0.5 * Color{x: n.x() + 1.0, y: n.y() + 1.0, z: n.z() + 1.0}
    } else {
        let unit = r.direction().unit_vector();
        let t: f64 = 0.5 * (unit.y() + 1.0);
        (1.0 - t) * Color{x: 1.0, y: 1.0, z: 1.0} + t * Color{x: 0.5, y: 0.7, z: 1.0}
    }
}

fn main() {

    // Image
    const ratio: f64 = 16.0 / 9.0;    
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ratio) as u32;

    // World
    let mut world: HittableList::default();
    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ratio;
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
            let pixel_color = Ray_Color(r);
            write_Color(pixel_color);
        }
    }
}

fn write_Color(col: Color) {
    println!("{} {} {}", 
        col.x() * 255.999, 
        col.y() * 255.999,
        col.z() * 255.999
    );
}

// fn degree_to_radians(degree: f64) -> f64 {
//     degree * pi / 180.
// }