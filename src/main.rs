#![allow(unused, non_snake_case)]
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod point;
mod ray;
mod aabb;
mod bvh;
mod sphere;
mod utils;
mod vec3;
mod materials {
    pub mod dielectric;
    pub mod lambertian;
    pub mod material;
    pub mod metal;
}
mod paths {
    pub mod line;
    pub mod path;
    pub mod stationary;
    pub mod circle;
}

use clap::{Parser, Subcommand};
use env_logger::{Builder, Env, Target};
use log::{self, LevelFilter, log_enabled};
use std::f32::consts::FRAC_PI_4;
use std::sync::Arc;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::paths::line::Line;
use crate::paths::stationary::Stationary;
use crate::point::Point;
use crate::sphere::Sphere;
use crate::utils::{Degrees, random_float, random_float_range};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'j', long, default_value_t = 1)]
    num_threads: usize,

    #[arg(short = 'd', long, default_value = "info")]
    debug_level: String,
}

fn env_init() -> Cli {
    let cli = Cli::parse();

    rayon::ThreadPoolBuilder::new()
        .num_threads(cli.num_threads)
        .build_global()
        .unwrap();
    println!("Rendering with {} threads", cli.num_threads);

    let env = Env::default().default_filter_or(&cli.debug_level);
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

    cli
}

fn camera_init() -> Camera {
    Camera::default()
}

fn make_glass_metal_diffuse_world() -> HittableList {
    // Materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_left_bubble = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.8));

    // World
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(
        Point::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0., 0., -1.2),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(-1., 0., -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(-1., 0., -1.0),
        0.4,
        material_left_bubble,
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(1., 0., -1.0),
        0.5,
        material_right,
    )));

    world
}

fn make_camera_test_world() -> HittableList {
    let material_left = Arc::new(Lambertian::new(Color::blue()));
    let material_right = Arc::new(Lambertian::new(Color::red()));

    let R = FRAC_PI_4.cos();

    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(
        Point::new(-R, 0., -1.),
        R,
        material_left,
    )));
    world.add(Arc::new(Sphere::new_moving(
        Point::new(R, 0., -1.),
        Point::new(R, 0., -1.5),
        R,
        material_right,
    )));

    world
}

fn outside_large_spheres(
    center: &Point,
    small_sphere_radius: f32,
    large_sphere_radius: f32,
    large_sphere_1_center: &Point,
    large_sphere_2_center: &Point,
    large_sphere_3_center: &Point,
) -> bool {
    let threshold = small_sphere_radius + large_sphere_radius;

    (*center - *large_sphere_1_center).length() > threshold
        && (*center - *large_sphere_2_center).length() > threshold
        && (*center - *large_sphere_3_center).length() > threshold
}

fn outside_other_spheres(center: &Point, small_sphere_radius: f32, world: &HittableList) -> bool {
    world.objects.iter().all(|s| {
        (*center - *s.as_any().downcast_ref::<Sphere>().unwrap().center().origin()).length()
            > (2. * small_sphere_radius)
    })
}

