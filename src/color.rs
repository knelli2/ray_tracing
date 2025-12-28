#![allow(unused)]

use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::vec3::Vec3;

pub type Color = Vec3<f32>;

impl Color {
    pub fn write_color(&self, out_buffer: &mut BufWriter<File>) -> Result<(), std::io::Error> {
        let r = (255.99 * self.x) as u32;
        let g = (255.99 * self.y) as u32;
        let b = (255.99 * self.z) as u32;

        writeln!(out_buffer, "{r} {g} {b}")
    }

    pub fn black() -> Color {
        Color::zero()
    }

    pub fn white() -> Color {
        Color {
            x: 1.,
            y: 1.,
            z: 1.,
        }
    }

    pub fn red() -> Color {
      Color::unit_x()
    }

    pub fn green() -> Color {
      Color::unit_y()
    }

    pub fn blue() -> Color {
      Color::unit_z()
    }
}
