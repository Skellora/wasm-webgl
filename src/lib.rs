
#![allow(unused_variables)]
extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;

#[macro_use] extern crate failure;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

mod render;
use render::*;
mod webgl_renderer;
use webgl_renderer::*;

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
    let renderer = WebGlRenderer::new().unwrap();

    let vertex_source = r#"
        attribute vec4 position;
        void main() {
            gl_Position = position;
        }
    "#;
    let fragment_source = r#"
        void main() {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    renderer.link_and_use_program(vertex_source, fragment_source).unwrap();

    let vertices = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let buffer = renderer.create_buffer().unwrap();
    renderer.bind_buffer(BufferType::Array, &buffer);
    renderer.buffer_data(
        BufferType::Array,
        &vertices,
        BufferDataType::Static,
    );
    renderer.vertex_attrib_pointer(0, 3, DataType::Float, false, 0, 0.0);
    renderer.enable_vertex_attrib_array(0);

    renderer.clear_color(0.7, 0.7, 0.3, 1.0);
    renderer.clear(ClearMask::ColourBuffer);

    renderer.draw_arrays(
        DrawMode::Triangles,
        0,
        (vertices.len() / 3) as i32,
    );
}
