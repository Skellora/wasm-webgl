use js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{self, WebGlProgram, WebGlRenderingContext, WebGlShader};

use render::{ Renderer, RenderResult, BufferType, BufferDataType, DataType, ClearMask, DrawMode };

#[derive(Debug, Fail)]
pub enum RenderError {
    #[fail(display = "Could not find {} element", 0)]
    MissingElement(String),
    #[fail(display = "Could not cast an element to {}", 0)]
    CouldNotCast(String),
    #[fail(display = "JS had something to say {:?}", 0)]
    JsValueError(JsValue),
    #[fail(display = "Buffer create error")]
    BufferCreateError,
}

pub struct WebGlRenderer {
    canvas: web_sys::HtmlCanvasElement,
    context: WebGlRenderingContext,
}

fn buffer_data_type_to_web_sys_type(buffer_data_type: BufferDataType) -> u32 {
    match buffer_data_type {
        BufferDataType::Static => WebGlRenderingContext::STATIC_DRAW,
    }
}
fn buffer_type_to_web_sys_type(buffer_type: BufferType) -> u32 {
    match buffer_type {
        BufferType::Array => WebGlRenderingContext::ARRAY_BUFFER,
    }
}
fn data_type_to_web_sys_type(data_type: DataType) -> u32 {
    match data_type {
        DataType::Float => WebGlRenderingContext::FLOAT,
    }
}
fn clear_mask_to_web_sys_type(clear_mask: ClearMask) -> u32 {
    match clear_mask {
        ClearMask::ColourBuffer => WebGlRenderingContext::COLOR_BUFFER_BIT,
    }
}
fn draw_mode_to_web_sys_type(draw_mode: DrawMode) -> u32 {
    match draw_mode {
        DrawMode::Triangles => WebGlRenderingContext::TRIANGLES,
    }
}

impl Renderer for WebGlRenderer {
    type Buffer = web_sys::WebGlBuffer;
    fn new() -> RenderResult<Box<Self>> {
        let document = 
              web_sys::window()
              .ok_or(RenderError::MissingElement("Window".to_owned()))?
              .document()
              .ok_or(RenderError::MissingElement("Document".to_owned()))?;
        let canvas = document.get_element_by_id("canvas")
              .ok_or(RenderError::MissingElement("#canvas".to_owned()))?;
        let canvas: web_sys::HtmlCanvasElement = canvas
              .dyn_into::<web_sys::HtmlCanvasElement>()
              .map_err(|_| RenderError::CouldNotCast("HtmlCanvasElement".to_owned()))
                       ?;

        let context = canvas
            .get_context("webgl")
            .map_err(|js| RenderError::JsValueError(js))?
            .ok_or(RenderError::MissingElement("webgo".to_owned()))?
            .dyn_into::<WebGlRenderingContext>()
            .map_err(|_| RenderError::CouldNotCast("WebGlRenderingContext".to_owned()))
            ?;
        Ok(Box::new(Self {
            canvas,
            context,
        }))
        
    }

    fn link_and_use_program(&self, vertex_source: &str, fragment_source: &str) -> RenderResult<()> {
        let vert_shader = compile_shader(
            &self.context,
            WebGlRenderingContext::VERTEX_SHADER,
            vertex_source,
        ).unwrap();
        let frag_shader = compile_shader(
            &self.context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            fragment_source,
        ).unwrap();
        let program = link_program(&self.context, [vert_shader, frag_shader].iter()).unwrap();
        self.context.use_program(Some(&program));
        Ok(())
    }

    fn create_buffer(&self) -> RenderResult<web_sys::WebGlBuffer> {
        let buff = self.context.create_buffer()
            .ok_or(RenderError::BufferCreateError)?;
        Ok(buff)
    }

    fn bind_buffer(&self, buffer_type: BufferType, buffer: &web_sys::WebGlBuffer) {
        self.context.bind_buffer(buffer_type_to_web_sys_type(buffer_type), Some(buffer))
    }

    fn buffer_data(&self, buffer_type: BufferType, data: &[f32], buffer_data_type: BufferDataType) {
        let data_array = js_sys::Float32Array::new(&JsValue::from(data.len() as u32));
        for (i, f) in data.iter().enumerate() {
            data_array.fill(*f, i as u32, (i + 1) as u32);
        }
        self.context.buffer_data_with_opt_array_buffer(
            buffer_type_to_web_sys_type(buffer_type),
            Some(&data_array.buffer()),
            buffer_data_type_to_web_sys_type(buffer_data_type),
        )
    }

    fn vertex_attrib_pointer(&self, indx: u32, size: i32, data_type: DataType, normalized: bool, stride: i32, offset: f64) {
        self.context.vertex_attrib_pointer_with_f64(indx, size, data_type_to_web_sys_type(data_type), normalized, stride, offset)
    }

    fn enable_vertex_attrib_array(&self, index: u32) {
        self.context.enable_vertex_attrib_array(index)
    }

    fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.context.clear_color(red, green, blue, alpha)
    }

    fn clear(&self, mask: ClearMask) {
        self.context.clear(clear_mask_to_web_sys_type(mask))
    }

    fn draw_arrays(&self, mode: DrawMode, first: i32, count: i32) {
        self.context.draw_arrays(draw_mode_to_web_sys_type(mode), first, count)
    }
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".into()))
    }
}

pub fn link_program<'a, T: IntoIterator<Item = &'a WebGlShader>>(
    context: &WebGlRenderingContext,
    shaders: T,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    for shader in shaders {
        context.attach_shader(&program, shader)
    }
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program object".into()))
    }
}

