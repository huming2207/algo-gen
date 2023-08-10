mod utils;
mod algorithm_binary;
mod elf;
mod flash_device;
mod parser;
// mod generate; // TODO: revisit CMSIS pack later, do ELF first...

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, algo-gen!");
}
