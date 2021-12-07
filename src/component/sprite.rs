use specs::{Component, VecStorage};
use glm::Vec4;

#[derive(Default, Debug)]
pub struct Sprite {
    pub image_name: String,
    pub rect: Vec4
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}