pub mod canvas;
pub mod draw;
pub mod math;
pub mod rasterize;

#[cfg(not(target_arch = "wasm32"))]
pub mod main_sdl;
#[cfg(not(target_arch = "wasm32"))]
pub mod sdl_canvas;

#[cfg(target_arch = "wasm32")]
pub mod main_wasm;
#[cfg(target_arch = "wasm32")]
pub mod wasm_canvas;
