use specs::{Read, ReadStorage, WriteStorage, System, Entities};
use glm::vec2;
use crate::component::{Mesh, Material, Sprite, AnimatedSprite, Spritesheet};
use crate::rendering::{
    load_texture,
};
use crate::resource::DeltaTime;

pub struct InitSprite;
pub struct InitAnimatedSprite;
pub struct UpdateAnimatedSprite;

impl<'a> System<'a> for InitSprite {
    type SystemData = (Entities<'a>,
                    ReadStorage<'a, Sprite>,
                    WriteStorage<'a, Mesh>,
                    WriteStorage<'a, Material>);

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (entities, sprites, mut meshes, mut materials) = data;

        for (entity, sprite) in (&entities, &sprites).join() {
            println!("entity {:?}", sprite);
            let texture = load_texture(&sprite.image_name).unwrap();

            let width = sprite.rect.z;
            let height = sprite.rect.w;
            meshes.insert(entity, Mesh {
                vertices: vec![
                    // positions
                    -(width as f32)/2., -(height as f32)/2., 0.0, // bottom left
                    (width as f32)/2., -(height as f32)/2., 0.0, // bottom right
                    (width as f32)/2., (height as f32)/2., 0.0, // top right
                    -(width as f32)/2., (height as f32)/2., 0.0, // top left
                ],
                uv: vec![
                    sprite.rect.x / (texture.width as f32), (sprite.rect.y + sprite.rect.w) / (texture.height as f32), // bottom left
                    (sprite.rect.x + sprite.rect.z) / (texture.width as f32), (sprite.rect.y + sprite.rect.w) / (texture.height as f32), // bottom right
                    (sprite.rect.x + sprite.rect.z) / (texture.width as f32), sprite.rect.y / (texture.height as f32), // top right
                    sprite.rect.x / (texture.width as f32), sprite.rect.y / (texture.height as f32), // top left
                ],
                colors: vec![
                    1.0, 0.0, 0.0, 1.0,
                    1.0, 1.0, 1.0, 1.0,
                    1.0, 1.0, 1.0, 1.0,
                    1.0, 1.0, 1.0, 1.0,
                ],
                indices: vec![0, 1, 2, 3],
                ..Default::default()
            }).unwrap();

            materials.insert(entity, Material {
                shader: "textured".to_string(),  
                texture_name: sprite.image_name.to_string(),
                texture: texture,
                ..Default::default()
            }).unwrap();
        }
    }
}

impl<'a> System<'a> for InitAnimatedSprite {
    type SystemData = (Entities<'a>,
                    ReadStorage<'a, Spritesheet>,
                    ReadStorage<'a, AnimatedSprite>,
                    WriteStorage<'a, Mesh>,
                    WriteStorage<'a, Material>);
    
    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (entities, spritesheet, animated_sprite, mut meshes, mut materials) = data;

        for (entity, spritesheet, animated_sprite) in (&entities, &spritesheet, &animated_sprite).join() {
            println!("entity {:?}; {:?}", spritesheet, animated_sprite);
            let texture = load_texture(&spritesheet.image_name).unwrap();

            let rect = animated_sprite.rects[animated_sprite.current_anim][animated_sprite.current_frame].clone();
            let width = rect.z;
            let height = rect.w;
            meshes.insert(entity, Mesh {
                vertices: vec![
                    // positions
                    -(width as f32)/2., -(height as f32)/2., 0.0, // bottom left
                    (width as f32)/2., -(height as f32)/2., 0.0, // bottom right
                    (width as f32)/2., (height as f32)/2., 0.0, // top right
                    -(width as f32)/2., (height as f32)/2., 0.0, // top left
                ],
                uv: vec![
                    rect.x / (texture.width as f32), (rect.y + rect.w) / (texture.height as f32), // bottom left
                    (rect.x + rect.z) / (texture.width as f32), (rect.y + rect.w) / (texture.height as f32), // bottom right
                    (rect.x + rect.z) / (texture.width as f32), rect.y / (texture.height as f32), // top right
                    rect.x / (texture.width as f32), rect.y / (texture.height as f32), // top left
                ],
                colors: vec![
                    1.0, 0.0, 0.0, 1.0,
                    1.0, 1.0, 1.0, 1.0,
                    1.0, 1.0, 1.0, 1.0,
                    1.0, 1.0, 1.0, 1.0,
                ],
                indices: vec![0, 1, 2, 3],
                ..Default::default()
            }).unwrap();

            materials.insert(entity, Material {
                shader: "offset_textured".to_string(),  
                texture_name: spritesheet.image_name.to_string(),
                texture: texture,
                ..Default::default()
            }).unwrap();
        }
    }
}

impl<'a> System<'a> for UpdateAnimatedSprite {
    type SystemData = (Read<'a, DeltaTime>,
        WriteStorage<'a, AnimatedSprite>,
        WriteStorage<'a, Material>);

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (delta_time, mut animated_sprites, mut materials) = data;

        for (sprite, material) in (&mut animated_sprites, &mut materials).join() {
            
            sprite.tick += delta_time.0;
            if sprite.tick < sprite.frame_time {
                continue;
            }

            sprite.tick = 0.;
            if sprite.current_frame < sprite.rects[sprite.current_anim].len() - 1 {
                sprite.current_frame += 1;
            } else {
                sprite.current_frame = 0;
            }
            println!("sprite.current_frame {}", sprite.current_frame);
            
            let texture = &material.texture;
            let rect = sprite.rects[sprite.current_anim][sprite.current_frame].clone();
            let offset = vec2((rect.x - sprite.rect_origin.x) / texture.width as f32, (rect.y - sprite.rect_origin.y) / texture.height as f32);

            println!("Offset {}", offset);
            material.uv_offset = offset;
        }
    }
}