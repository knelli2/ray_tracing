use std::{
    f32,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    time::Instant,
};

use derivative::Derivative;
use log::{debug, info, trace};
use num_traits::Zero;

use crate::{color::Color, utils::random_float_range};
use crate::{
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    point::Point,
    vec3::Vec3,
};
use crate::{
    ray::Ray,
    utils::{Degrees, degrees_to_radians},
};

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct Camera {
    // Public resolution params
    pub aspect_ratio: f32,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: i32,

    // Public camera params
    pub center: Point,
    pub look_at: Vec3,
    pub view_up: Vec3,
    pub vertical_fov: Degrees,

    // Public focus params
    pub defocus_angle: Degrees,
    pub focus_distance: f32,

    // Public output params
    pub filename: String,
    pub output_dir: String,

    // Private resolution params
    image_height: usize,
    viewport_height: f32,
    viewport_width: f32,
    viewport_u: Vec3,
    viewport_v: Vec3,
    viewport_upper_left: Vec3,
    one_over_samples_per_pixel: f32,
    pixel_00_center: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    // Private focus params
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

    // Private camera params
    camera_basis_x: Vec3,
    camera_basis_y: Vec3,
    camera_basis_z: Vec3,

    // Private output params
    file_path: PathBuf,
    #[derivative(Debug = "ignore")]
    out_buffer: Option<BufWriter<File>>, // Option so it can be default constructed

    // Private extra params
    initialized: bool,
}

impl Camera {
    fn initialize(&mut self) {
        // Checks
        assert!(self.aspect_ratio > 0.);
        assert!(self.image_width > 0);
        assert!(!self.view_up.is_zero());
        assert!(self.center != self.look_at);
        assert!(self.vertical_fov.value > 0.);
        assert!(self.samples_per_pixel > 0);
        assert!(self.max_depth > 0);
        assert!(!self.filename.is_empty());
        assert!(!self.output_dir.is_empty());

        // Input debug
        debug!("Aspect ratio={}", self.aspect_ratio);
        debug!("Center={:?}", self.center);
        debug!("Look at={:?}", self.look_at);
        debug!("View up={:?}", self.view_up);
        debug!("Vertical FoV={}", self.vertical_fov.value);
        debug!("Samples per pixel={}", self.samples_per_pixel);
        debug!("Max depth={}", self.max_depth);
        debug!("Defocus angle={}", self.defocus_angle.value);
        debug!("Focus distance={}", self.focus_distance);
        debug!("Output dir={}", self.output_dir);
        debug!("Filename={}", self.filename);

        // Camera basis
        self.camera_basis_z = (self.center - self.look_at).unit();
        self.camera_basis_x = self.view_up.cross(&self.camera_basis_z).unit();
        self.camera_basis_y = self.camera_basis_z.cross(&self.camera_basis_x);

        // Defocus vectors
        let defocus_radius =
            self.focus_distance * (0.5 * degrees_to_radians(&self.defocus_angle).value).tan();
        self.defocus_disk_u = self.camera_basis_x * defocus_radius;
        self.defocus_disk_v = self.camera_basis_y * defocus_radius;

        // Compute height. Ensure at least a height of 1
        self.image_height = ((self.image_width as f32) / self.aspect_ratio) as usize;
        self.image_height = self.image_height.max(1);

        // Viewport
        self.viewport_height =
            2. * (0.5 * degrees_to_radians(&self.vertical_fov).value).tan() * self.focus_distance;
        self.viewport_width =
            self.viewport_height * (self.image_width as f32) / (self.image_height as f32);
        self.viewport_u = self.camera_basis_x * self.viewport_width;
        self.viewport_v = -self.camera_basis_y * self.viewport_height;
        self.viewport_upper_left = self.center
            - self.camera_basis_z * self.focus_distance
            - (self.viewport_u + self.viewport_v) * 0.5;

        // Pixels in viewport
        self.one_over_samples_per_pixel = 1. / (self.samples_per_pixel as f32);
        self.pixel_delta_u = self.viewport_u / (self.image_width as f32);
        self.pixel_delta_v = self.viewport_v / (self.image_height as f32);
        self.pixel_00_center =
            self.viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        // Debug for image, viewport, pixels, camera
        debug!("Camera basis x={:?}", self.camera_basis_x);
        debug!("Camera basis y={:?}", self.camera_basis_y);
        debug!("Camera basis z={:?}", self.camera_basis_z);
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

        self.initialized = true;
    }

    fn sample_square(&self) -> Vec3 {
        if self.samples_per_pixel > 1 {
            Vec3::new(
                random_float_range(-0.5, 0.5),
                random_float_range(-0.5, 0.5),
                0.,
            )
        } else {
            Vec3::zero()
        }
    }

    fn ray_color(world: &HittableList, ray: &Ray, depth: i32) -> Color {
        if depth <= 0 {
            return Color::black();
        }

        let record = world.hit(ray, Interval::new(1.0e-3, f32::INFINITY));
        if record.hit {
            let material_record = record.material.borrow().scatter(ray, &record);
            if material_record.scattered {
                return material_record.attenuation
                    * Self::ray_color(world, &material_record.scattered_ray, depth - 1);
            }

            return Color::black();
        }

        let normalized_direction = ray.direction().unit();
        let a = 0.5 * (normalized_direction.y + 1.);
        return Color::white() * (1. - a) + Color::blue() * a;
        // Color::white() * (1. - a) + Color::new(0.4, 0.5, 1.0) * a
    }

    fn defocus_disk_sample(&self) -> Point {
        let random_p = Point::random_in_unit_disk();

        self.center + (self.defocus_disk_u * random_p.x) + (self.defocus_disk_v * random_p.y)
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel_00_center
            + self.pixel_delta_u * ((i as f32) + offset.x)
            + self.pixel_delta_v * ((j as f32) + offset.y);

        let ray_origin = if self.defocus_angle.value <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        Ray::new(ray_origin, pixel_sample - ray_origin)
    }

    pub fn render(&mut self, world: &HittableList) {
        if !self.initialized {
            self.initialize();
        }

        debug!("Starting to write image {}", self.file_path.display());
        let now = Instant::now();

        for j in 0..self.image_height {
            trace!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut unaveraged_color = Color::black();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    unaveraged_color += Self::ray_color(world, &ray, self.max_depth);
                }

                let pixel_color = unaveraged_color * self.one_over_samples_per_pixel;
                pixel_color
                    .write_color(self.out_buffer.as_mut().unwrap())
                    .expect("Could not write pixel color");
            }
        }

        let elapsed = now.elapsed();

        debug!("Done writing image {}", self.file_path.display());
        info!(
            "Writing image {} took {} seconds.",
            self.file_path.display(),
            elapsed.as_secs_f32()
        );
    }
}
