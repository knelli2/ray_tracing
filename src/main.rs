use env_logger::{Builder, Env, Target};
#[allow(unused_imports)]
use log::{self, debug, info};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

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
            // Not sure why we had to make these doubles first and then turn
            // them into ints, but I guess I'll leave this from the book
            // let red = (i as f32) / ((image_width - 1) as f32);
            // let green = (j as f32) / ((image_height - 1) as f32);
            // let blue = 0.0;

            // let red = 255.999 * red;
            // let green = 255.999 * green;
            // let blue = 255.999 * blue;

            // writeln!(out_buffer, "{} {} {}", red as u32, green as u32, blue
            // as u32).expect("Unable to write");

            let red = i;
            let green = j;
            let blue = 0;

            writeln!(out_buffer, "{} {} {}", red, green, blue).expect("Unable to write");
        }
    }
}
