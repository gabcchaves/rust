use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub fn log(s: &str) {
    console::log_1(&JsValue::from_str(&format!("{}", s)));
}

#[wasm_bindgen]
pub fn change_color() {
    let doc = Document::new().unwrap();
    let body = Document::query_selector_all(&doc, "body").unwrap();
    let n = body.item(0);
    console::log_1(&JsValue::from_str(&format!("{:?}", n)));
}
