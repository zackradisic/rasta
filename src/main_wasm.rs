use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    canvas::Canvas,
    draw::Rasterizer,
    light::{Light, Shading},
    math::{Degrees, Mat4, Vec3},
    object::{Cube, Instance, Model, WavefrontModel},
    rasterize::Color,
    texture::Texture,
    wasm_canvas::WasmCanvas,
    wavefront,
};

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("Window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut wasm_canvas = WasmCanvas::new(canvas, context);

    let dia = include_bytes!("../assets/textures/diamond_ore.png");
    let rust = include_bytes!("../assets/textures/rust-texture.png");
    let rust_texture: &Texture = Box::leak(Box::new(
        Texture::from_bytes(rust, image::ImageFormat::Png).unwrap(),
    ));
    let dia_texture: &Texture = Box::leak(Box::new(
        Texture::from_bytes(dia, image::ImageFormat::Png).unwrap(),
    ));
    let helmet_texture: &Texture = Box::leak(Box::new(
        Texture::from_bytes(
            include_bytes!("../assets/textures/helmet.jpeg"),
            image::ImageFormat::Jpeg,
        )
        .unwrap(),
    ));

    let helmet_model = include_bytes!("../assets/models/helmet.obj");
    let obj = wavefront::WavefrontObj::from_reader(helmet_model.as_ref(), 1.0);

    // let helmet_model = WavefrontModel::new(obj, Color(200, 200, 0), false);
    let helmet_model = Box::leak(Box::new(WavefrontModel::new_with_tex(
        obj,
        &helmet_texture,
        true,
    )));

    let helmet_instance = Box::leak(Box::new(
        Instance::new(helmet_model)
            .pos((0.0, -0.5, -1.0).into())
            // .shading(Shading::Phong)
            .build(),
    ));

    let raf_cell = Rc::new(RefCell::new(None));
    let raf = raf_cell.clone();
    let t_cell = Rc::new(RefCell::new(0));

    let aspect = wasm_canvas.height() as f32 / wasm_canvas.width() as f32;
    let camera_translation = Mat4::translate(Vec3(0.0, 0.0, 5.0));
    let camera_rotation = Mat4::identity();
    let perspective = Mat4::perspective(-1.0, 1.0, -aspect, aspect, 1.0, 1000.0);
    let viewport_to_canvas = Mat4::viewport_to_canvas(
        wasm_canvas.width() as f32,
        wasm_canvas.height() as f32,
        1.0,
        1.0,
    );
    let view_matrix = camera_translation.invert().unwrap() * camera_rotation.invert().unwrap();
    let projection = viewport_to_canvas * perspective;
    let raster = Rc::new(RefCell::new(Rasterizer::new(
        wasm_canvas.width() as f32,
        wasm_canvas.height() as f32,
        1.0,
        aspect,
        1.0,
        view_matrix,
        projection,
        vec![
            Light::Ambient(0.2),
            Light::Directional(0.4, Vec3(0.0, 0.0, 1.0)),
        ],
    )));

    let rust_textured_cube = Box::leak(Box::new(Cube::new_with_texture(
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
    )));

    let dia_textured_cube = Box::leak(Box::new(Cube::new_with_texture(
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
        &dia_texture,
    )));

    let mut instances = vec![
        Instance::new(rust_textured_cube)
            .pos((0.0, 0.0, 0.0).into())
            // .shading(Shading::Phong)
            .build(),
        Instance::new(dia_textured_cube)
            .pos((-3.0, -1.0, -3.5).into())
            // .shading(Shading::Phong)
            .build(),
    ];

    *raf.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut raster = raster.borrow_mut();
        raster.clear(&mut wasm_canvas, Color(21, 20, 28));

        let t = *t_cell.borrow();

        for (c, i) in instances.iter_mut().enumerate() {
            let s = if c % 2 == 0 { 1.0 } else { 1.0 };
            i.set_rotation(Degrees((t as f32 / 30.0) * 13.0 * s));
            let delta = (t as f32 / 120.0).sin() * 0.0045;
            i.set_pos(i.pos() + Vec3(0.0, delta, -delta + delta));
            i.update_transform_matrix();
            raster.render_instance(&mut wasm_canvas, i, i.model.texture());
        }

        helmet_instance.set_rotation(Degrees((t as f32 / 30.0) * 13.0));
        helmet_instance.update_transform_matrix();
        raster.render_instance(
            &mut wasm_canvas,
            &helmet_instance,
            helmet_instance.model.texture(),
        );

        wasm_canvas.draw();
        *t_cell.borrow_mut() += 1;
        request_animation_frame(raf_cell.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut()>));

    request_animation_frame(raf.borrow().as_ref().unwrap());
}
