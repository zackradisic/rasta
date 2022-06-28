use core::time;

use image::{open, GenericImageView};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};

use crate::{
    canvas::Canvas,
    draw::Rasterizer,
    light::{Light, Shading},
    math::{Degrees, Mat4, Vec3},
    object::{Cube, Instance, Model, Triangle, WavefrontModel},
    rasterize::{Color, Point},
    sdl_canvas::SDLCanvas,
    texture::Texture,
    wavefront,
};

// const WIDTH: u32 = 960;
// const HEIGHT: u32 = 540;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

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

    // let shrek_texture = Texture::load("./shrek.png")?;
    let shrek_texture = Texture::load("./assets/textures/diamond_ore.png")?;
    let rust_texture = Texture::load("./assets/textures/rust-texture.png")?;
    let helmet_texture = Texture::load("./assets/textures/helmet.jpeg")?;

    let mut sdl_canvas = SDLCanvas::new(WIDTH, HEIGHT, canvas, texture);

    // let obj = wavefront::WavefrontObj::from_file("./assets/models/homer.obj", 1.0);
    let obj = wavefront::WavefrontObj::from_file("./assets/models/helmet.obj", 1.0);

    // let helmet_model = WavefrontModel::new(obj, Color(200, 200, 0), false);
    let helmet_model = WavefrontModel::new_with_tex(obj, &helmet_texture, true);

    let cube = Cube::new_with_texture(
        (-0.5, 0.5, 0.5).into(),
        (-0.5, -0.5, 0.5).into(),
        (0.5, -0.5, 0.5).into(),
        (0.5, 0.5, 0.5).into(),
        (-0.5, 0.5, -0.5).into(),
        (-0.5, -0.5, -0.5).into(),
        (0.5, -0.5, -0.5).into(),
        (0.5, 0.5, -0.5).into(),
        [
            // front
            [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            [(1.0, 0.0), (0.0, 0.0), (1.0, 1.0)],
            // back
            [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            [(1.0, 0.0), (0.0, 0.0), (1.0, 1.0)],
            // left
            [(1.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0)],
            // right
            [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            [(1.0, 1.0), (1.0, 0.0), (0.0, 0.0)],
            // top
            [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            [(1.0, 1.0), (1.0, 0.0), (0.0, 0.0)],
            // bot
            [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            [(1.0, 1.0), (1.0, 0.0), (0.0, 0.0)],
        ],
        &rust_texture,
    );

    let textured_cube = Cube::new_with_texture(
        (-0.5, 0.5, 0.5).into(),
        (-0.5, -0.5, 0.5).into(),
        (0.5, -0.5, 0.5).into(),
        (0.5, 0.5, 0.5).into(),
        (-0.5, 0.5, -0.5).into(),
        (-0.5, -0.5, -0.5).into(),
        (0.5, -0.5, -0.5).into(),
        (0.5, 0.5, -0.5).into(),
        [
            // front
            [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            [(1.0, 0.0), (0.0, 0.0), (1.0, 1.0)],
            // back
            [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            [(1.0, 0.0), (0.0, 0.0), (1.0, 1.0)],
            // left
            [(1.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0)],
            // right
            [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            [(1.0, 1.0), (1.0, 0.0), (0.0, 0.0)],
            // top
            [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            [(1.0, 1.0), (1.0, 0.0), (0.0, 0.0)],
            // bot
            [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            [(1.0, 1.0), (1.0, 0.0), (0.0, 0.0)],
        ],
        &shrek_texture,
    );

    let mut truck_instance = Instance::new(&helmet_model)
        .pos((0.0, -0.5, -1.0).into())
        .shading(Shading::Phong)
        .build();

    let mut instances = vec![
        Instance::new(&cube)
            .pos((-2.0, -1.0, -5.0).into())
            .rotation_y(Degrees(90.0))
            .shading(Shading::Phong)
            .build(),
        // Instance::new(&cube).pos((1.0, 0.0, -2.0).into()).build(),
        Instance::new(&textured_cube)
            .pos((-4.0, -1.0, -5.0).into())
            .shading(Shading::Phong)
            .build(),
    ];

    let aspect = sdl_canvas.height() as f32 / sdl_canvas.width() as f32;
    let camera_translation = Mat4::translate(Vec3(0.0, 0.0, 5.0));
    let camera_rotation = Mat4::identity();
    let perspective = Mat4::perspective(-1.0, 1.0, -aspect, aspect, 1.0, 1000.0);
    let viewport_to_canvas = Mat4::viewport_to_canvas(
        sdl_canvas.width() as f32,
        sdl_canvas.height() as f32,
        1.0,
        1.0,
    );
    let view_matrix = camera_translation.invert().unwrap() * camera_rotation.invert().unwrap();
    let projection = viewport_to_canvas * perspective;
    let mut raster = Rasterizer::new(
        sdl_canvas.width() as f32,
        sdl_canvas.height() as f32,
        1.0,
        aspect,
        1.0,
        view_matrix,
        projection,
        vec![
            Light::Ambient(0.2),
            Light::Directional(0.4, Vec3(0.0, 0.0, 1.0)),
        ],
    );

    let mut t = 0;
    let mut paused = false;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    paused = !paused;
                }
                _ => {}
            }
        }

        if !paused {
            raster.clear(&mut sdl_canvas, Color(21, 20, 28));

            for (c, i) in instances.iter_mut().enumerate() {
                let s = if c % 2 == 0 { -1.0 } else { 1.0 };
                i.set_rotation(Degrees((t as f32 / 30.0) * 20.0 * s));
                let delta = (t as f32 / 20.0).sin() * 0.02;
                i.set_pos(i.pos() + Vec3(0.0, delta, -delta + delta));
                i.update_transform_matrix();
                raster.render_instance(&mut sdl_canvas, i, i.model.texture());
            }
            truck_instance.set_rotation(Degrees((t as f32 / 30.0) * 20.0));
            let delta = (t as f32 / 20.0).sin() * 0.05;
            // truck_instance.set_pos(truck_instance.pos() + Vec3(0.0, delta, -delta));
            truck_instance.update_transform_matrix();
            raster.render_instance(
                &mut sdl_canvas,
                &truck_instance,
                truck_instance.model.texture(),
            );

            sdl_canvas.draw();
            t += 1;
        }
        std::thread::sleep(time::Duration::from_millis(16));
    }

    Ok(())
}
