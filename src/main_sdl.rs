use core::time;

use image::{open, GenericImageView};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};

use crate::{
    canvas::Canvas,
    draw::{
        draw_animated_cube_wireframe, draw_cube, draw_cube_wireframe, draw_cube_wireframe_obj,
        draw_line, draw_line_broken, draw_shaded_line, draw_shaded_triangle, draw_triangle,
        draw_wireframe_triangle, render_instance, render_model,
    },
    math::{Degrees, Mat4, Vec3},
    object::{Cube, Instance, Triangle},
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

    // draw_shaded_line(
    //     &mut sdl_canvas,
    //     (Point::new(-50.0, -200.0), Color(0, 255, 0)),
    //     (Point::new(60.0, 240.0), Color(0, 0, 255)),
    // );

    // draw_shaded_triangle(
    //     &mut sdl_canvas,
    //     (Point::new(-400.0, -250.0), Color(255, 0, 0)),
    //     (Point::new(0.0, 50.0), Color(0, 255, 0)),
    //     (Point::new(-180.0, 250.0), Color(0, 0, 255)),
    // );

    let cube = Cube::new(
        (-0.5, 0.5, -0.5).into(),
        (-0.5, -0.5, -0.5).into(),
        (0.5, -0.5, -0.5).into(),
        (0.5, 0.5, -0.5).into(),
        (-0.5, 0.5, 0.5).into(),
        (-0.5, -0.5, 0.5).into(),
        (0.5, -0.5, 0.5).into(),
        (0.5, 0.5, 0.5).into(),
        [
            Color(255, 0, 0),
            Color(255, 0, 0),
            Color(255, 0, 0),
            Color(255, 0, 0),
            Color(255, 0, 0),
            Color(255, 0, 0),
        ],
    );

    let mut instances = vec![
        Instance::new(&cube).pos((-2.0, 0.0, 0.0).into()).build(),
        Instance::new(&cube).pos((1.0, 0.0, 0.0).into()).build(),
    ];

    let aspect = sdl_canvas.height() as f32 / sdl_canvas.width() as f32;
    let camera_translation = Mat4::translate(Vec3(0.0, 0.0, -15.0));
    let camera_rotation = Mat4::identity();
    let projection = Mat4::perspective(-1.0, 1.0, -aspect, aspect, 1.0, 10.0);
    let viewport_to_canvas = Mat4::viewport_to_canvas(
        sdl_canvas.width() as f32,
        sdl_canvas.height() as f32,
        1.0,
        1.0,
    );
    let view_projection = viewport_to_canvas * projection * camera_translation * camera_rotation;

    let mut t = 0;
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
        }
        sdl_canvas.clear(Color(21, 20, 28));

        let aspect = sdl_canvas.height() as f32 / sdl_canvas.width() as f32;

        draw_animated_cube_wireframe(
            &mut sdl_canvas,
            [
                (-2.0, -0.5, 5.0).into(),
                (-2.0, 0.5, 5.0).into(),
                (-1.0, 0.5, 5.0).into(),
                (-1.0, -0.5, 5.0).into(),
            ],
            [
                (-2.0, -0.5, 6.0).into(),
                (-2.0, 0.5, 6.0).into(),
                (-1.0, 0.5, 6.0).into(),
                (-1.0, -0.5, 6.0).into(),
            ],
            (1.0, aspect),
            01.0,
            t,
            Vec3(-1.5, 0.0, 5.5),
        );

        // draw_animated_cube_wireframe(
        //     &mut sdl_canvas,
        //     [
        //         (-2.0 + 0.2, -0.5, 5.0 + 10.0).into(),
        //         (-2.0 + 0.2, 0.5, 5.0 + 10.0).into(),
        //         (-1.0 + 0.2, 0.5, 5.0 + 10.0).into(),
        //         (-1.0 + 0.2, -0.5, 5.0 + 10.0).into(),
        //     ],
        //     [
        //         (-2.0 + 0.2, -0.5, 6.0 + 10.0).into(),
        //         (-2.0 + 0.2, 0.5, 6.0 + 10.0).into(),
        //         (-1.0 + 0.2, 0.5, 6.0 + 10.0).into(),
        //         (-1.0 + 0.2, -0.5, 6.0 + 10.0).into(),
        //     ],
        //     (1.0, aspect),
        //     1.0,
        //     t,
        //     Vec3(-1.5 + 0.2, 0.0, 5.5 + 10.0),
        // );

        // draw_animated_cube_wireframe(
        //     &mut sdl_canvas,
        //     [
        //         (-2.0 + 2.0, -0.5, 5.0 + 2.5).into(),
        //         (-2.0 + 2.0, 0.5, 5.0 + 2.5).into(),
        //         (-1.0 + 2.0, 0.5, 5.0 + 2.5).into(),
        //         (-1.0 + 2.0, -0.5, 5.0 + 2.5).into(),
        //     ],
        //     [
        //         (-2.0 + 2.0, -0.5, 6.0 + 2.5).into(),
        //         (-2.0 + 2.0, 0.5, 6.0 + 2.5).into(),
        //         (-1.0 + 2.0, 0.5, 6.0 + 2.5).into(),
        //         (-1.0 + 2.0, -0.5, 6.0 + 2.5).into(),
        //     ],
        //     (1.0, aspect),
        //     1.0,
        //     t,
        //     Vec3(-1.5 + 2.0, 0.0, 5.5 + 2.5),
        // );

        render_model(
            &mut sdl_canvas,
            &Triangle {
                p0: (0.0, 0.0, 5.0).into(),
                p1: (1.0, 0.0, 5.0).into(),
                p2: (1.0, 1.0, 5.0).into(),
                color: Color(255, 0, 0),
            },
            &view_projection,
        );
        // render_model(&mut sdl_canvas, &cube, &view_projection);

        for i in instances.iter_mut() {
            i.set_rotation(Degrees((t as f32 / 60.0) * 15.0));
            i.set_pos(i.pos() + Vec3(0.0, (t as f32 / 20.0).sin() * 0.01, 0.0));
            i.update_transform_matrix();
            render_instance(&mut sdl_canvas, i, &view_projection)
        }
        // draw_cube_wireframe_obj(&mut sdl_canvas, &cube, (1.0, aspect), 1.0);

        sdl_canvas.draw();
        t += 1;
        std::thread::sleep(time::Duration::from_millis(16));
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
