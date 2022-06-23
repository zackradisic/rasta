use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Pod, Zeroable)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn to_vec3_f32s(self) -> Vec3<f32> {
        Vec3(
            self.0 as f32 / 255.0,
            self.1 as f32 / 255.0,
            self.2 as f32 / 255.0,
        )
    }

    pub fn from_vec3_f32s(v: Vec3<f32>) -> Option<Self> {
        Some(Self(
            (v.0.clamp(0.0, 1.0) * 255.0) as u8,
            (v.1.clamp(0.0, 1.0) * 255.0) as u8,
            (v.2.clamp(0.0, 1.0) * 255.0) as u8,
        ))
        // Some(Self(
        //     (v.0 * 255.0) as u8,
        //     (v.1 * 255.0) as u8,
        //     (v.2 * 255.0) as u8,
        // ))
    }
}

pub struct Vec3<T>(pub T, pub T, pub T);

impl<T: Debug> Debug for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Vec3")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

impl<T: Clone> Clone for Vec3<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), self.2.clone())
    }
}

impl<T: Sub<Output = T> + Copy> Sub for &Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: Add<Output = T> + Copy> Add for &Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for &Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl From<Color> for Vec3<u8> {
    fn from(c: Color) -> Self {
        Vec3(c.0, c.1, c.2)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
