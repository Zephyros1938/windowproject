use gl::types::{self, GLuint};
use log::debug;

use crate::asset_management::read_asset_to_cstr;
use crate::macros::*;
mod buffer;
#[allow(non_snake_case, non_camel_case_types)]
pub struct Shader {
    ID: GLuint,
}

pub fn ShaderConstructor(vertexPath: &str, fragmentPath: &str) -> Shader {
    unsafe {
        let vertexSource = read_asset_to_cstr(vertexPath);
        let fragmentSource = read_asset_to_cstr(fragmentPath);
        let vertexShader: u32 = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(
            vertexShader,
            1,
            crate::cstr_to_ptr_array!(vertexSource),
            std::ptr::null(),
        );
        gl::CompileShader(vertexShader);
        crate::check_shader_compile!(vertexShader);
        let fragmentShader: u32 = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(
            fragmentShader,
            1,
            crate::cstr_to_ptr_array!(fragmentSource),
            std::ptr::null(),
        );
        gl::CompileShader(fragmentShader);
        crate::check_shader_compile!(fragmentShader);

        let ID = gl::CreateProgram();
        gl::AttachShader(ID, vertexShader);
        gl::AttachShader(ID, fragmentShader);
        gl::LinkProgram(ID);
        crate::check_program_link!(ID);
        gl::DeleteShader(vertexShader);
        gl::DeleteShader(fragmentShader);

        Shader { ID }
    }
}

impl Shader {
    pub fn getId(&self) -> GLuint {
        self.ID
    }
    pub unsafe fn useshader(&self) {
        unsafe {
            gl::UseProgram(self.ID);
        }
    }
    pub unsafe fn setBool(&self, name: &str, value: bool) {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.ID, crate::cstr_ptr!(name)),
                value as i32,
            );
        }
    }
    pub unsafe fn setInt(&self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.ID, crate::cstr_ptr!(name)),
                value,
            );
        }
    }
    pub unsafe fn setFloat(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(
                gl::GetUniformLocation(self.ID, crate::cstr_ptr!(name)),
                value,
            );
        }
    }
    pub unsafe fn setVec4f(
        &self,
        name: &str,
        v0: types::GLfloat,
        v1: types::GLfloat,
        v2: types::GLfloat,
        v3: types::GLfloat,
    ) {
        unsafe {
            gl::Uniform4f(
                gl::GetUniformLocation(self.ID, crate::cstr_ptr!(name)),
                v0,
                v1,
                v2,
                v3,
            );
        }
    }
}
