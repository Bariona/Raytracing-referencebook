#![allow(non_snake_case, unused_variables)]
pub mod Hit;
pub mod basic;
pub mod material;
pub mod obj;

// use std::os::windows::process;
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs::File, process::exit};

use basic::{
    camera::Camera,
    random_double,
    RAY::Ray,
    VEC3::{Color, Point3, Vec3},
};
use Hit::HittableList;

pub const PI: f64 = std::f64::consts::PI;
pub const INF: f64 = f64::INFINITY;

fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(rec) = world.hit(&r, 0.001, INF) {
        if let Some(ScatterRecord) = (rec.mat).scatter(&r, &rec) {
            return ScatterRecord.attenuation
                * ray_color(ScatterRecord.scattered, world, depth - 1);
        }
        return Color::new(0., 0., 0.);
        // let target: Point3 = rec.p + Vec3::random_in_hemishpere(&rec.normal);
        // return 0.5 * ray_color(Ray::new(rec.p, target - rec.p), world, depth - 1);
    }

    let unit_direction = r.direction().unit_vector();
    let t: f64 = 0.5 * (unit_direction.y() + 1.);
    (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
}

fn main() {
    // Image
    const RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / RATIO) as u32;
    const SAMPLES_PER_PIXEL: usize = 500;
    const MAX_DEPTH: i32 = 50;

    let quality = 100;
    let path = "output/output.jpg";

    // World
    let world = HittableList::random_scene();

    // Camera
    let lf = Point3::new(13., 2., 3.);
    let la = Point3::new(0., 0., 0.);
    let cam = Camera::new(lf, la, Vec3::new(0., 1., 0.), 20., RATIO, 0.1, 10.);

    // Render

    println!(
        "Image size: {}\nJPEG quality: {}",
        style(IMAGE_WIDTH.to_string() + &"x".to_string() + &IMAGE_HEIGHT.to_string()).yellow(),
        style(quality.to_string()).yellow(),
    );
    // Create image data
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    // Progress bar UI powered by library `indicatif`
    // Get environment variable CI, which is true for GitHub Action
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((IMAGE_HEIGHT * IMAGE_WIDTH) as u64)
    };
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color: Color = Color::new(0., 0., 0.);
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH as f64 - 1.);
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT as f64 - 1.);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }

            let pixel = img.get_pixel_mut(i, IMAGE_HEIGHT - j - 1);
            *pixel = image::Rgb(write_color(pixel_color, SAMPLES_PER_PIXEL));
            progress.inc(1);
        }
    }
    progress.finish();

    // Output image to file
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
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
fn write_color(pixel_color: Color, samples_per_pixel: usize) -> [u8; 3] {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / (samples_per_pixel as f64);
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    [
        (256. * clamp(r, 0., 0.999)) as u8,
        (256. * clamp(g, 0., 0.999)) as u8,
        (256. * clamp(b, 0., 0.999)) as u8,
    ]
}