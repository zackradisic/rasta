#[cfg(not(feature = "wasm"))]
fn main() -> Result<(), String> {
    rasta::main_sdl::main()
}

#[cfg(feature = "wasm")]
fn main() -> Result<(), String> {
    rasta::main_wasm::main()
}
