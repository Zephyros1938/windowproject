#![allow(non_snake_case, non_camel_case_types, dead_code)]
use gl::{self, types::GLenum};
use glfw::ffi::TRUE;
use stb_image;

#[repr(C)]
#[derive(Clone)]
pub struct Texture {
    pub ID: u32,
    pub type_s: String,
    pub path: String,
}

impl Texture {
    pub fn get_texture(&self) -> u32 {
        self.ID
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            ID: 0,
            type_s: String::new(),
            path: String::new(),
        }
    }
}

pub unsafe fn TextureConstructor(
    path: String,
    flip: bool,
    wrap_s: Option<GLenum>,
    wrap_t: Option<GLenum>,
    min_filter: Option<GLenum>,
    mag_filter: Option<GLenum>,
    type_s: String,
) -> Texture {
    unsafe {
        let mut texture = 0;

        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            match wrap_s {
                Some(gl::MIRRORED_REPEAT) => gl::MIRRORED_REPEAT,
                Some(gl::CLAMP_TO_EDGE) => gl::CLAMP_TO_EDGE,
                None | Some(_) => gl::REPEAT,
            } as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            match wrap_t {
                Some(gl::MIRRORED_REPEAT) => gl::MIRRORED_REPEAT,
                Some(gl::CLAMP_TO_EDGE) => gl::CLAMP_TO_EDGE,
                None | Some(_) => gl::REPEAT,
            } as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            match min_filter {
                Some(gl::LINEAR) => gl::LINEAR,
                Some(gl::NEAREST_MIPMAP_NEAREST) => gl::NEAREST_MIPMAP_NEAREST,
                Some(gl::LINEAR_MIPMAP_NEAREST) => gl::LINEAR_MIPMAP_NEAREST,
                Some(gl::NEAREST_MIPMAP_LINEAR) => gl::NEAREST_MIPMAP_LINEAR,
                Some(gl::NEAREST) => gl::NEAREST,
                None | Some(_) => gl::LINEAR_MIPMAP_LINEAR,
            } as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            match mag_filter {
                Some(gl::NEAREST) => gl::NEAREST,
                Some(gl::NEAREST_MIPMAP_NEAREST) => gl::NEAREST_MIPMAP_NEAREST,
                Some(gl::LINEAR_MIPMAP_NEAREST) => gl::LINEAR_MIPMAP_NEAREST,
                Some(gl::NEAREST_MIPMAP_LINEAR) => gl::NEAREST_MIPMAP_LINEAR,
                Some(gl::LINEAR_MIPMAP_LINEAR) => gl::LINEAR_MIPMAP_LINEAR,
                None | Some(_) => gl::LINEAR,
            } as i32,
        );
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut nrChannels: i32 = 0;
        let loc = crate::asset_management::get_asset_path_cstr(path.as_str())
            .expect("Could not get asset path");
        if flip {
            stb_image::stb_image::stbi_set_flip_vertically_on_load(TRUE);
        }
        let data: *mut std::ffi::c_void = stb_image::stb_image::stbi_load(
            loc.as_ptr(),
            &mut width,
            &mut height,
            &mut nrChannels,
            0,
        ) as *mut _;
        if !data.is_null() {
            let format = match nrChannels {
                1 => gl::RED,
                2 => gl::RG,
                3 => gl::RGB,
                4 => gl::RGBA,
                _ => panic!(
                    "Could not get number of channels for Texture {}\r\n\tCHNLS: {}",
                    loc.into_string().unwrap().as_str(),
                    nrChannels
                ),
            };
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                width,
                height,
                0,
                format,
                gl::UNSIGNED_BYTE,
                data,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        } else {
            panic!("Failed to load texture: [{}]!", path);
        }
        stb_image::stb_image::stbi_image_free(data);
        Texture {
            ID: texture,
            type_s,
            path,
        }
    }
}
