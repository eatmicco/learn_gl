use specs::{Read, ReadStorage, WriteStorage, System, Entities};
use crate::component::Transform;
use crate::resource::{Keyboard, KeycodeEx};

pub struct KeyboardInput;

impl<'a> System<'a> for KeyboardInput {
    type SystemData = (Read<'a, Keyboard>,
                     WriteStorage<'a, Transform>); 

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (keyboard, mut transform) = data;

        let keyboard = &keyboard.0;

        for transform in (&mut transform).join() {
            let mut current_pos = transform.position;
            match keyboard {
                KeycodeEx::LeftArrow => {
                    current_pos.x -= 10.;
                },
                KeycodeEx::RightArrow => {
                    current_pos.x += 10.;
                },
                KeycodeEx::UpArrow => {
                    current_pos.y += 10.;
                },
                KeycodeEx::DownArrow => {
                    current_pos.y -= 10.;
                },
                _ => {}
            }
            transform.position = current_pos;
        }
    }
}