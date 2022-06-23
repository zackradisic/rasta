use crate::rasterize::Color;

pub trait Canvas {
    fn put_pixel<X: IntoPixelValue, Y: IntoPixelValue>(&mut self, x: X, y: Y, color: Color);

    /// Display the contents of the offscreen buffer into the canvas.
    fn draw(&mut self);
}

pub trait IntoPixelValue {
    fn into_pixel_value(self) -> i32;
}

macro_rules! make_trait_impls {
	($($ty:ident),*) => {
        $(
            impl IntoPixelValue for $ty {
                fn into_pixel_value(self) -> i32 {
                    self as i32
                }
            }
        )*
    };
}

make_trait_impls!(f64, f32, i64, i32, i16, i8, isize, u64, u32, u16, u8, usize);
