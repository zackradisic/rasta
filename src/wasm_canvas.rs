use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use crate::canvas::{canvas_coords_to_screen_coords, Canvas};

pub struct WasmCanvas {
    ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    canvas_buffer: Vec<u8>,
}

impl WasmCanvas {
    pub fn new(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> Self {
        let width = canvas.width();
        let height = canvas.height();
        let canvas_buffer = ctx
            .get_image_data(0.0, 0.0, width as f64, height as f64)
            .unwrap()
            .data()
            .0;

        Self {
            ctx,
            width,
            height,
            canvas_buffer,
        }
    }
}

impl Canvas for WasmCanvas {
    fn put_pixel<X: crate::canvas::IntoPixelValue, Y: crate::canvas::IntoPixelValue>(
        &mut self,
        x: X,
        y: Y,
        color: crate::rasterize::Color,
    ) {
        match canvas_coords_to_screen_coords(x, y, self.width, self.height) {
            Some((x, y)) => {
                let data = &mut self.canvas_buffer;
                let i: usize = (y as usize * self.width as usize * 4) + (x as usize * 4);

                data[i] = color.0;
                data[i + 1] = color.1;
                data[i + 2] = color.2;
                data[i + 3] = 255;
            }
            None => (),
        }
    }

    fn clear(&mut self, color: crate::rasterize::Color) {
        let mut i = 0;
        while i < self.canvas_buffer.len() {
            self.canvas_buffer[i] = color.0;
            self.canvas_buffer[i + 1] = color.1;
            self.canvas_buffer[i + 2] = color.2;
            self.canvas_buffer[i + 3] = 255;

            i += 4;
        }
    }

    fn draw(&mut self) {
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.canvas_buffer),
            self.width,
            self.height,
        )
        .unwrap();
        self.ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}
