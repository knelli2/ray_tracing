use crate::{color::Color, point::Point, textures::texture::Texture};

pub struct Solid {
    albedo: Color,
}

impl Solid {
    pub fn new(albedo: Color) -> Self {
        Self { albedo: albedo }
    }

    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(Color::new(r, g, b))
    }
}

impl Texture for Solid {
    fn color(&self, u: f32, v: f32, p: &Point) -> Color {
        self.albedo
    }
}
