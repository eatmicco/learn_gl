use crate::gl;

pub fn new_buffer<T>(gl: &gl::Gl, arr: Vec<T>, target: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let mut index: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut index);
        gl.BindBuffer(target, index);
        gl.BufferData(
            target,
            (arr.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
            arr.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        gl.BindBuffer(target, 0);
    }

    Ok(index)
}

pub fn bind_buffer(gl: &gl::Gl, target: gl::types::GLenum, index: gl::types::GLuint) {
    unsafe {
        gl.BindBuffer(target, index);
    }
}

pub fn unbind_buffer(gl: &gl::Gl, target: gl::types::GLenum) {
    unsafe {
        gl.BindBuffer(target, 0);
    }
}

pub fn new_vertex_array(gl: &gl::Gl) -> Result<gl::types::GLuint, String> {
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
    }

    Ok(vao)
}

pub fn bind_vertex_array(gl: &gl::Gl, vao: gl::types::GLuint) {
    unsafe {
        gl.BindVertexArray(vao);
    }
}

pub fn unbind_vertex_array(gl: &gl::Gl) {
    unsafe {
        gl.BindVertexArray(0);
    }
}

pub fn vertex_attrib_pointer(gl: &gl::Gl, buffer_index: gl::types::GLuint, index: gl::types::GLuint, size: gl::types::GLint,
type_: gl::types::GLenum, normalized: gl::types::GLboolean, stride: gl::types::GLsizei) {
    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, buffer_index);
        gl.EnableVertexAttribArray(index);
        gl.VertexAttribPointer(
            index,
            size,
            type_,
            normalized,
            stride,
            std::ptr::null()
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    }
}