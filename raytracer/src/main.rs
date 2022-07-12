#![allow(non_snake_case)]
pub mod Hit;
pub mod basic;
pub mod bvh;
pub mod material;
pub mod obj;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{
    fs::File,
    process::exit,
    sync::{mpsc, Arc},
    thread,
};

use basic::{
    camera::Camera,
    random_double,
    RAY::Ray,
    VEC3::{Color, Point3, Vec3},
};
use Hit::{Hittable, HittableList};

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
    }

    let unit_direction = r.direction().unit_vector();
    let t: f64 = 0.5 * (unit_direction.y() + 1.);
    (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
}

fn main() {
    const THREAD_NUMBER: usize = 3;

    // Image
    const RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: i32 = 50;

    let quality = 100;
    let path = "output/output.jpg";

    // World
    let world = HittableList::random_scene();

    // Camera
    let lf = Point3::new(13., 2., 3.); // look_from
    let la = Point3::new(0., 0., 0.); // look_at
    let cam = Camera::new(lf, la, Vec3::new(0., 1., 0.), 20., RATIO, 0.1, 10., 0., 1.);

    // Render
    println!(
        "         Image size:                {}",
        style(IMAGE_WIDTH.to_string() + &"x".to_string() + &IMAGE_HEIGHT.to_string()).yellow()
    );
    println!(
        "         Sample number per pixel:   {}",
        style(SAMPLES_PER_PIXEL.to_string()).yellow()
    );
    println!(
        "         Reflection max depth:      {}",
        style(MAX_DEPTH.to_string()).yellow()
    );

    const SECTION_LINE_NUM: usize = IMAGE_HEIGHT / THREAD_NUMBER;

    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    let mut output_pixel_color = Vec::<Color>::new(); // store pixels
    let mut thread_pool = Vec::<_>::new(); // store closures

    let multiprogress = Arc::new(MultiProgress::new());
    multiprogress.set_move_cursor(true);

    for thread_id in 0..THREAD_NUMBER {
        let line_beg = SECTION_LINE_NUM * thread_id;
        let mut line_end = line_beg + SECTION_LINE_NUM;
        if line_end > IMAGE_HEIGHT || (thread_id == THREAD_NUMBER - 1 && line_end < IMAGE_HEIGHT) {
            line_end = IMAGE_HEIGHT;
        }

        let mp = multiprogress.clone();
        let progress_bar = mp.add(ProgressBar::new(
            ((line_end - line_beg) * IMAGE_WIDTH) as u64,
        ));
        progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

        let (tx, rx) = mpsc::channel();

        let clone_world = world.clone(); // due to multithread's ownership problem

        thread_pool.push((
            thread::spawn(move || {
                let mut progress = 0;
                progress_bar.set_position(0);

                let channel_send = tx;

                let mut section_pixel_color = Vec::<Color>::new();

                for j in line_beg..line_end {
                    for i in 0..IMAGE_WIDTH {
                        let mut pixel_color: Color = Color::new(0., 0., 0.);
                        for _s in 0..SAMPLES_PER_PIXEL {
                            let u = (i as f64 + random_double()) / (IMAGE_WIDTH as f64 - 1.);
                            let v = (j as f64 + random_double()) / (IMAGE_HEIGHT as f64 - 1.);
                            let r = cam.get_ray(u, v);
                            pixel_color += ray_color(r, &clone_world, MAX_DEPTH);
                        }
                        section_pixel_color.push(pixel_color);
                        progress += 1;
                        progress_bar.set_position(progress);
                    }
                }
                channel_send.send(section_pixel_color).unwrap();
                progress_bar.finish_with_message("Finished.");
            }),
            rx,
        ));
    }

    multiprogress.join().unwrap();

    let mut thread_progress_finish: bool = true;
    let collecting_progress_bar = ProgressBar::new(THREAD_NUMBER as u64);

    for thread_id in 0..THREAD_NUMBER {
        let thread = thread_pool.remove(0);
        match thread.0.join() {
            Ok(_) => {
                let mut received = thread.1.recv().unwrap();
                output_pixel_color.append(&mut received);
                collecting_progress_bar.inc(1);
            }
            Err(_) => {
                thread_progress_finish = false;
                println!(
                    "      ⚠️ {}{}{}",
                    style("Joining the ").red(),
                    style(thread_id.to_string()).yellow(),
                    style("th thread failed!").red(),
                );
            }
        }
    }
    if !thread_progress_finish {
        println!("{}", style("RE").bold().red());
        exit(1);
    }
    collecting_progress_bar.finish_and_clear();

    let mut pixel_id = 0;
    for j in 0..IMAGE_HEIGHT as u32 {
        for i in 0..IMAGE_WIDTH as u32 {
            let pixel_color = output_pixel_color[pixel_id];
            let pixel = img.get_pixel_mut(i, IMAGE_HEIGHT as u32 - j - 1);
            *pixel = image::Rgb(write_color(pixel_color, SAMPLES_PER_PIXEL));
            pixel_id += 1;
        }
    }
    println!(
        "Image size: {}\nJPEG quality: {}",
        style(IMAGE_WIDTH.to_string() + &"x".to_string() + &IMAGE_HEIGHT.to_string()).yellow(),
        style(quality.to_string()).yellow(),
    );

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

/*
Questions:
1. 多个tx, rx? 为什么不一个呢: 方便期间
2. Send类型可以在线程间安全传递其所有权??? 可是都已经move了呀: 但是可以通过channel之间传递
3. Sync + Send 的trait为什么是加在hittable trait后面? 是不是别的struct默认已经derive了Sync+Send: 正确的
    而且为什么一定要Sync + Send: 基本定义 (
4. Cam(line 119)变量被调用为什么没有交出所有权: 因为Cam已经实现了copy, 没有实现copy的类型会直接转移所有权
*/
