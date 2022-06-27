use image::{GenericImageView, ImageFormat};

use crate::rasterize::Color;

#[derive(Debug, Clone)]
pub struct Texture {
    pixels: Vec<Color>,
    w: u32,
    h: u32,
}

impl Texture {
    pub fn load(path: &str) -> Result<Texture, String> {
        let image = image::open(path).map_err(to_string)?;

        let (w, h) = image.dimensions();

        let mut pixels = Vec::with_capacity(w as usize * h as usize * 3);

        for y in 0..h {
            for x in 0..w {
                let pixel = image.get_pixel(x, y);
                // pixels.push(pixel[0]);
                // pixels.push(pixel[1]);
                // pixels.push(pixel[2]);
                pixels.push(Color(pixel[0], pixel[1], pixel[2]));
            }
        }

        Ok(Texture { pixels, w, h })
    }

    pub fn from_bytes(bytes: &[u8], format: ImageFormat) -> Result<Self, String> {
        let image = image::load_from_memory_with_format(bytes, format).map_err(to_string)?;

        let (w, h) = image.dimensions();

        let mut pixels = Vec::with_capacity(w as usize * h as usize * 3);

        for y in 0..h {
            for x in 0..w {
                let pixel = image.get_pixel(x, y);
                // pixels.push(pixel[0]);
                // pixels.push(pixel[1]);
                // pixels.push(pixel[2]);
                pixels.push(Color(pixel[0], pixel[1], pixel[2]));
            }
        }

        Ok(Texture { pixels, w, h })
    }

    pub fn texel(&self, u: f32, v: f32) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let x = (self.w as f32 * u) as usize;
        let y = (self.h as f32 * v) as usize;

        let x = if x == 0 { x } else { x - 1 };
        let y = if y == 0 { y } else { y - 1 };

        let i = (y * self.w as usize) + x;
        self.pixels[i]
    }
}

fn to_string<T: ToString>(t: T) -> String {
    t.to_string()
}
