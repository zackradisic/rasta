use sdl2::{render::Texture, video::Window};

use crate::{canvas::Canvas, rasterize::Color};

pub struct SDLCanvas<'a> {
    width: u32,
    height: u32,
    buffer: Vec<u8>,

    inner: sdl2::render::Canvas<Window>,
    texture: Texture<'a>,
}

impl<'a> SDLCanvas<'a> {
    pub fn new(
        width: u32,
        height: u32,
        inner: sdl2::render::Canvas<Window>,
        texture: Texture<'a>,
    ) -> Self {
        let buffer = vec![0 as u8; width as usize * height as usize * 3];

        Self {
            width,
            height,
            buffer,
            inner,
            texture,
        }
    }
}

impl<'a> Canvas for SDLCanvas<'a> {
    fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        let x: i32 = (self.width as i32 / 2) + x as i32;
        let y: i32 = (self.height as i32 / 2) - y as i32 - 1;

        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }

        let i: usize = (y as usize * self.width as usize * 3) + (x as usize * 3);

        self.buffer[i] = color.0;
        self.buffer[i + 1] = color.1;
        self.buffer[i + 2] = color.2;
    }

    fn draw(&mut self) {
        self.texture
            .with_lock(None, |buf: &mut [u8], _: usize| {
                buf.copy_from_slice(&self.buffer);
            })
            .unwrap();

        let screen_rect = sdl2::rect::Rect::new(0, 0, self.width as u32, self.height as u32);

        self.inner
            .copy(&self.texture, screen_rect, screen_rect)
            .unwrap();

        // And finally, the canvas is shown
        self.inner.present();
    }
}
