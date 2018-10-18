
extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;

extern crate specs;

#[macro_use] extern crate failure;

mod render;
use render::*;
mod webgl_renderer;
#[cfg(target_arch = "wasm32")]
mod testy;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;


#[cfg(target_arch = "wasm32")]
type R = webgl_renderer::WebGlRenderer;

pub fn get_renderer() -> RenderResult<Box<R>> {
    Renderer::new()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run() {
    testy::run();
}
