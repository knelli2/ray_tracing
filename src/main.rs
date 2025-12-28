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

use crate::color::Color;
use crate::point::Point;
use crate::ray::Ray;

fn init() {
    let env = Env::default().default_filter_or("info");
    Builder::from_env(env).target(Target::Stdout).init();
}

fn main() {
    init();

    let image_width = 256usize;
    let image_height = 256usize;

    let output_dir = "/home/knelli/ray_tracing/output/";
    let filename = "test_image.ppm";
    let file_path = Path::new(&output_dir).join(filename);
    let out_file = File::create(file_path).expect("Why can't I create the file");
    let mut out_buffer = BufWriter::new(out_file);

    writeln!(out_buffer, "P3").expect("Unable to write");
    writeln!(out_buffer, "{image_width} {image_height}").expect("Unable to write");
    writeln!(out_buffer, "255").expect("Unable to write");

    for j in 0..image_height {
        debug!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel = Color::new(
                (i as f32) / ((image_width - 1) as f32),
                (j as f32) / ((image_height - 1) as f32),
                0.,
            );
            pixel.write_color(&mut out_buffer).expect("Could not write pixel color");
        }
    }
}
