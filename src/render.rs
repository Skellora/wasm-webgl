use failure::Error;

pub type RenderResult<T> = Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub enum BufferDataType {
    Static,
}

pub enum BufferType {
    Array,
}

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    Float,
}

#[derive(Debug, Clone, Copy)]
pub enum ClearMask {
    ColourBuffer,
}

#[derive(Debug, Clone, Copy)]
pub enum DrawMode {
    Triangles,
}

pub trait Renderer {
    type Buffer;
    fn new() -> RenderResult<Box<Self>>;
    fn link_and_use_program(&self, vertex_source: &str, fragment_source: &str) -> RenderResult<()>;
    fn create_buffer(&self) -> RenderResult<Self::Buffer>;
    fn bind_buffer(&self, buffer_type: BufferType, buffer: &Self::Buffer);
    fn buffer_data(&self, buffer_type: BufferType, data: &[f32], buffer_data_type: BufferDataType);
    fn vertex_attrib_pointer(&self, indx: u32, size: i32, data_type: DataType, normalized: bool, stride: i32, offset: f64);
    fn enable_vertex_attrib_array(&self, index: u32);
    fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32);
    fn clear(&self, clear_mask: ClearMask);
    fn draw_arrays(&self, mode: DrawMode, first: i32, count: i32);
}
