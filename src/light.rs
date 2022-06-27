use crate::math::Vec3;

pub enum Light {
    Ambient(f32),
    Directional(f32, Vec3<f32>),
    Point(f32, Vec3<f32>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shading {
    Gourad,
    Phong,
}

impl Default for Shading {
    fn default() -> Self {
        Self::Gourad
    }
}
