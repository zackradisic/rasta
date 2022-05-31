use core::time;

use image::{open, GenericImageView};
use rasterize::Color;
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};

mod rasterize;

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;

fn to_string<T: ToString>(t: T) -> String {
    t.to_string()
}

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;

    let mut event_pump = sdl_ctx.event_pump()?;
    let video_subsystem = sdl_ctx.video()?;

    let window = video_subsystem
        .window("Rasta", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(to_string)?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(to_string)?;

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT)
        .map_err(to_string)?;

    let img = load_texture("./shrek.png")?;
    let mut texture_buf: Vec<u8> = vec![0; WIDTH as usize * HEIGHT as usize * 3];

    {
        let mut i = 0;
        for y in 0..img.h {
            for x in 0..img.w {
                if (y * img.w) as usize + x as usize == 0 {
                    println!("FUCK: {} {} {}", y, x, i);
                }
                let pix = img.pixels[(y * img.w) as usize + x as usize];
                texture_buf[i] = pix.0;
                texture_buf[i + 1] = pix.1;
                texture_buf[i + 2] = pix.2;
                i += 3;
            }
            i = (y * WIDTH * 3) as usize;
        }
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }

            texture.with_lock(None, |buf: &mut [u8], _: usize| {
                buf.copy_from_slice(&texture_buf);
            })?;

            let screen_rect = sdl2::rect::Rect::new(0, 0, WIDTH, HEIGHT);
            canvas.copy(&texture, screen_rect, screen_rect)?;

            // And finally, the canvas is shown
            canvas.present();
            std::thread::sleep(time::Duration::from_millis(16));
        }
    }

    Ok(())
}

struct Img {
    pixels: Vec<Color>,
    w: u32,
    h: u32,
}

fn load_texture(path: &str) -> Result<Img, String> {
    let image = image::open(path).map_err(to_string)?;

    let (w, h) = image.dimensions();

    let mut pixels = Vec::with_capacity(w as usize * h as usize);

    for y in 0..h {
        for x in 0..w {
            let pixel = image.get_pixel(x, y);
            pixels.push(Color(pixel[0], pixel[1], pixel[2]))
        }
    }

    Ok(Img { pixels, w, h })
}
