mod dir;
mod utils;

use wasm_bindgen::prelude::*;

pub struct 

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, searching!");
}
