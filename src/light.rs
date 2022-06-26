use crate::math::Vec3;

pub enum Light {
    Ambient(f32),
    Directional(f32, Vec3<f32>),
    Point(f32, Vec3<f32>),
}

pub enum Shading {
    Flat,
    Gourad,
    Phong,
}
