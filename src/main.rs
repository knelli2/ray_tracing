#![allow(unused)]
mod color;
mod point;
mod vec3;
mod ray;

use env_logger::{Builder, Env, Target};
use log::{self, debug, info};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::vec3::Vec3;
use crate::color::Color;
use crate::point::Point;
use crate::ray::Ray;

fn init() {
    let env = Env::default().default_filter_or("info");
    Builder::from_env(env).target(Target::Stdout).init();
}

fn ray_color(ray: &Ray) -> Color {
    let normalized_direction = ray.direction().unit();
    let a = 0.5 * (normalized_direction.y + 1.);
    debug!("{}", normalized_direction.y);
    // return Color::white() * (1.-a) + Color::blue() * a;
    return Color::white() * (1.-a) + Color::new(0.4,0.5,1.0) * a;
}

fn main() {
    init();

    let aspect_ratio: f32 = 16. / 9.;
    let image_width = 400usize;

    // Compute height. Ensure at least a height of 1
    let mut image_height = ((image_width as f32) / aspect_ratio) as usize;
    image_height = image_height.max(1);
    debug!("image width={image_width} image_height={image_height}");

    // Camera
    let focal_length: f32 = 1.0;
    let camera_center = Point::zero();

    // Viewport
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * (image_width as f32) / (image_height as f32);
    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);
    let viewport_upper_left = camera_center - Vec3::unit_z() * focal_length - (viewport_u + viewport_v) * 0.5;
    debug!("viewport_width={viewport_width:?} viewport_height={viewport_height:?}");
    debug!("viewport_u={viewport_u:?} viewport_v={viewport_v:?}");
    debug!("viewport_upper_left={viewport_upper_left:?}");

    // Pixels in viewport
    let pixel_delta_u = viewport_u / (image_width as f32);
    let pixel_delta_v = viewport_v / (image_height as f32);
    let pixel_00_center = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
    debug!("pixel_delta_u={pixel_delta_u:?} pixel_delta_v={pixel_delta_u:?}");
    debug!("pixel_00_center={pixel_00_center:?}");

    // Output file
    let output_dir = "/home/knelli/ray_tracing/output/";
    let filename = "test_image.ppm";
    let file_path = Path::new(&output_dir).join(filename);
    let out_file = File::create(file_path).expect("Why can't I create the file");
    let mut out_buffer = BufWriter::new(out_file);

    writeln!(out_buffer, "P3").expect("Unable to write");
    writeln!(out_buffer, "{image_width} {image_height}").expect("Unable to write");
    writeln!(out_buffer, "255").expect("Unable to write");

    // Write image
    for j in 0..image_height {
        debug!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel_00_center + (pixel_delta_u * (i as f32)) + (pixel_delta_v * (j as f32));
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(pixel_center, ray_direction);

            let pixel_color = ray_color(&ray);
            pixel_color.write_color(&mut out_buffer).expect("Could not write pixel color");
        }
    }
}
