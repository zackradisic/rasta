use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    canvas::Canvas,
    draw::{
        draw_animated_cube_wireframe, draw_cube_wireframe, draw_line, draw_shaded_line,
        draw_shaded_triangle, draw_triangle,
    },
    math::Vec3,
    rasterize::{Color, Point},
    wasm_canvas::{self, WasmCanvas},
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

    *raf.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        wasm_canvas.clear(Color(21, 20, 28));
        let aspect = wasm_canvas.height() as f32 / wasm_canvas.width() as f32;
        let t = *t_cell.borrow();
        draw_animated_cube_wireframe(
            &mut wasm_canvas,
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

        draw_animated_cube_wireframe(
            &mut wasm_canvas,
            [
                (-2.0 + 0.2, -0.5, 5.0 + 10.0).into(),
                (-2.0 + 0.2, 0.5, 5.0 + 10.0).into(),
                (-1.0 + 0.2, 0.5, 5.0 + 10.0).into(),
                (-1.0 + 0.2, -0.5, 5.0 + 10.0).into(),
            ],
            [
                (-2.0 + 0.2, -0.5, 6.0 + 10.0).into(),
                (-2.0 + 0.2, 0.5, 6.0 + 10.0).into(),
                (-1.0 + 0.2, 0.5, 6.0 + 10.0).into(),
                (-1.0 + 0.2, -0.5, 6.0 + 10.0).into(),
            ],
            (1.0, aspect),
            1.0,
            t,
            Vec3(-1.5 + 0.2, 0.0, 5.5 + 10.0),
        );

        draw_animated_cube_wireframe(
            &mut wasm_canvas,
            [
                (-2.0 + 2.0, -0.5, 5.0 + 2.5).into(),
                (-2.0 + 2.0, 0.5, 5.0 + 2.5).into(),
                (-1.0 + 2.0, 0.5, 5.0 + 2.5).into(),
                (-1.0 + 2.0, -0.5, 5.0 + 2.5).into(),
            ],
            [
                (-2.0 + 2.0, -0.5, 6.0 + 2.5).into(),
                (-2.0 + 2.0, 0.5, 6.0 + 2.5).into(),
                (-1.0 + 2.0, 0.5, 6.0 + 2.5).into(),
                (-1.0 + 2.0, -0.5, 6.0 + 2.5).into(),
            ],
            (1.0, aspect),
            1.0,
            t,
            Vec3(-1.5 + 2.0, 0.0, 5.5 + 2.5),
        );

        wasm_canvas.draw();

        *t_cell.borrow_mut() += 1;
        request_animation_frame(raf_cell.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut()>));

    request_animation_frame(raf.borrow().as_ref().unwrap());
}
