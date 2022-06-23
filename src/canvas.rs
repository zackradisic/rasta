use crate::rasterize::Color;

pub trait Canvas {
    fn put_pixel<X: IntoPixelValue, Y: IntoPixelValue>(&mut self, x: X, y: Y, color: Color);

    /// Display the contents of the offscreen buffer into the canvas.
    fn draw(&mut self);
    fn clear(&mut self, color: Color);

    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

pub fn canvas_coords_to_screen_coords<X: IntoPixelValue, Y: IntoPixelValue>(
    x: X,
    y: Y,
    width: u32,
    height: u32,
) -> Option<(u32, u32)> {
    let x: i32 = width as i32 / 2 + x.into_pixel_value();
    let y: i32 = (height as i32 / 2) - y.into_pixel_value() - 1;

    if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
        return None;
    }

    Some((x as u32, y as u32))
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
