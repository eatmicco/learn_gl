use specs::{Component, VecStorage};
use gl;

#[derive(Default, Debug)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub uv: Vec<f32>,
    pub colors: Vec<f32>,
    pub indices: Vec<u32>,
    pub vao: gl::types::GLuint,
    pub vertex_vbo: gl::types::GLuint,
    pub uv_vbo: gl::types::GLuint,
    pub colors_vbo: gl::types::GLuint,
    pub ibo: gl::types::GLuint
}

impl Component for Mesh {
    type Storage = VecStorage<Self>;
}