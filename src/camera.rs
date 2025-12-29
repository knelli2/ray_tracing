use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use derivative::Derivative;
use log::debug;
use rand::{Rng, rngs::ThreadRng};

use crate::color::Color;
use crate::ray::Ray;
use crate::{
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    point::Point,
    vec3::Vec3,
};

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct Camera {
    // Public camera params
    pub aspect_ratio: f32,
    pub image_width: usize,
    pub center: Point,
    pub focal_length: f32,
    pub viewport_height: f32,
    pub samples_per_pixel: usize,

    // Public output params
    pub filename: String,
    pub output_dir: String,

    // Private camera params
    image_height: usize,
    viewport_width: f32,
    viewport_u: Vec3,
    viewport_v: Vec3,
    viewport_upper_left: Vec3,
    one_over_samples_per_pixel: f32,
    pixel_00_center: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    // Private output params
    file_path: PathBuf,
    #[derivative(Debug = "ignore")]
    out_buffer: Option<BufWriter<File>>, // Option so it can be default constructed

    // Private extra params
    rng: ThreadRng,
    initialized: bool,
}

impl Camera {
    fn initialize(&mut self) {
        // Checks
        assert!(self.aspect_ratio > 0.);
        assert!(self.image_width > 0);
        assert!(self.focal_length > 0.);
        assert!(self.viewport_height > 0.);
        assert!(self.samples_per_pixel > 0);
        assert!(!self.filename.is_empty());
        assert!(!self.output_dir.is_empty());

        // Input debug
        debug!("Aspect ratio={}", self.aspect_ratio);
        debug!("Center={:?}", self.center);
        debug!("Focal length={}", self.focal_length);
        debug!("Output dir={}", self.output_dir);
        debug!("Samples per pixel={}", self.samples_per_pixel);
        debug!("Filename={}", self.filename);

        // Compute height. Ensure at least a height of 1
        self.image_height = ((self.image_width as f32) / self.aspect_ratio) as usize;
        self.image_height = self.image_height.max(1);

        // Viewport self.viewport_width =
        self.viewport_width =
            self.viewport_height * (self.image_width as f32) / (self.image_height as f32);
        self.viewport_u = Vec3::new(self.viewport_width, 0., 0.);
        self.viewport_v = Vec3::new(0., -self.viewport_height, 0.);
        self.viewport_upper_left = self.center
            - Vec3::new(0., 0., self.focal_length)
            - (self.viewport_u + self.viewport_v) * 0.5;

        // Pixels in viewport
        self.one_over_samples_per_pixel = 1. / (self.samples_per_pixel as f32);
        self.pixel_delta_u = self.viewport_u / (self.image_width as f32);
        self.pixel_delta_v = self.viewport_v / (self.image_height as f32);
        self.pixel_00_center =
            self.viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        // Debug for image, viewport, pixels
        debug!(
            "image width={} image_height={}",
            self.image_width, self.image_height
        );
        debug!(
            "viewport_width={:?} viewport_height={:?}",
            self.viewport_width, self.viewport_height
        );
        debug!(
            "viewport_u={:?} viewport_v={:?}",
            self.viewport_u, self.viewport_v
        );
        debug!("viewport_upper_left={:?}", self.viewport_upper_left);
        debug!(
            "pixel_delta_u={:?} pixel_delta_v={:?}",
            self.pixel_delta_u, self.pixel_delta_v
        );
        debug!("pixel_00_center={:?}", self.pixel_00_center);

        // Output file
        self.file_path = Path::new(&self.output_dir).join(&self.filename);
        let out_file = File::create(&self.file_path).expect("Why can't I create the file");
        self.out_buffer = Some(BufWriter::new(out_file));

        writeln!(self.out_buffer.as_mut().unwrap(), "P3").expect("Unable to write");
        writeln!(
            self.out_buffer.as_mut().unwrap(),
            "{} {}",
            self.image_width,
            self.image_height
        )
        .expect("Unable to write");
        writeln!(self.out_buffer.as_mut().unwrap(), "255").expect("Unable to write");

        self.rng = rand::rng();
        self.initialized = true;
    }

    fn sample_square(&mut self) -> Vec3 {
        if self.samples_per_pixel > 1 {
          Vec3::new(
              self.rng.random_range(-0.5..0.5),
              self.rng.random_range(-0.5..0.5),
              0.,
          )
        } else {
          Vec3::zero()
        }
    }

    fn ray_color(world: &HittableList, ray: &Ray) -> Color {
        let record = world.hit(ray, Interval::half_universe_pos());
        if record.hit {
            return Color::new(
                record.normal.x + 1.,
                record.normal.y + 1.,
                record.normal.z + 1.,
            ) * 0.5;
        }

        let normalized_direction = ray.direction().unit();
        let a = 0.5 * (normalized_direction.y + 1.);
        debug!("{}", normalized_direction.y);
        return Color::white() * (1. - a) + Color::blue() * a;
        // Color::white() * (1. - a) + Color::new(0.4, 0.5, 1.0) * a
    }

    fn get_ray(&mut self, i: usize, j: usize) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel_00_center
            + self.pixel_delta_u * ((i as f32) + offset.x)
            + self.pixel_delta_v * ((j as f32) + offset.y);

        Ray::new(self.center, pixel_sample - self.center)
    }

    pub fn render(&mut self, world: &HittableList) {
        if !self.initialized {
            self.initialize();
        }

        for j in 0..self.image_height {
            debug!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut unaveraged_color = Color::black();
                for _ in 0..self.samples_per_pixel {
                  let ray = self.get_ray(i, j);
                  unaveraged_color += Self::ray_color(world, &ray);
                }

                let pixel_color = unaveraged_color * self.one_over_samples_per_pixel;
                pixel_color
                    .write_color(self.out_buffer.as_mut().unwrap())
                    .expect("Could not write pixel color");
            }
        }

        debug!("Done writing image {}", self.file_path.display());
    }
}
