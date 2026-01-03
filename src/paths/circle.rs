use std::f32::consts::PI;

use crate::{paths::path::CameraPath, point::Point, vec3::Vec3};

#[derive(Default, Debug)]
pub struct Circle {
    center: Point,
    radius: f32,
    normal: Vec3,
    /// Starts aligned with +x axis
    initial_rotation: f32,

    transformed_x: Vec3,
    transformed_y: Vec3,
    start: Vec3,
}

impl Circle {
    pub fn new(center: Point, radius: f32, normal: Vec3, initial_rotation: f32) -> Self {
        // Follow Rodrigues' formula for rotating unit-z into normal vector
        // found here:
        // https://math.stackexchange.com/questions/61547/rotation-of-a-vector-distribution-to-align-with-a-normal-vector
        let factor = 1. / (1. + normal.z);
        let n_xy = normal.x * normal.y;
        let new_unit_x = Vec3::new(
            normal.z - normal.x + factor * (normal.y * normal.y - n_xy),
            0.,
            0.,
        )
        .unit();
        let new_unit_y = Vec3::new(
            normal.z - normal.y + factor * (normal.x * normal.x - n_xy),
            0.,
            0.,
        )
        .unit();

        let cos_phi = initial_rotation.cos();
        let sin_phi = initial_rotation.sin();

        let transformed_x = new_unit_x * cos_phi;
        let transformed_y = new_unit_y * sin_phi;

        Self {
            center,
            radius,
            normal,
            initial_rotation,
            transformed_x: transformed_x,
            transformed_y: transformed_y,
            start: center + transformed_x * cos_phi + transformed_y * sin_phi,
        }
    }
}

impl CameraPath for Circle {
    fn get_point(&self, t: f32) -> Vec3 {
        let t = 2.0 * PI * t;
        self.start + self.transformed_x * t.cos() + self.transformed_y * t.sin()
    }
}
