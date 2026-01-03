use crate::vec3::Vec3;

pub trait CameraPath: Send + Sync {
  fn get_point(&self, t: f32) -> Vec3;
}