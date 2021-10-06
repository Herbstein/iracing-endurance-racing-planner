﻿use wasm_bindgen::prelude::*;

// wasm-bindgen will automatically take care of including this script
#[wasm_bindgen(module = "/src/md-enable-functions.js")]
extern "C" {
    #[wasm_bindgen(js_name = "enableRipple")]
    pub fn enable_ripple(query_selector: &str);

    #[wasm_bindgen(js_name = "enableTextField")]
    pub fn enable_text_field(query_selector: &str);
}