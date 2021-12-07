use specs::{Component, VecStorage};
use crate::rendering::texture::Texture;
use glm::Vec2;
use gl;

#[derive(Default, Debug)]
pub struct Material {
    pub shader: String,  
    pub texture_name: String,
    pub program: gl::types::GLuint,
    pub texture: Texture,
    pub uv_offset: Vec2,
}

impl Component for Material {
    type Storage = VecStorage<Self>;
}