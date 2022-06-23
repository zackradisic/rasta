use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use crate::canvas::{canvas_coords_to_screen_coords, Canvas};

pub struct WasmCanvas {
    ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    canvas_buffer: ImageData,
}

impl WasmCanvas {
    pub fn new(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> Self {
        let width = canvas.width();
        let height = canvas.height();
        let canvas_buffer = ctx
            .get_image_data(0.0, 0.0, width as f64, height as f64)
            .unwrap();

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
                let mut data = self.canvas_buffer.data();
                let i: usize = (y as usize * self.width as usize * 4) + (x as usize * 4);

                data[i] = color.0;
                data[i + 1] = color.1;
                data[i + 2] = color.2;
                data[i + 3] = 255;

                self.canvas_buffer = ImageData::new_with_u8_clamped_array_and_sh(
                    Clamped(&mut data.0),
                    self.width,
                    self.height,
                )
                .unwrap();
            }
            None => (),
        }
    }

    fn draw(&mut self) {
        self.ctx
            .put_image_data(&self.canvas_buffer, 0.0, 0.0)
            .unwrap();
    }
}
