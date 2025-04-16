use gl::{self, types::GLenum};
use glfw::ffi::TRUE;
use log::{debug, error};
use stb_image;

pub struct Texture {
    ID: u32,
}

impl Texture {
    pub fn get_texture(&self) -> u32 {
        self.ID
    }
}

pub unsafe fn TextureConstructor(
    location: &str,
    format: GLenum,
    flip: bool,
    wrap_s: Option<GLenum>,
    wrap_t: Option<GLenum>,
    min_filter: Option<GLenum>,
    mag_filter: Option<GLenum>,
) -> Texture {
    unsafe {
        debug!("Loading Texture");
        let mut texture = 0;

        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            match wrap_s {
                Some(gl::REPEAT) => gl::REPEAT,
                None => gl::REPEAT,
                Some(_) => gl::REPEAT,
            } as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut nrChannels: i32 = 0;
        let loc = crate::asset_management::get_asset_path_cstr(location)
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
            debug!("Loaded Texture");
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width,
                height,
                0,
                format,
                gl::UNSIGNED_BYTE,
                data,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        } else {
            error!("Failed to load texture: [{}]!", location);
            panic!("Failed to load texture: [{}]!", location);
        }
        stb_image::stb_image::stbi_image_free(data);
        Texture { ID: texture }
    }
}
