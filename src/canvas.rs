use crate::rasterize::Color;

pub trait Canvas {
    fn put_pixel(&mut self, x: u32, y: u32, color: Color);

    /// Display the contents of the offscreen buffer into the canvas.
    fn draw(&mut self);
}
