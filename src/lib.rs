pub mod canvas;
pub mod draw;
pub mod lerp;
pub mod light;
pub mod math;
pub mod object;
pub mod rasterize;
pub mod texture;
pub mod wavefront;

#[cfg(not(target_arch = "wasm32"))]
pub mod main_sdl;
#[cfg(not(target_arch = "wasm32"))]
pub mod sdl_canvas;

#[cfg(target_arch = "wasm32")]
pub mod main_wasm;
#[cfg(target_arch = "wasm32")]
pub mod wasm_canvas;
