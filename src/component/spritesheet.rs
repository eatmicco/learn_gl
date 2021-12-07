use specs::{Component, VecStorage};
use glm::Vec4;

#[derive(Default, Debug)]
pub struct Spritesheet {
    pub image_name: String,
    pub rects: Vec<Vec4>
}

impl Component for Spritesheet {
    type Storage = VecStorage<Self>;
}