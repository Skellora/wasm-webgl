use wasm_bindgen::prelude::*;

use specs::prelude::*;

use render::*;
use webgl_renderer::*;

use alert;

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

#[wasm_bindgen]
pub struct WasmWrapped {
    inner: Wrapped<WebGlRenderer>,
}

#[wasm_bindgen]
impl WasmWrapped {
    pub fn update(&mut self) {
        self.inner.update();
    }
}

pub struct Wrapped<T: Renderer> {
    w: World,
    r: Box<T>,
    p: T::Program,
}

impl<T: Renderer> Wrapped<T> {
    pub fn update(&mut self) {
        alert("Update");
        let mut systems = vec![SysA];
        for sys in systems.iter_mut() {
            sys.run_now(&self.w.res);
        }
        self.w.maintain();
    }
}

#[wasm_bindgen]
pub fn init() -> WasmWrapped {
    alert("init");
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

    renderer.clear_color(0.7, 0.7, 0.3, 1.0);
    renderer.clear(ClearMask::ColourBuffer);

    let mut d = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    renderer.set_uniform_mat4(&program, "model", &mut d).expect("set_uniform");

    renderer.draw_arrays(
        DrawMode::Triangles,
        0,
        (vertices.len() / 3) as i32,
    );

    let mut world = World::new();
    world.register::<Pos>();

    // An entity may or may not contain some component.

    world.create_entity().with(Pos { x: 0.0, y: 0.0 }).build();
    WasmWrapped {
        inner: Wrapped {
            w: world,
            r: renderer,
            p: program,
        }
    }
}

pub fn run() {
    alert("I've begun");
    let mut wrapped = init();


    alert("I'm done");
}
