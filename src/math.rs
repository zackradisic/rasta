use std::f32::consts::PI;
use std::fmt::Debug;
use std::ops::{Add, Div, Index, Mul, Sub};

use crate::rasterize::Color;

pub struct Vec2<T>(pub T, pub T);

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

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vec3<T> {
    pub fn dot(&self, rhs: &Vec3<T>) -> T {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
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

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from((a, b, c): (T, T, T)) -> Self {
        Self(a, b, c)
    }
}

pub struct Vec4<T>(pub T, pub T, pub T, pub T);

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
pub struct Radians(f32);

impl From<Degrees> for Radians {
    fn from(d: Degrees) -> Self {
        d.into_radians()
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

impl Mat4<f32> {
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

impl<T: Debug + Mul<Output = T> + Add<Output = T> + Copy> Mul<Vec4<T>> for &Mat4<T> {
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

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Mul<Mat4<T>> for Mat4<T> {
    type Output = Mat4<T>;

    fn mul(self, rhs: Mat4<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Mul<&Mat4<T>> for Mat4<T> {
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
}
