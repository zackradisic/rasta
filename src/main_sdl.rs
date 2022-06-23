use core::time;

use image::{open, GenericImageView};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};

use crate::{
    canvas::Canvas,
    draw::{
        draw_line, draw_line_broken, draw_shaded_line, draw_shaded_triangle, draw_triangle,
        draw_wireframe_triangle,
    },
    rasterize::{Color, Point},
    sdl_canvas::SDLCanvas,
};

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;

fn to_string<T: ToString>(t: T) -> String {
    t.to_string()
}

pub fn main() -> Result<(), String> {
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
    let texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT)
        .map_err(to_string)?;

    let img = load_texture("./shrek.png")?;

    let mut sdl_canvas = SDLCanvas::new(WIDTH, HEIGHT, canvas, texture);

    {
        // let mut i = 0;
        for y in 0..img.h {
            for x in 0..img.w {
                let pix = img.pixels[(y * img.w) as usize + x as usize];
                // texture_buf[i] = pix.0;
                // texture_buf[i + 1] = pix.1;
                // texture_buf[i + 2] = pix.2;
                // i += 3;
                sdl_canvas.put_pixel(x as i32, img.h as i32 - y as i32, pix);
            }

            // i = (y * WIDTH * 3) as usize;
        }
    }

    draw_shaded_line(
        &mut sdl_canvas,
        (Point::new(-50, -200), Color(0, 255, 0)),
        (Point::new(60, 240), Color(0, 0, 255)),
    );

    // draw_shaded_line(
    //     &mut sdl_canvas,
    //     (Point::new(-150, -200), Color(255, 0, 0)),
    //     (Point::new(-40, 240), Color(0, 0, 255)),
    // );

    // draw_triangle(
    //     &mut sdl_canvas,
    //     Point::new(-200, -250),
    //     Point::new(200, 50),
    //     Point::new(20, 250),
    //     Color(255, 0, 0),
    // );

    draw_shaded_triangle(
        &mut sdl_canvas,
        (Point::new(-200, -250), Color(255, 0, 0)),
        (Point::new(200, 50), Color(0, 255, 0)),
        (Point::new(20, 250), Color(0, 0, 255)),
    );

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

            sdl_canvas.draw();
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
