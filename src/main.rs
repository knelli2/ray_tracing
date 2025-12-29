#![allow(unused, non_snake_case)]
mod angle_utils;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod point;
mod ray;
mod sphere;
mod vec3;

use env_logger::{Builder, Env, Target};
use log::{self, LevelFilter, log_enabled};
use std::cell::RefCell;
use std::rc::Rc;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::point::Point;
use crate::sphere::Sphere;

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
    camera.center = Point::zero();
    camera.focal_length = 1.;
    camera.viewport_height = 2.;
    camera.filename = "test_image.ppm".to_string();
    camera.output_dir = "/home/knelli/ray_tracing/output/".to_string();

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point::new(0., 0., -1.),
        0.5,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point::new(0., -100.5, -1.),
        100.,
    ))));

    // Render
    camera.render(&world);
}
