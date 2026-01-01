#![allow(unused, non_snake_case)]
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod point;
mod ray;
mod sphere;
mod utils;
mod vec3;
mod materials {
    pub mod dielectric;
    pub mod lambertian;
    pub mod material;
    pub mod metal;
}

use env_logger::{Builder, Env, Target};
use log::{self, LevelFilter, log_enabled};
use std::cell::RefCell;
use std::f32::consts::FRAC_PI_4;
use std::rc::Rc;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::point::Point;
use crate::sphere::Sphere;
use crate::utils::Degrees;

fn env_init() {
    let env = Env::default().default_filter_or("info");
    Builder::from_env(env).target(Target::Stdout).init();

    let current_level = log::max_level();

    match current_level {
        LevelFilter::Off => println!("Logging is turned off."),
        LevelFilter::Error => println!("Only ERROR messages are enabled."),
        LevelFilter::Warn => println!("WARN and ERROR messages are enabled."),
        LevelFilter::Info => println!("INFO, WARN, and ERROR messages are enabled."),
        LevelFilter::Debug => println!("DEBUG, INFO, WARN, and ERROR messages are enabled."),
        LevelFilter::Trace => println!("TRACE messages and all others are enabled."),
    }
}

fn camera_init() -> Camera {
    Camera::default()
}

fn make_glass_metal_diffuse_world() -> HittableList {
    // Materials
    let material_ground = Rc::new(RefCell::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))));
    let material_center = Rc::new(RefCell::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))));
    let material_left = Rc::new(RefCell::new(Dielectric::new(1.5)));
    let material_left_bubble = Rc::new(RefCell::new(Dielectric::new(1.0 / 1.5)));
    let material_right = Rc::new(RefCell::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.8)));

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point::new(0., -100.5, -1.),
        100.,
        material_ground,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point::new(0., 0., -1.2),
        0.5,
        material_center,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point::new(-1., 0., -1.0),
        0.5,
        material_left,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point::new(-1., 0., -1.0),
        0.4,
        material_left_bubble,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point::new(1., 0., -1.0),
        0.5,
        material_right,
    ))));

    world
}

fn make_camera_test_world() -> HittableList {
    let material_left = Rc::new(RefCell::new(Lambertian::new(Color::blue())));
    let material_right = Rc::new(RefCell::new(Lambertian::new(Color::red())));

    let R = FRAC_PI_4.cos();

    let mut world = HittableList::default();
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point::new(-R, 0., -1.),
        R,
        material_left,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point::new(R, 0., -1.),
        R,
        material_right,
    ))));

    world
}

fn main() {
    env_init();

    // Camera
    let mut camera = camera_init();
    camera.aspect_ratio = 16. / 9.;
    camera.image_width = if log_enabled!(log::Level::Debug) {
        40usize
    } else {
        400usize
    };
    camera.center = Point::new(-2., 2., 1.);
    camera.look_at = Point::new(0., 0., -1.);
    camera.view_up = Point::unit_y();
    camera.vertical_fov = Degrees::new(20.);
    camera.samples_per_pixel = 50;
    camera.max_depth = 25;
    camera.defocus_angle = Degrees::new(10.);
    camera.focus_distance = 3.4;
    camera.filename = "test_image.ppm".to_string();
    camera.output_dir = "/home/knelli/ray_tracing/output/".to_string();

    let world = make_glass_metal_diffuse_world();
    // let world = make_camera_test_world();

    // Render
    camera.render(&world);
}
