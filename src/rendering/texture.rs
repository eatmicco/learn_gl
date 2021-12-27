use gl;
use crate::rendering::get_uniform_location;
use stb_image::image::{load, LoadResult};

#[derive(Default, Debug)]
pub struct Texture {
    pub index: gl::types::GLuint,
    pub width: usize,
    pub height: usize
}

pub fn load_texture(path: &str) -> Result<Texture, String> {
    
    let result_texture = load(path);
    let mut mode = gl::RGB;
    let texture_index = gen_texture()?;
    bind_texture(texture_index);

    set_texture_filter(gl::NEAREST);

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

            set_texture_2d::<u8>(mode, img.width as i32, img.height as i32, gl::UNSIGNED_BYTE, &img.data[..]);
        },
        LoadResult::ImageF32(img) => {
            println!("ImageF32: h:{}, w:{}", img.height, img.width);
            result.width = img.width;
            result.height = img.height;
            
            if img.depth == 4 {
                mode = gl::RGBA;
            }

            set_texture_2d::<f32>(mode, img.width as i32, img.height as i32, gl::FLOAT, &img.data[..]);
        }
    }

    Ok(result)
}

pub fn set_texture_to_program(active_texture: gl::types::GLenum, texture: gl::types::GLuint, 
    program: gl::types::GLuint, uniform_name: &str) {
    bind_texture_unit(active_texture, texture);
    let uniform_location = get_uniform_location(program, uniform_name).unwrap();
    
    unsafe {
        gl::Uniform1i(uniform_location, 0);
    }

    unbind_texture();
}

pub fn gen_texture() -> Result<gl::types::GLuint, String> {
    let mut texture_index: gl::types::GLuint = 0;

    unsafe {
        gl::GenTextures(1, &mut texture_index);
    }

    Ok(texture_index)
}

pub fn bind_texture(texture: gl::types::GLuint) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture);
    }
}

pub fn bind_texture_unit(active_texture: gl::types::GLenum, texture: gl::types::GLuint) {
    unsafe {
        gl::ActiveTexture(active_texture);
    }

    bind_texture(texture);
}

pub fn unbind_texture() {
    unsafe {
        gl::ActiveTexture(0);
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
}

pub fn get_current_bound_texture() -> Result<gl::types::GLint, String> {
    let mut current_texture: gl::types::GLint = 0;
    
    unsafe {
        gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut current_texture);
    }

    Ok(current_texture)
}

pub fn set_texture_filter(filter: gl::types::GLenum) {
    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter as i32);
    }
}

pub fn set_pixel_store_mode(mode: gl::types::GLenum) {
    unsafe {
        gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 0);
    }
}

pub fn set_texture_2d<T>(mode: gl::types::GLenum, width: i32, height: i32, data_type: gl::types::GLenum, data: &[T]) {
    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            mode as i32, 
            width, 
            height, 
            0, 
            mode, 
            data_type, 
            data.as_ptr() as *const gl::types::GLvoid
        );
    }
}