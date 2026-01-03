use crate::{paths::path::CameraPath, vec3::Vec3};

#[derive(Default,Debug)]
pub struct Line {
    start: Vec3,
    end: Vec3,
}

impl Line {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        Self { start, end }
    }
}

impl CameraPath for Line {
    fn get_point(&self, t: f32) -> Vec3 {
        self.start * (1. - t) + self.end * t
    }
}
