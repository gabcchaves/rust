use wasm_bindgen::prelude::*;
use web_sys::prelude::*;

#[wasm_bindgen]
pub fn log(s: &str) {
    console_log(&format!("{}", s));
}
