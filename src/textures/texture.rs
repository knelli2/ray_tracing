use std::sync::Arc;

use crate::{color::Color, point::Point};

pub trait Texture: Send + Sync {
    fn color(&self, u: f32, v: f32, p: &Point) -> Color {
        Color::black()
    }
}

pub type SharedTexture = Arc<dyn Texture>;
