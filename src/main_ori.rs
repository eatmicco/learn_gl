extern crate sdl2;
extern crate stb_image;
extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
extern crate num_traits;
extern crate gl;

// mod gl {
//     include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// }

mod rendering;
mod component;
mod system;
mod resource;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use glm::vec2;
use crate::rendering::{
    load_cstring,
    shader_from_source,
    create_program,
    load_texture,
    set_texture_to_program,
    new_buffer,
    new_vertex_array,
    bind_vertex_array,
    vertex_attrib_pointer,
    unbind_vertex_array,
    bind_texture,
    unbind_texture,
    bind_buffer,
    unbind_buffer,
    use_program,
    set_mvp_to_program,
    push_uniform_vec2
};

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

    /**
     * For OpenGL, canvas is not used, we create gl_context instead
    */
    // let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // canvas.set_draw_color(Color::RGB(255, 0, 0));
    // canvas.clear();
    // canvas.present();

    let _gl_context = window.gl_create_context()?;
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let vertex_shader_src = load_cstring("shaders\\offset_textured.vs").map_err(|er| er.to_string())?;
    let vertex_shader = shader_from_source(&vertex_shader_src, gl::VERTEX_SHADER)?;

    let fragment_shader_src = load_cstring("shaders\\offset_textured.fs").map_err(|er| er.to_string())?;
    let fragment_shader = shader_from_source(&fragment_shader_src, gl::FRAGMENT_SHADER)?;

    let program = create_program([vertex_shader, fragment_shader])?;

    let projection_matrix: glm::TMat4<f32> = glm::ortho(0., 900., 0., 700., -1., 1.);
    let view_matrix = glm::translation(&glm::vec3(0., 0., 0.));
    let model_matrix = glm::translation(&glm::vec3(100., 100., 0.));
    let mvp = projection_matrix * view_matrix * model_matrix;
    
    // load texture
    let texture = load_texture("./tower.png")?;
    set_texture_to_program(gl::TEXTURE0, texture.index, program, "Texture");

    let halfw = (texture.width as f32) / 2.; 
    let halfh = (texture.height as f32) / 2.; 
    let vertices: Vec<f32> = vec![
        // positions
        -halfw, -halfh, 0.0, // bottom left
        halfw, -halfh, 0.0, // bottom right
        halfw, halfh, 0.0, // top right
        -halfw, halfh, 0.0, // top left
    ];
    // let vertices: Vec<f32> = vec![
    //     // positions
    //     -1.2277777, -1.2828572, 0.0, // bottom left
    //     -0.7722222, -1.2828572, 0.0, // bottom right
    //     -0.7722222, -0.7171428, 0.0, // top right
    //     -1.2277777, -0.7171428, 0.0, // top left
    // ];

    let uv: Vec<f32> = vec![
        0.0, 1.0, // bottom left
        1.0, 1.0, // bottom right
        1.0, 0.0, // top right
        0.0, 0.0, // top left
    ];

    let colors: Vec<f32> = vec![
        1.0, 0.0, 0.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
    ];

    let indices: Vec<u32> = vec![0, 1, 2, 3];

    // let debug_pos = mvp * glm::vec4(-halfw, -halfh, 0., 1.);
    // let debug_pos1 = mvp * glm::vec4(halfw, -halfh, 0., 1.);
    // let debug_pos2 = mvp * glm::vec4(halfw, halfh, 0., 1.);
    // let debug_pos3 = mvp * glm::vec4(-halfw, halfh, 0., 1.);
    // println!("{}", debug_pos);
    // println!("{}", debug_pos1);
    // println!("{}", debug_pos2);
    // println!("{}", debug_pos3);

    let vbo_vertices: gl::types::GLuint = new_buffer::<f32>(&vertices, gl::ARRAY_BUFFER)?;
    let vbo_uv: gl::types::GLuint = new_buffer::<f32>(&uv, gl::ARRAY_BUFFER)?;
    let vbo_color: gl::types::GLuint = new_buffer::<f32>(&colors, gl::ARRAY_BUFFER)?;
    let ibo: gl::types::GLuint = new_buffer::<u32>(&indices, gl::ELEMENT_ARRAY_BUFFER)?;

    let vao: gl::types::GLuint = new_vertex_array()?;
    bind_vertex_array(vao);
    
    vertex_attrib_pointer(vbo_vertices, 0, 3, gl::FLOAT, gl::FALSE, (3 * ::std::mem::size_of::<f32>()) as gl::types::GLint);
    vertex_attrib_pointer(vbo_uv, 1, 2, gl::FLOAT, gl::FALSE, (2 * ::std::mem::size_of::<f32>()) as gl::types::GLint);
    vertex_attrib_pointer(vbo_color, 2, 4, gl::FLOAT, gl::FALSE, (4 * ::std::mem::size_of::<f32>()) as gl::types::GLint);

    unbind_vertex_array();

    unsafe {
        gl::Viewport(0,0,900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        bind_texture(gl::TEXTURE0, texture.index);

        use_program(program);
        set_mvp_to_program(&mvp, program, "MVPMatrix");
        push_uniform_vec2(&vec2(0.5, 0.), program, "Offset");

        bind_vertex_array(vao);

        bind_buffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
        
        unsafe {    
            gl::DrawElements(
                gl::TRIANGLE_FAN,
                4,
                gl::UNSIGNED_INT,
                std::ptr::null()
            );
        }

        use_program(0);

        unbind_buffer(gl::ELEMENT_ARRAY_BUFFER);
        
        unbind_texture();
        unbind_vertex_array();

        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }

    Ok(())
}
