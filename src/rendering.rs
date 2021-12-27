pub mod shader;
pub mod resource;
pub mod buffer;
pub mod texture;
pub mod transform;

pub use self::shader::{
    create_program, 
    shader_from_source,
    use_program,
    get_uniform_location,
    get_attrib_location,
    push_uniform_vec2,
    push_uniform_vec3
};
pub use self::resource::load_cstring;
pub use self::buffer::{
    new_buffer,
    bind_buffer,
    unbind_buffer,
    new_vertex_array,
    bind_vertex_array,
    unbind_vertex_array,
    vertex_attrib_pointer
};
pub use self::texture::{
    load_texture, 
    set_texture_to_program,
    gen_texture,
    bind_texture_unit,
    bind_texture,
    unbind_texture,
    get_current_bound_texture,
    set_texture_filter,
    set_pixel_store_mode,
    set_texture_2d
};
pub use self::transform::set_mvp_to_program;