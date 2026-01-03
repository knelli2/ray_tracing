use crate::{paths::path::CameraPath, vec3::Vec3};

#[derive(Default,Debug)]
pub struct Stationary {
    point: Vec3,
}

impl Stationary {
    pub fn new(point: Vec3) -> Self {
        Self { point }
    }
}

impl CameraPath for Stationary {
    fn get_point(&self, _: f32) -> Vec3 {
        self.point
    }
}
