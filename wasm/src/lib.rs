use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub fn log(s: &str) {
    console::log(&format!("{}", s));
}
