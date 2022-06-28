use std::f32::consts::PI;
use std::fmt::Debug;
use std::ops::{Add, Div, Index, Mul, Sub, Deref};

use crate::rasterize::Color;

// pub trait Vector<T> {
//     fn mul(&self, rhs: Self) -> Self;
// }

pub struct Vec2<T>(pub T, pub T);

impl<T> From<(T, T)> for Vec2<T> {
    fn from((a, b): (T, T)) -> Self {
        Self(a, b)
    }
}

impl<T: Copy> Copy for Vec2<T> {}

impl<T: Debug> Debug for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Vec2").field(&self.0).field(&self.1).finish()
    }
}

impl<T: Clone> Clone for Vec2<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<T: Sub<Output = T> + Copy> Sub for &Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Add<Output = T> + Copy> Add for &Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for &Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

pub struct Vec3<T>(pub T, pub T, pub T);

impl Vec3<f32> {
    pub fn magnitude(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
    
    pub fn angle(&self, rhs: &Vec3<f32>) -> f32 {
        self.dot(rhs) / (self.magnitude() * rhs.magnitude())
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self(self.0 / mag, self.1 / mag, self.2 / mag)
    }

    pub fn to_point_vec4(&self) -> Vec4<f32> {
        Vec4(self.0, self.1, self.2, 1.0)
    }

    pub fn component_wise_mul(&self, rhs: &Vec3<f32>) -> Vec3<f32> {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vec3<T> {
    pub fn dot(&self, rhs: &Vec3<T>) -> T {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
    
}

impl<T: Mul<Output = T> + Add<Output = T>  + Sub<Output = T> + Copy> Vec3<T> {
    pub fn cross(&self, rhs: &Vec3<T>) -> Vec3<T> {
        Vec3(self.1*rhs.2 - self.2*rhs.1, self.2*rhs.0 - self.0*rhs.2, self.0*rhs.1 - self.1*rhs.0)
    }
}


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

impl<T: Sub<Output = T> + Copy> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl<T: Sub<Output = T> + Copy> Sub for &Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: Add<Output = T> + Copy> Add<&Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: &Vec3<T>) -> Self::Output {
        &self + rhs
    }
}

impl<T: Add<Output = T> + Copy> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        &self + &rhs
    }
}

impl<T: Add<Output = T> + Copy> Add<Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        self + &rhs
    }
}

impl<T: Add<Output = T> + Copy> Add for &Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}


impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
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

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from((a, b, c): (T, T, T)) -> Self {
        Self(a, b, c)
    }
}

pub struct Vec4<T>(pub T, pub T, pub T, pub T);

