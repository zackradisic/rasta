use std::fmt::Debug;

use crate::math::{Vec2, Vec3};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub const RED: Self = Color(255, 0, 0);
    pub const GREEN: Self = Color(0, 255, 0);
    pub const BLUE: Self = Color(0, 0, 255);
}

impl Color {
    pub fn to_vec3_f32s(self) -> Vec3<f32> {
        Vec3(
            self.0 as f32 / 255.0,
            self.1 as f32 / 255.0,
            self.2 as f32 / 255.0,
        )
    }

    pub fn from_vec3_f32s(v: Vec3<f32>) -> Self {
        Self(
            (v.0.clamp(0.0, 1.0) * 255.0) as u8,
            (v.1.clamp(0.0, 1.0) * 255.0) as u8,
            (v.2.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl<T: Into<f32>> From<Vec2<T>> for Point {
    fn from(v: Vec2<T>) -> Self {
        Self {
            x: v.0.into(),
            y: v.1.into(),
        }
    }
}
