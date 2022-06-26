use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    canvas::Canvas,
    draw::Rasterizer,
    light::Light,
    math::{Degrees, Mat4, Vec3},
    object::{Cube, Instance},
    rasterize::Color,
    wasm_canvas::WasmCanvas,
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

    wasm_canvas.clear(Color(21, 20, 28));

    // draw_shaded_line(
    //     &mut wasm_canvas,
    //     (Point::new(-50.0, -200.0), Color(0, 255, 0)),
    //     (Point::new(60.0, 240.0), Color(0, 0, 255)),
    // );

    // draw_shaded_line(
    //     &mut wasm_canvas,
    //     (Point::new(-150.0, -200.0), Color(255, 0, 0)),
    //     (Point::new(-40.0, 240.0), Color(0, 0, 255)),
    // );

    // draw_line(
    //     &mut wasm_canvas,
    //     Point::new(0.0, 0.0),
    //     Point::new(420.0, 420.0),
    //     Color(255, 0, 0),
    // );

    // draw_shaded_triangle(
    //     &mut wasm_canvas,
    //     (Point::new(-200.0, -250.0), Color(255, 0, 0)),
    //     (Point::new(200.0, 50.0), Color(0, 255, 0)),
    //     (Point::new(20.0, 250.0), Color(0, 20, 255)),
    // );

    // let aspect = wasm_canvas.height() as f32 / wasm_canvas.width() as f32;
    // draw_cube_wireframe(
    //     &mut wasm_canvas,
    //     [
    //         (-2.0, -0.5, 5.0).into(),
    //         (-2.0, 0.5, 5.0).into(),
    //         (-1.0, 0.5, 5.0).into(),
    //         (-1.0, -0.5, 5.0).into(),
    //     ],
    //     [
    //         (-2.0, -0.5, 6.0).into(),
    //         (-2.0, 0.5, 6.0).into(),
    //         (-1.0, 0.5, 6.0).into(),
    //         (-1.0, -0.5, 6.0).into(),
    //     ],
    //     (1.0, aspect),
    //     1.0,
    // );

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
    let mut raster = Rc::new(RefCell::new(Rasterizer::new(
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

    let cube = Cube::new(
        (-0.5, 0.5, 0.5).into(),
        (-0.5, -0.5, 0.5).into(),
        (0.5, -0.5, 0.5).into(),
        (0.5, 0.5, 0.5).into(),
        (-0.5, 0.5, -0.5).into(),
        (-0.5, -0.5, -0.5).into(),
        (0.5, -0.5, -0.5).into(),
        (0.5, 0.5, -0.5).into(),
        [
            Color(255, 0, 0),
            Color(0, 255, 0),
            Color(0, 0, 255),
            Color(255, 0, 255),
            Color(255, 255, 0),
            Color(0, 255, 255),
        ],
    );
    let cube: &Cube = Box::leak(Box::new(cube));

    let mut instances = vec![
        Instance::new(cube).pos((-1.0, 0.0, 0.0).into()).build(),
        Instance::new(cube).pos((1.0, 0.0, 0.0).into()).build(),
    ];

    let identity = Mat4::identity();

    *raf.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        raster
            .borrow_mut()
            .clear(&mut wasm_canvas, Color(21, 20, 28));

        let t = *t_cell.borrow();

        // draw_animated_cube_wireframe(
        //     &mut wasm_canvas,
        //     [
        //         (-2.0, -0.5, 5.0).into(),
        //         (-2.0, 0.5, 5.0).into(),
        //         (-1.0, 0.5, 5.0).into(),
        //         (-1.0, -0.5, 5.0).into(),
        //     ],
        //     [
        //         (-2.0, -0.5, 6.0).into(),
        //         (-2.0, 0.5, 6.0).into(),
        //         (-1.0, 0.5, 6.0).into(),
        //         (-1.0, -0.5, 6.0).into(),
        //     ],
        //     (1.0, aspect),
        //     01.0,
        //     t,
        //     Vec3(-1.5, 0.0, 5.5),
        // );

        // draw_animated_cube_wireframe(
        //     &mut wasm_canvas,
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
        //     &mut wasm_canvas,
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

        for (c, i) in instances.iter_mut().enumerate() {
            let s = if c % 2 == 0 { 1.0 } else { 1.0 };
            i.set_rotation(Degrees((t as f32 / 30.0) * 20.0 * s));
            let delta = (t as f32 / 120.0).sin() * 0.0045;
            i.set_pos(i.pos() + Vec3(0.0, delta, -delta + delta));
            i.update_transform_matrix();
            raster.borrow_mut().render_instance(&mut wasm_canvas, i)
        }

        wasm_canvas.draw();

        *t_cell.borrow_mut() += 1;
        request_animation_frame(raf_cell.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut()>));

    request_animation_frame(raf.borrow().as_ref().unwrap());
}
