use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

#[wasm_bindgen]
pub fn log(s: &str) {
    console::log_1(&JsValue::from_str(&format!("{}", s)));
}

#[wasm_bindgen]
pub fn change_color() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::Document>().unwrap();
    console::log_1(&JsValue::from_str(&format!("{:?}", html_document)));
}
