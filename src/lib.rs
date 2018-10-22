
extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;

extern crate specs;

#[macro_use] extern crate failure;

mod render;
use render::*;
mod webgl_renderer;
mod testy;
pub use testy::*;

use wasm_bindgen::prelude::*;


#[cfg(target_arch = "wasm32")]
type R = webgl_renderer::WebGlRenderer;

pub fn get_renderer() -> RenderResult<Box<R>> {
    Renderer::new()
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn run() {
    use std::panic;
    panic::set_hook(Box::new(|e| {
        let estr = &format!("Behoo: {:?}", e);
        web_sys::console::log_1(&estr.into());
        alert(&estr);
    }));
    testy::run();
}
