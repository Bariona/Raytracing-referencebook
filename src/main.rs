// use std::io::Write;
mod VEC3;
mod RAY;

use VEC3::{vec3, color};
use RAY::ray;

use crate::VEC3::point3;

fn ray_color(r: ray) -> color {
    let unit = r.direction().unit_vector();
    let t: f64 = 0.5 * (unit.y() + 1.0);
    (1.0 - t) * color{x: 1.0, y: 1.0, z: 1.0} + t * color{x: 0.5, y: 0.7, z: 1.0}
}

fn main() {
    //let mut file = std::fs::File::create("2_1.txt").expect("Fail!");

    // Image
    const ratio: f64 = 16.0 / 9.0;    
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ratio;
    let focal_length = 1.0;

    let origin = point3 {x: 0.0, y: 0.0, z: 0.0};
    let horizontal = vec3 {x: viewport_width, y: 0.0, z: 0.0};
    let vertical = vec3 {x: 0.0, y: viewport_width, z: 0.0};

    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - vec3{x: 0.0, y: 0.0, z: focal_length};

    // Render
    println!("P3");
    print!("{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);


    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = ray{orig: origin, dir: lower_left_corner + u * horizontal + v * vertical - origin};
            let pixel_color = ray_color(r);
            write_color(pixel_color);
            // file.write_all(b"{ir} {ig} {ib}\n").unwrap();
        }
    }
}

fn write_color(col: color) {
    println!("{} {} {}", 
        col.x() * 255.999, 
        col.y() * 255.999,
        col.z() * 255.999
    );
}