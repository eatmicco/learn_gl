use crate::gl;
use crate::rendering::get_uniform_location;
use glm::{TMat4, value_ptr};

pub fn set_mvp_to_program(gl: &gl::Gl, mvp: &TMat4<f32>, program: gl::types::GLuint, uniform_name: &str) {
    
    let uniform_location = get_uniform_location(&gl, program, uniform_name).unwrap();
    unsafe {
        gl.UniformMatrix4fv(uniform_location, 1, gl::FALSE, value_ptr(mvp).as_ptr());
    }
}