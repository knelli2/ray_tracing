#![allow(unused)]

use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3<f32>;

fn linear_to_gamma(linear: f32) -> f32 {
    linear.max(0.).sqrt()
}

impl Color {
    pub fn write_color(&self, out_buffer: &mut BufWriter<File>) -> Result<(), std::io::Error> {
        let intensity = Interval::new(0., 0.999);
        let r = linear_to_gamma(self.x);
        let g = linear_to_gamma(self.y);
        let b = linear_to_gamma(self.z);

        let r = (256. * intensity.clamp(r)) as u32;
        let g = (256. * intensity.clamp(g)) as u32;
        let b = (256. * intensity.clamp(b)) as u32;

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

    pub fn grey(value: f32) -> Color {
        assert!(0. <= value && value <= 1.);
        Color::new(value, value, value)
    }
}
