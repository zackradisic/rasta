#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), String> {
    rasta::main_sdl::main()
}

#[cfg(target_arch = "wasm32")]
fn main() -> Result<(), String> {
    rasta::main_wasm::start();
    Ok(())
}
