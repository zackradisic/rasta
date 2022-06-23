use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    canvas::Canvas,
    draw::{draw_cube_wireframe, draw_line, draw_shaded_line, draw_shaded_triangle, draw_triangle},
    rasterize::{Color, Point},
    wasm_canvas::{self, WasmCanvas},
};

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

    draw_shaded_line(
        &mut wasm_canvas,
        (Point::new(-50.0, -200.0), Color(0, 255, 0)),
        (Point::new(60.0, 240.0), Color(0, 0, 255)),
    );

    draw_shaded_line(
        &mut wasm_canvas,
        (Point::new(-150.0, -200.0), Color(255, 0, 0)),
        (Point::new(-40.0, 240.0), Color(0, 0, 255)),
    );

    draw_line(
        &mut wasm_canvas,
        Point::new(0.0, 0.0),
        Point::new(420.0, 420.0),
        Color(255, 0, 0),
    );

    draw_shaded_triangle(
        &mut wasm_canvas,
        (Point::new(-200.0, -250.0), Color(255, 0, 0)),
        (Point::new(200.0, 50.0), Color(0, 255, 0)),
        (Point::new(20.0, 250.0), Color(0, 20, 255)),
    );

    let aspect = wasm_canvas.height() as f32 / wasm_canvas.width() as f32;
    draw_cube_wireframe(
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
        1.0,
    );

    wasm_canvas.draw();
}