fn make_many_spheres_world() -> HittableList {
    let mut world = HittableList::default();

    // Ground
    world.add(Arc::new(Sphere::new(
        Point::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new(Color::grey(0.5))),
    )));

    let glass_index = 1.5;

    // 3 large spheres
    let large_sphere_radius = 1.0;
    let large_sphere_1_center = Point::unit_y();
    let large_sphere_2_center = Point::new(-4.0, 1.0, 0.0);
    let large_sphere_3_center = Point::new(4.0, 1.0, 0.0);
    world.add(Arc::new(Sphere::new(
        large_sphere_1_center,
        large_sphere_radius,
        Arc::new(Dielectric::new(glass_index)),
    )));
    world.add(Arc::new(Sphere::new(
        large_sphere_2_center,
        large_sphere_radius,
        Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    )));
    world.add(Arc::new(Sphere::new(
        large_sphere_3_center,
        large_sphere_radius,
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

    // Many small spheres
    let small_sphere_radius = 0.2;
    let sphere_bound = 11;
    for a in -sphere_bound..sphere_bound {
        for b in -sphere_bound..sphere_bound {
            let center = Point::new(
                (a as f32) + 0.9 * random_float::<f32>(),
                0.2,
                (b as f32) + 0.9 * random_float::<f32>(),
            );

            if outside_large_spheres(
                &center,
                small_sphere_radius,
                large_sphere_radius,
                &large_sphere_1_center,
                &large_sphere_2_center,
                &large_sphere_3_center,
            ) && outside_other_spheres(&center, small_sphere_radius, &world)
            {
                let material_chooser: f32 = random_float();

                if material_chooser < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    world.add(Arc::new(Sphere::new(
                        center,
                        small_sphere_radius,
                        Arc::new(Lambertian::new(albedo)),
                    )));
                } else if material_chooser < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = random_float_range(0., 0.5);
                    world.add(Arc::new(Sphere::new(
                        center,
                        small_sphere_radius,
                        Arc::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    // Glass
                    world.add(Arc::new(Sphere::new(
                        center,
                        small_sphere_radius,
                        Arc::new(Dielectric::new(glass_index)),
                    )));
                }
            }
        }
    }

    HittableList::from_hittable(Arc::new(world))
}

fn make_moving_test_camera() -> Camera {
    // Camera
    let mut camera = camera_init();
    camera.aspect_ratio = 16. / 9.;
    camera.image_width = if log_enabled!(log::Level::Debug) {
        40usize
    } else {
        400usize
    };
    camera.look_from = Box::new(Line::new(Point::new(-2., 2., 1.), Point::new(2., 2., 1.)));
    camera.look_at = Box::new(Stationary::new(Point::new(0., 0., -1.)));
    camera.view_up = Point::unit_y();
    camera.vertical_fov = Degrees::new(40.);
    camera.samples_per_pixel = 50;
    camera.max_depth = 50;
    camera.defocus_angle = Degrees::new(0.5);
    camera.focus_distance = 5.0;
    camera.num_frames = 5;
    camera.file_extension = "ppm".to_string();
    camera.filename_prefix = "test_image".to_string();
    camera.output_dir = "output/".to_string();

    camera
}

fn make_stationary_test_camera() -> Camera {
    // Camera
    let mut camera = camera_init();
    camera.aspect_ratio = 16. / 9.;
    camera.image_width = if log_enabled!(log::Level::Debug) {
        40usize
    } else {
        400usize
    };
    camera.look_from = Box::new(Stationary::new(Point::new(-2., 2., 1.)));
    camera.look_at = Box::new(Stationary::new(Point::new(0., 0., -1.)));
    camera.view_up = Point::unit_y();
    camera.vertical_fov = Degrees::new(40.);
    camera.samples_per_pixel = 50;
    camera.max_depth = 50;
    camera.defocus_angle = Degrees::new(0.5);
    camera.focus_distance = 5.0;
    camera.num_frames = 1;
    camera.file_extension = "ppm".to_string();
    camera.filename_prefix = "test_image".to_string();
    camera.output_dir = "output/".to_string();

    camera
}

fn make_many_spheres_camera() -> Camera {
    // Camera
    let mut camera = camera_init();
    camera.aspect_ratio = 16. / 9.;
    camera.image_width = 1920;
    camera.look_from = Box::new(Stationary::new(Point::new(13.0, 2.0, 3.0)));
    camera.look_at = Box::new(Stationary::new(Point::new(0., 0., 0.)));
    camera.view_up = Point::unit_y();
    camera.vertical_fov = Degrees::new(20.);
    camera.samples_per_pixel = 50;
    camera.max_depth = 50;
    camera.defocus_angle = Degrees::new(0.6);
    camera.focus_distance = 10.0;
    camera.filename_prefix = "many_spheres".to_string();
    camera.file_extension = "ppm".to_string();
    camera.output_dir = "output/".to_string();

    camera
}

fn main() {
    let cli = env_init();

    // let world = make_glass_metal_diffuse_world();
    // let world = make_camera_test_world();
    let world = make_many_spheres_world();

    // let mut camera = make_stationary_test_camera();
    let mut camera = make_many_spheres_camera();

    // Render
    camera.render(&world, cli.num_threads);
}
