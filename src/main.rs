// use std::io::Write;
mod VEC3;

use VEC3::vec3;

use vec3 as color;
use vec3 as point;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn write_color(col: color) {
    println!("{} {} {}", 
        col.x * 255.999, 
        col.y * 255.999,
        col.z * 255.999
    );
}
fn main() {
    //let mut file = std::fs::File::create("2_1.txt").expect("Fail!");

    println!("P3");
    print!("{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let pixel :color = color {
                x: i as f64 / (IMAGE_WIDTH - 1) as f64,
                y: j as f64 / (IMAGE_HEIGHT - 1) as f64,
                z: 0.25,
            };
            write_color(pixel);
            // file.write_all(b"{ir} {ig} {ib}\n").unwrap();
        }
    }
}
