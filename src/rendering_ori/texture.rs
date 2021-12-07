use crate::gl;
use crate::rendering::get_uniform_location;
use stb_image::image::{load, LoadResult};

#[derive(Default)]
pub struct Texture {
    pub index: gl::types::GLuint,
    pub width: usize,
    pub height: usize
}

pub fn load_texture(gl: &gl::Gl, path: &str) -> Result<Texture, String> {
    let mut texture_index: gl::types::GLuint = 0;
    let result_texture = load(path);
    let mut mode = gl::RGB;
    
    unsafe {
        gl.GenTextures(1, &mut texture_index);
        gl.BindTexture(gl::TEXTURE_2D, texture_index);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    }

    let mut result = Texture {
        index: texture_index,
        ..Default::default()
    };

    match result_texture {
        LoadResult::Error(msg) => {
            println!("Error: {}", msg);
            return Err(msg);
        },
        LoadResult::ImageU8(img) => {
            println!("ImageU8: h:{}, w:{}, d:{}", img.height, img.width, img.depth);
            result.width = img.width;
            result.height = img.height;

            if img.depth == 4 {
                mode = gl::RGBA;
            }

            unsafe {
                gl.TexImage2D(
                    gl::TEXTURE_2D, 
                    0, 
                    mode as i32, 
                    img.width as i32, 
                    img.height as i32, 
                    0, 
                    mode, 
                    gl::UNSIGNED_BYTE, 
                    img.data.as_ptr() as *const gl::types::GLvoid
                );
            }
        },
        LoadResult::ImageF32(img) => {
            println!("ImageF32: h:{}, w:{}", img.height, img.width);
            result.width = img.width;
            result.height = img.height;
            
            if img.depth == 4 {
                mode = gl::RGBA;
            }

            unsafe {
                gl.TexImage2D(
                    gl::TEXTURE_2D, 
                    0, 
                    mode as i32, 
                    img.width as i32, 
                    img.height as i32, 
                    0, 
                    mode, 
                    gl::FLOAT, 
                    img.data.as_ptr() as *const gl::types::GLvoid
                );
            }
        }
    }

    Ok(result)
}

pub fn set_texture_to_program(gl: &gl::Gl, active_texture: gl::types::GLenum, texture: gl::types::GLuint, 
    program: gl::types::GLuint, uniform_name: &str) {
    unsafe {
        gl.ActiveTexture(active_texture);
        gl.BindTexture(gl::TEXTURE_2D, texture);
    }

    let uniform_location = get_uniform_location(&gl, program, uniform_name).unwrap();
    
    unsafe {
        gl.Uniform1i(uniform_location, 0);
        gl.BindTexture(gl::TEXTURE_2D, 0);
        gl.ActiveTexture(0);
    }
}

pub fn bind_texture(gl: &gl::Gl, active_texture: gl::types::GLenum, texture: gl::types::GLuint) {
    unsafe {
        gl.ActiveTexture(active_texture);
        gl.BindTexture(gl::TEXTURE_2D, texture);
    }
}

pub fn unbind_texture(gl: &gl::Gl) {
    unsafe {
        gl.ActiveTexture(0);
        gl.BindTexture(gl::TEXTURE_2D, 0);
    }
}