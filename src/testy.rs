use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

use specs::prelude::*;

use render::*;
use webgl_renderer::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[derive(Debug)]
struct Pos {
    x: f64,
    y: f64,
}

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

struct SysA;

impl<'a> System<'a> for SysA {
    type SystemData = (WriteStorage<'a, Pos>);

    fn run(&mut self, mut pos: Self::SystemData) {
        for pos in (&mut pos).join() {
            pos.x += 0.01;
            if pos.x > 1.0 {
                pos.x = -1.0;
            }
            pos.y += 0.01;
            if pos.y > 1.0 {
                pos.y = -1.0;
            }
            alert(&format!("Pos: {:?}", pos));
        }
    }
}

pub fn run() {
    let renderer = WebGlRenderer::new().expect("WebGlRenderer");

    let vertex_source = r#"
        attribute vec4 position;
        uniform mat4 model;
        void main() {
            gl_Position = model * position;
        }
    "#;
    let fragment_source = r#"
        void main() {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    let program = renderer.link_and_use_program(vertex_source, fragment_source).expect("link_and_use_program");

    let vertices = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let buffer = renderer.create_buffer().expect("create_buffer");
    renderer.bind_buffer(BufferType::Array, &buffer);
    renderer.buffer_data(
        BufferType::Array,
        &vertices,
        BufferDataType::Static,
    );
    renderer.vertex_attrib_pointer(0, 3, DataType::Float, false, 0, 0.0);
    renderer.enable_vertex_attrib_array(0);

    for i in 0..10 {

        renderer.clear_color(0.7, 0.7, 0.3, 1.0);
        renderer.clear(ClearMask::ColourBuffer);

        renderer.draw_arrays(
            DrawMode::Triangles,
            0,
            (vertices.len() / 3) as i32,
        );
    }

    let mut world = World::new();
    world.register::<Pos>();

    let mut dispatcher = DispatcherBuilder::new().with(SysA, "sys_a", &[]).build();

    dispatcher.setup(&mut world.res);

    // An entity may or may not contain some component.

    world.create_entity().with(Pos { x: 0.0, y: 0.0 }).build();
    dispatcher.dispatch(&world.res);
}
