use specs::{Read, ReadStorage, WriteStorage, System};
use glm::vec3;
use crate::component::{Mesh, Material, Transform};
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
    bind_texture_unit,
    unbind_texture,
    bind_buffer,
    unbind_buffer,
    use_program,
    set_mvp_to_program,
    push_uniform_vec2
};
use crate::resource::{Camera, Projection};

pub struct InitRender;
pub struct Render;

impl<'a> System<'a> for InitRender {
    type SystemData = (WriteStorage<'a, Mesh>, 
                    WriteStorage<'a, Material>);

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (mut mesh, mut material) = data;

        unsafe {
            gl::Viewport(0,0,900, 700);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        for (mesh, material) in (&mut mesh, &mut material).join() {
            let vertex_shader_path = format!("shaders\\{}.vs", material.shader);
            let vertex_shader_src = load_cstring(&vertex_shader_path).map_err(|er| er.to_string()).unwrap();
            let vertex_shader = shader_from_source(&vertex_shader_src, gl::VERTEX_SHADER).unwrap();
        
            let fragment_shader_path = format!("shaders\\{}.fs", material.shader);
            let fragment_shader_src = load_cstring(&fragment_shader_path).map_err(|er| er.to_string()).unwrap();
            let fragment_shader = shader_from_source(&fragment_shader_src, gl::FRAGMENT_SHADER).unwrap();
        
            material.program = create_program([vertex_shader, fragment_shader]).unwrap();

            if material.texture.width == 0 {
                let texture_path = format!(".\\{}", material.texture_name);
                material.texture = load_texture(&texture_path).unwrap();
                set_texture_to_program(gl::TEXTURE0, material.texture.index, material.program, "Texture");    
            }

            mesh.vertex_vbo = new_buffer::<f32>(mesh.vertices.as_slice(), gl::ARRAY_BUFFER).unwrap();
            mesh.uv_vbo = new_buffer::<f32>(mesh.uv.as_slice(), gl::ARRAY_BUFFER).unwrap();
            mesh.colors_vbo = new_buffer::<f32>(mesh.colors.as_slice(), gl::ARRAY_BUFFER).unwrap();
            mesh.ibo = new_buffer::<u32>(mesh.indices.as_slice(), gl::ELEMENT_ARRAY_BUFFER).unwrap();

            mesh.vao = new_vertex_array().unwrap();
            bind_vertex_array(mesh.vao);
            
            vertex_attrib_pointer(mesh.vertex_vbo, 0, 3, gl::FLOAT, gl::FALSE, (3 * ::std::mem::size_of::<f32>()) as gl::types::GLint);
            vertex_attrib_pointer(mesh.uv_vbo, 1, 2, gl::FLOAT, gl::FALSE, (2 * ::std::mem::size_of::<f32>()) as gl::types::GLint);
            vertex_attrib_pointer(mesh.colors_vbo, 2, 4, gl::FLOAT, gl::FALSE, (4 * ::std::mem::size_of::<f32>()) as gl::types::GLint);

            unbind_vertex_array();
        }
    }
}

impl<'a> System<'a> for Render {
    type SystemData = (Read<'a, Projection>,
                    Read<'a, Camera>,
                    ReadStorage<'a, Transform>,
                    ReadStorage<'a, Mesh>, 
                    ReadStorage<'a, Material>);

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let (projection, camera, transform, mesh, material) = data;
        let view_matrix = glm::translation(&camera.0);
        for (transform, mesh, material) in (&transform, &mesh, &material).join() {

            let mut model_matrix = glm::translation(&transform.position);
            model_matrix = glm::rotate(&model_matrix, transform.rotation_rad, &vec3(0., 0., 1.));
            model_matrix = glm::scale(&model_matrix, &transform.scale);
            let mvp = projection.0 * view_matrix * model_matrix;
            bind_texture_unit(gl::TEXTURE0, material.texture.index);

            use_program(material.program);
            set_mvp_to_program(&mvp, material.program, "MVPMatrix");
            push_uniform_vec2(&material.uv_offset, material.program, "Offset");

            bind_vertex_array(mesh.vao);

            bind_buffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ibo);
            
            unsafe {    
                gl::DrawElements(
                    gl::TRIANGLE_FAN,
                    4,
                    gl::UNSIGNED_INT,
                    std::ptr::null()
                );
            }

            use_program(material.program);

            unbind_buffer(gl::ELEMENT_ARRAY_BUFFER);
            
            unbind_texture();
            unbind_vertex_array();
        }
    }
}