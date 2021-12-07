use specs::{Component, VecStorage};
use glm::Vec4;

#[derive(Default, Debug)]
pub struct AnimatedSprite {
    pub rects: Vec<Vec<Vec4>>,
    pub rect_origin: Vec4,
    pub current_anim: usize,
    pub current_frame: usize,
    pub frame_time: f32,
    pub tick: f32,
}

impl Component for AnimatedSprite {
    type Storage = VecStorage<Self>;
}