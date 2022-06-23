use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    canvas::Canvas,
    draw::{draw_line, draw_triangle},
    rasterize::{Color, Point},
    wasm_canvas::WasmCanvas,
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

    draw_line(
        &mut wasm_canvas,
        Point::new(0, 0),
        Point::new(420, 420),
        Color(255, 0, 0),
    );

    draw_line(
        &mut wasm_canvas,
        Point::new(-50, -200),
        Point::new(60, 240),
        Color(255, 0, 0),
    );

    draw_triangle(
        &mut wasm_canvas,
        Point::new(-200, -250),
        Point::new(200, 50),
        Point::new(20, 250),
        Color(128, 210, 0),
    );

    wasm_canvas.draw();
}
