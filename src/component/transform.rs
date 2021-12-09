use specs::{Component, VecStorage};
use glm::TVec3;

#[derive(Default)]
pub struct Transform {
    pub position: TVec3<f32>,
    pub rotation_rad: f32,
    pub scale: TVec3<f32>,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}