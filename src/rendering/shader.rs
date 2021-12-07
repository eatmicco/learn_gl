use std::{ffi::{CString, CStr}};
use glm::{Vec2, Vec3, value_ptr};
use gl;

pub fn create_program(shaders: [gl::types::GLuint; 2]) -> Result<gl::types::GLuint, String> {
    let program_id = unsafe { gl::CreateProgram() };

    for shader in shaders {
        unsafe { gl::AttachShader(program_id, shader) };
    }

    unsafe { gl::LinkProgram(program_id); }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;

        unsafe {
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error: CString = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetProgramInfoLog(
                program_id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    for shader in shaders {
        unsafe { gl::DetachShader(program_id, shader) };
    }

    Ok(program_id)
}

pub fn use_program(program: gl::types::GLuint) {
    unsafe {
        gl::UseProgram(program);
    }
}

pub fn get_uniform_location(program: gl::types::GLuint, name: &str) -> Result<gl::types::GLint, String> {
    let c_name = CString::new(name).unwrap();
    let uniform_location: gl::types::GLint;
    unsafe {
        uniform_location = gl::GetUniformLocation(program, c_name.as_ptr() as *const gl::types::GLchar);
    }

    Ok(uniform_location)
}

pub fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
   let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error: CString = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer to correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len as usize));
    // convert buffer to CString
    unsafe {
        CString::from_vec_unchecked(buffer)
    }
}

pub fn push_uniform_vec2(val: &Vec2, program: gl::types::GLuint, uniform_name: &str) {
    let uniform_location = get_uniform_location(program, uniform_name).unwrap();
    unsafe {
        gl::Uniform2fv(uniform_location, 1, value_ptr(val).as_ptr());
    }
}

pub fn push_uniform_vec3(val: &Vec3, program: gl::types::GLuint, uniform_name: &str) {
    let uniform_location = get_uniform_location(program, uniform_name).unwrap();
    unsafe {
        gl::Uniform3fv(uniform_location, 1, value_ptr(val).as_ptr());
    }
}