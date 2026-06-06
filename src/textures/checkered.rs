use num_traits::Zero;

use crate::{
    color::Color,
    point::Point,
    textures::texture::{SharedTexture, Texture},
};

pub struct Checkered {
    texture_0: SharedTexture,
    texture_1: SharedTexture,
    inv_scale: f32,
}

impl Checkered {
    pub fn new(texture_0: SharedTexture, texture_1: SharedTexture, scale: f32) -> Self {
        if scale.is_zero() {
            panic!("Scale for Checkered texture cannot be zero!");
        }
        Self {
            texture_0,
            texture_1,
            inv_scale: 1.0 / scale,
        }
    }
}

impl Texture for Checkered {
    fn color(&self, u: f32, v: f32, p: &Point) -> Color {
        let x_floor = (self.inv_scale * p.x).floor() as i32;
        let y_floor = (self.inv_scale * p.y).floor() as i32;
        let z_floor = (self.inv_scale * p.z).floor() as i32;

        if (x_floor + y_floor + z_floor) % 2 == 0 {
            self.texture_0.color(u, v, p)
        } else {
            self.texture_1.color(u, v, p)
        }
    }
}
