extern crate sdl2;
extern crate stb_image;
extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
extern crate num_traits;
extern crate gl;

mod component;
mod system;
mod rendering;
mod resource;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};
use specs::{Builder, World, WorldExt, RunNow, DispatcherBuilder};
use glm::{vec3, vec4};
use crate::component::{Transform, Mesh, Material, Sprite, Spritesheet, AnimatedSprite};
use crate::resource::{Projection, Camera, Keyboard, KeycodeEx, DeltaTime};
use crate::system::{InitRender, InitSprite, InitAnimatedSprite, UpdateAnimatedSprite, Render, KeyboardInput};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 4);

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let _gl_context = window.gl_create_context()?;
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut world = World::new();
    world.register::<Transform>();
    world.register::<Mesh>();
    world.register::<Material>();
    world.register::<Sprite>();
    world.register::<Spritesheet>();
    world.register::<AnimatedSprite>();

    // world.create_entity()
    //     .with(Transform { position: vec3(100., 100., 0.) })
    //     .with(Mesh {
    //         vertices: vec![
    //             // positions
    //             -102.5, -99., 0.0, // bottom left
    //             102.5, -99., 0.0, // bottom right
    //             102.5, 99., 0.0, // top right
    //             -102.5, 99., 0.0, // top left
    //         ],
    //         uv: vec![
    //             0.0, 1.0, // bottom left
    //             1.0, 1.0, // bottom right
    //             1.0, 0.0, // top right
    //             0.0, 0.0, // top left
    //         ],
    //         colors: vec![
    //             1.0, 0.0, 0.0, 1.0,
    //             1.0, 1.0, 1.0, 1.0,
    //             1.0, 1.0, 1.0, 1.0,
    //             1.0, 1.0, 1.0, 1.0,
    //         ],
    //         indices: vec![0, 1, 2, 3],
    //         ..Default::default()
    //     })
    //     .with(Material {
    //         shader: "textured".to_string(),  
    //         texture_name: "./tower.png".to_string(),
    //         ..Default::default()
    //     })
    //     .build();
    world.create_entity()
        .with(Transform { position: vec3(100., 100., 0.) })
        .with(Sprite { 
            image_name: "tower.png".to_string(), 
            rect: vec4(0., 0., 205., 198.)
         })
        .build();

    let rect_anims = vec![
        vec4(32., 0., 224., 224.),
        vec4(288., 0., 224., 224.),
        vec4(32., 256., 224., 224.),
    ];
    world.create_entity()
        .with(Transform { position: vec3(300., 300., 0.) })
        .with(Spritesheet {
            image_name: "tileset.png".to_string(),
            rects: rect_anims.clone() 
        })
        .with(AnimatedSprite {
            rects: vec![
                rect_anims.clone()
            ],
            rect_origin: rect_anims[0],
            current_anim: 0,
            current_frame: 0,
            frame_time: 0.5,
            tick: 0.
        })
        .build();

    world.insert(Projection(glm::ortho(0., 900., 0., 700., -1., 1.)));
    world.insert(Camera(glm::vec3(0., 0., 0.)));
    world.insert(Keyboard::default());
    world.insert(DeltaTime(0.0));

    let mut dispatcher = DispatcherBuilder::new()
        .with(KeyboardInput, "keyboard_input", &[])
        .with(UpdateAnimatedSprite, "update_animated_sprite", &["keyboard_input"])
        .with_thread_local(Render)
        .build();

    let mut init_sprite = InitSprite;
    init_sprite.run_now(&world);
    let mut init_animated_sprite = InitAnimatedSprite;
    init_animated_sprite.run_now(&world);
    let mut init_render = InitRender;
    init_render.run_now(&world);

    let now = Instant::now();
    let mut current_time = now.elapsed();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {

        let temp_current_time = now.elapsed();
        let delta = temp_current_time - current_time;
        current_time = temp_current_time;
        *world.write_resource::<DeltaTime>() = DeltaTime(
            delta.as_secs() as f32 + delta.subsec_nanos() as f32 * 1e-9);

        let mut keyboard = Keyboard::default();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    println!("Right Pressed");
                    keyboard = Keyboard(KeycodeEx::RightArrow);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    println!("Left Pressed");
                    keyboard = Keyboard(KeycodeEx::LeftArrow);
                },
                _ => {}
            }
        }

        *world.write_resource::<Keyboard>() = keyboard;

        dispatcher.dispatch(&mut world);
        world.maintain();

        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }

    Ok(())
}