impl<T: Copy> Vec4<T> {
    pub fn drop_fourth_component(&self) -> Vec3<T> {
        Vec3(self.0, self.1, self.2)
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vec4<T> {
    pub fn dot(&self, rhs: &Vec4<T>) -> T {
        (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2) + (self.3 * rhs.3)
    }
    pub fn dot3(&self, rhs: &Vec3<T>) -> T {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

impl<T: Debug> Debug for Vec4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Vec4")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .field(&self.3)
            .finish()
    }
}

impl<T: Clone> Clone for Vec4<T> {
    fn clone(&self) -> Self {
        Self(
            self.0.clone(),
            self.1.clone(),
            self.2.clone(),
            self.3.clone(),
        )
    }
}

impl<T: Sub<Output = T> + Copy> Sub for &Vec4<T> {
    type Output = Vec4<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl<T: Add<Output = T> + Copy> Add for &Vec4<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for &Vec4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec4(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<Vec3<T>> for &Vec4<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec4<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

impl<T> From<(T, T, T, T)> for Vec4<T> {
    fn from((a, b, c, d): (T, T, T, T)) -> Self {
        Self(a, b, c, d)
    }
}

pub struct Degrees(pub f32);
impl Degrees {
    pub fn into_radians(self) -> Radians {
        Radians(self.0 * (PI / 180.0))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Radians(f32);

impl Radians {
    pub fn new(val: f32) -> Self {
        Self(val)
    }
}

impl From<Degrees> for Radians {
    fn from(d: Degrees) -> Self {
        d.into_radians()
    }
}

impl Add for Radians {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

/// 3x3 Matrix in column-major order
pub struct Mat3<T>([[T; 3]; 3]);

impl<T> Mat3<T> {
    pub fn new(m00: T, m01: T, m02: T, m10: T, m11: T, m12: T, m20: T, m21: T, m22: T) -> Self {
        Self([[m00, m10, m20], [m01, m11, m21], [m02, m12, m22]])
    }

    // pub fn rotate_y<A: Into<Radians>>(angle: A, origin:)
}

impl<T: Copy> Mat3<T> {
    pub fn row(&self, i: u8) -> Vec3<T> {
        Vec3(self[(i, 0)], self[(i, 1)], self[(i, 2)])
    }

    pub fn col(&self, i: u8) -> Vec3<T> {
        Vec3(self[(0, i)], self[(1, i)], self[(2, i)])
    }
}

impl<T> Index<(u8, u8)> for Mat3<T> {
    type Output = T;

    fn index(&self, (row, col): (u8, u8)) -> &Self::Output {
        &self.0[col as usize][row as usize]
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Mul<&Vec3<T>> for Mat3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: &Vec3<T>) -> Self::Output {
        Vec3(
            self.row(0).dot(rhs),
            self.row(1).dot(rhs),
            self.row(2).dot(rhs),
        )
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Mul<&Mat3<T>> for Mat3<T> {
    type Output = Mat3<T>;

    fn mul(self, rhs: &Mat3<T>) -> Self::Output {
        Mat3::new(
            // row 0
            self.row(0).dot(&rhs.col(0)),
            self.row(0).dot(&rhs.col(1)),
            self.row(0).dot(&rhs.col(2)),
            // row 1
            self.row(1).dot(&rhs.col(0)),
            self.row(1).dot(&rhs.col(1)),
            self.row(1).dot(&rhs.col(2)),
            // row 2
            self.row(2).dot(&rhs.col(0)),
            self.row(2).dot(&rhs.col(1)),
            self.row(2).dot(&rhs.col(2)),
        )
    }
}

impl<T: Clone> Clone for Mat3<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Debug> Debug for Mat3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Mat3").field(&self.0).finish()
    }
}

pub struct Mat4<T>([[T; 4]; 4]);

impl<T> Mat4<T> {
    // m00 m01 m02 m03
    // m10 m11 m12 m13
    // m20 m21 m22 m12
    // m30 m31 m32 m33
    pub fn new(
        m00: T,
        m01: T,
        m02: T,
        m03: T,
        m10: T,
        m11: T,
        m12: T,
        m13: T,
        m20: T,
        m21: T,
        m22: T,
        m23: T,
        m30: T,
        m31: T,
        m32: T,
        m33: T,
    ) -> Self {
        Self([
            [m00, m10, m20, m30],
            [m01, m11, m21, m31],
            [m02, m12, m22, m32],
            [m03, m13, m23, m33],
        ])
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy> Mat4<T> {
    pub fn det(&self) -> T {
        let b00 = self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)];
        let b01 = self[(0, 0)] * self[(1, 2)] - self[(0, 2)] * self[(1, 0)];
        let b02 = self[(0, 0)] * self[(1, 3)] - self[(0, 3)] * self[(1, 0)];
        let b03 = self[(0, 1)] * self[(1, 2)] - self[(0, 2)] * self[(1, 1)];
        let b04 = self[(0, 1)] * self[(1, 3)] - self[(0, 3)] * self[(1, 1)];
        let b05 = self[(0, 2)] * self[(1, 3)] - self[(0, 3)] * self[(1, 2)];
        let b06 = self[(2, 0)] * self[(3, 1)] - self[(2, 1)] * self[(3, 0)];
        let b07 = self[(2, 0)] * self[(3, 2)] - self[(2, 2)] * self[(3, 0)];
        let b08 = self[(2, 0)] * self[(3, 3)] - self[(2, 3)] * self[(3, 0)];
        let b09 = self[(2, 1)] * self[(3, 2)] - self[(2, 2)] * self[(3, 1)];
        let b10 = self[(2, 1)] * self[(3, 3)] - self[(2, 3)] * self[(3, 1)];
        let b11 = self[(2, 2)] * self[(3, 3)] - self[(2, 3)] * self[(3, 2)];
        let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
        det
    }
}

impl Mat4<f32> {
    #[rustfmt::skip]
    pub fn invert(&self) -> Option<Mat4<f32>> {
        let det = self.det();
        if det == 0.0 {
            return None;
        }

        let a00 = self[(0, 0)];
        let a01 = self[(0, 1)];
        let a02 = self[(0, 2)];
        let a03 = self[(0, 3)];
        let a10 = self[(1, 0)];
        let a11 = self[(1, 1)];
        let a12 = self[(1, 2)];
        let a13 = self[(1, 3)];
        let a20 = self[(2, 0)];
        let a21 = self[(2, 1)];
        let a22 = self[(2, 2)];
        let a23 = self[(2, 3)];
        let a30 = self[(3, 0)];
        let a31 = self[(3, 1)];
        let a32 = self[(3, 2)];
        let a33 = self[(3, 3)];
        let b00 = self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)];
        let b01 = self[(0, 0)] * self[(1, 2)] - self[(0, 2)] * self[(1, 0)];
        let b02 = self[(0, 0)] * self[(1, 3)] - self[(0, 3)] * self[(1, 0)];
        let b03 = self[(0, 1)] * self[(1, 2)] - self[(0, 2)] * self[(1, 1)];
        let b04 = self[(0, 1)] * self[(1, 3)] - self[(0, 3)] * self[(1, 1)];
        let b05 = self[(0, 2)] * self[(1, 3)] - self[(0, 3)] * self[(1, 2)];
        let b06 = self[(2, 0)] * self[(3, 1)] - self[(2, 1)] * self[(3, 0)];
        let b07 = self[(2, 0)] * self[(3, 2)] - self[(2, 2)] * self[(3, 0)];
        let b08 = self[(2, 0)] * self[(3, 3)] - self[(2, 3)] * self[(3, 0)];
        let b09 = self[(2, 1)] * self[(3, 2)] - self[(2, 2)] * self[(3, 1)];
        let b10 = self[(2, 1)] * self[(3, 3)] - self[(2, 3)] * self[(3, 1)];
        let b11 = self[(2, 2)] * self[(3, 3)] - self[(2, 3)] * self[(3, 2)];

        Some(Mat4::new(
            // row 0
            a11 * b11 - a12 * b10 + a13 * b09,
            a02 * b10 - a01 * b11 - a03 * b09,
            a31 * b05 - a32 * b04 + a33 * b03,
            a22 * b04 - a21 * b05 - a23 * b03,
            // row 1
            a12 * b08 - a10 * b11 - a13 * b07,
            a00 * b11 - a02 * b08 + a03 * b07,
            a32 * b02 - a30 * b05 - a33 * b01,
            a20 * b05 - a22 * b02 + a23 * b01,
            // row 2
            a10 * b10 - a11 * b08 + a13 * b06,
            a01 * b08 - a00 * b10 - a03 * b06,
            a30 * b04 - a31 * b02 + a33 * b00,
            a21 * b02 - a20 * b04 - a23 * b00,
            // row 3
            a11 * b07 - a10 * b09 - a12 * b06,
            a00 * b09 - a01 * b07 + a02 * b06,
            a31 * b01 - a30 * b03 - a32 * b00,
            a20 * b03 - a21 * b01 + a22 * b00
        ) * (1.0/det))
    }
}

impl Mat4<f32> {
    #[rustfmt::skip]
    pub fn scale(scale: Vec3<f32>) -> Self {
        Mat4::new(
            scale.0,     0.0,     0.0, 0.0,
                0.0, scale.1,     0.0, 0.0,
                0.0,     0.0, scale.2, 0.0,
                0.0,     0.0,     0.0, 1.0,
        )
    }

    #[rustfmt::skip]
    pub fn translate(translation: Vec3<f32>) -> Self {
        Mat4::new(
            1.0, 0.0, 0.0, translation.0,
            0.0, 1.0, 0.0, translation.1,
            0.0, 0.0, 1.0, translation.2,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    #[rustfmt::skip]
    pub fn rotate_y_axis<R: Into<Radians>>(radians: R, point: Vec3<f32>) -> Self {
        let r: Radians = radians.into();
        let r: f32 = r.0;
        let translate_to_origin = Mat4::translate(&point * -1.0);
        let translate_back = Mat4::translate(point);

        let rotation_matrix = Mat4::new(
             r.cos(), 0.0, -r.sin(),  0.0,
                 0.0, 1.0,      0.0,  0.0,
             r.sin(), 0.0,  r.cos(),  0.0,
                 0.0, 0.0,      0.0,  1.0,
        );

        translate_back * rotation_matrix * translate_to_origin
    }

    #[rustfmt::skip]
    pub fn rotate_z_axis<R: Into<Radians>>(radians: R, point: Vec3<f32>) -> Self {
        let r: Radians = radians.into();
        let r: f32 = r.0;
        let translate_to_origin = Mat4::translate(&point * -1.0);
        let translate_back = Mat4::translate(point);

        let rotation_matrix = Mat4::new(
             r.cos(), -r.sin(),  0.0,  0.0,
             r.sin(),  r.cos(),  0.0,  0.0,
                 0.0,      0.0,  1.0,  0.0,
                 0.0,      0.0,  0.0,  1.0,
        );

        translate_back * rotation_matrix * translate_to_origin
    }

    // #[rustfmt::skip]
    pub fn perspective(right: f32, left: f32, bot: f32, top: f32, near: f32, far: f32) -> Self {
        let m00 = (2.0 * near)/(right - left);
        let m02 = -(right + left)/(right - left);
        let m11 = (2.0 * near) / (bot - top);
        let m12 = -(bot+top)/(bot - top);
        let m22 = far/(far - near);
        let m23 = (-1.0 * far * near)/(far - near);
        Mat4::new(
            m00, 0.0, m02, 0.0, 
            0.0, m11, m12, 0.0, 
            0.0, 0.0, m22, m23, 
            0.0, 0.0, 1.0, 0.0
        )
    }

    // #[rustfmt::skip]
    // pub fn projection(d: f32) -> Self {
    //     Mat4::new(
    //         d, 0.0, 0.0, 0.0, 
    //         0.0, d, 0.0, 0.0, 
    //         0.0, 0.0, 1.0, 0.0, 
    //         0.0, 0.0, 0.0, 1.0
    //     )
    // }


    #[rustfmt::skip]
    pub fn viewport_to_canvas(cw: f32, ch: f32, vw: f32, vh: f32) -> Self {
        Mat4::new(
            cw/vw, 0.0, 0.0, 0.0, 
            0.0, ch/vh, 0.0, 0.0, 
            0.0, 0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0, 1.0
        )
    }

    #[rustfmt::skip]
    pub fn identity() -> Self {
        Mat4::new(
            1.0, 0.0, 0.0, 0.0, 
            0.0, 1.0, 0.0, 0.0, 
            0.0, 0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0, 1.0
        )
    }
}

impl<T: Copy> Mat4<T> {
    pub fn row(&self, i: u8) -> Vec4<T> {
        Vec4(self[(i, 0)], self[(i, 1)], self[(i, 2)], self[(i, 3)])
    }

    pub fn col(&self, i: u8) -> Vec4<T> {
        // Vec4(self[(i, 0)], self[(i, 1)], self[(i, 2)], self[(i, 3)])
        Vec4(self[(0, i)], self[(1, i)], self[(2, i)], self[(3, i)])
    }
}

impl<T> Index<(u8, u8)> for Mat4<T> {
    type Output = T;

    fn index(&self, (row, col): (u8, u8)) -> &Self::Output {
        &self.0[col as usize][row as usize]
    }
}

impl<T: Debug + Mul<Output = T> + Add<Output = T> + Copy> Mul<Vec4<T>> for Mat4<T>{
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Debug + Mul<Output = T> + Add<Output = T> + Copy> Mul<Vec4<T>> for &Mat4<T>{
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T: Debug + Mul<Output = T> + Add<Output = T> + Copy> Mul<&Vec4<T>> for &Mat4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: &Vec4<T>) -> Self::Output {
        Vec4(
            self.row(0).dot(rhs),
            self.row(1).dot(rhs),
            self.row(2).dot(rhs),
            self.row(3).dot(rhs),
        )
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Mat4<T> {
    type Output = Mat4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Mat4::new(
            // row 0
            self[(0, 0)] * rhs,
            self[(0, 1)] * rhs,
            self[(0, 2)] * rhs,
            self[(0, 3)] * rhs,
            // row 1
            self[(1, 0)] * rhs,
            self[(1, 1)] * rhs,
            self[(1, 2)] * rhs,
            self[(1, 3)] * rhs,
            // row 2
            self[(2, 0)] * rhs,
            self[(2, 1)] * rhs,
            self[(2, 2)] * rhs,
            self[(2, 3)] * rhs,
            // row 3
            self[(3, 0)] * rhs,
            self[(3, 1)] * rhs,
            self[(3, 2)] * rhs,
            self[(3, 3)] * rhs,
        )
    }
}

impl<T: PartialEq> PartialEq for Mat4<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}


impl<T: Mul<Output = T> + Add<Output = T> + Copy> Mul<Mat4<T>> for Mat4<T> {
    type Output = Mat4<T>;

    fn mul(self, rhs: Mat4<T>) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Mul<&Mat4<T>> for Mat4<T> {
    type Output = Mat4<T>;

    fn mul(self, rhs: &Mat4<T>) -> Self::Output {
        &self * rhs
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Mul<Mat4<T>> for &Mat4<T> {
    type Output = Mat4<T>;

    fn mul(self, rhs: Mat4<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Mul<&Mat4<T>> for &Mat4<T> {
    type Output = Mat4<T>;

    fn mul(self, rhs: &Mat4<T>) -> Self::Output {
        Mat4::new(
            // row 0
            self.row(0).dot(&rhs.col(0)),
            self.row(0).dot(&rhs.col(1)),
            self.row(0).dot(&rhs.col(2)),
            self.row(0).dot(&rhs.col(3)),
            // row 1
            self.row(1).dot(&rhs.col(0)),
            self.row(1).dot(&rhs.col(1)),
            self.row(1).dot(&rhs.col(2)),
            self.row(1).dot(&rhs.col(3)),
            // row 2
            self.row(2).dot(&rhs.col(0)),
            self.row(2).dot(&rhs.col(1)),
            self.row(2).dot(&rhs.col(2)),
            self.row(2).dot(&rhs.col(3)),
            // row 3
            self.row(3).dot(&rhs.col(0)),
            self.row(3).dot(&rhs.col(1)),
            self.row(3).dot(&rhs.col(2)),
            self.row(3).dot(&rhs.col(3)),
        )
    }
}

impl<T: Clone> Clone for Mat4<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Debug> Debug for Mat4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Mat4").field(&self.0).finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn testing() {
        let point = Vec3(-1.5, 0.0, 5.5);
        let translate_to_origin = Mat4::translate(&point * -1.0);
        let translate_back = Mat4::translate(point);

        println!("ORIGIN {:?}", translate_to_origin);
        println!("BACK {:?}", translate_back);
        println!(
            "ROW {:?} COL {:?}",
            translate_to_origin.row(0),
            translate_back.col(0)
        );
    }

    #[test]
    fn invert1() {
        let data = vec![
            // translation
            (
                Mat4::translate(Vec3(1.0, 1.0, 1.0)).invert(),
                Mat4::translate(&Vec3(1.0, 1.0, 1.0) * -1.0),
            ),
            // scale
            (
                Mat4::scale(Vec3(420.0, 69.0, 999.0)).invert(),
                // last is 1 / 999.0 but it evaluations to 0.001001001 (missing a 1 at the end)
                Mat4::scale(Vec3(1.0 / 420.0, 1.0 / 69.0, 0.0010010011)),
            ),
        ];

        for (inversion, expected) in data {
            assert_eq!(inversion.unwrap(), expected)
        }
    }

    #[test]
    fn invert2() {
        let translate = Mat4::translate(Vec3(1.0, 1.0, 1.0));
        let inverse = translate.invert().unwrap();

        let expected = Mat4::translate(&Vec3(1.0, 1.0, 1.0) * -1.0);

        assert_eq!(inverse, expected)
    }
}
