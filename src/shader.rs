#![allow(non_snake_case, non_camel_case_types, dead_code)]
use std::{ffi::CString, io::Read};

use gl::types::{self, GLboolean, GLuint};
use nalgebra_glm as glm;

use crate::asset_management::get_asset;
mod buffer;
pub mod light;
pub mod material;
pub mod vertexattrib;
pub struct Shader {
    ID: GLuint,
}

#[inline(always)]
pub fn ShaderConstructor(vertexPath: &str, fragmentPath: &str) -> Shader {
    unsafe {
        // Read shader files
        let mut vertexCode = String::new();
        get_asset(vertexPath)
            .expect("Failed to read vertex shader file")
            .read_to_string(&mut vertexCode)
            .unwrap();
        let mut fragmentCode = String::new();
        get_asset(fragmentPath)
            .expect("Failed to read vertex shader file")
            .read_to_string(&mut fragmentCode)
            .unwrap();

        // Convert to CString
        let vertexSource =
            CString::new(vertexCode).expect("Could not convert vertex shader to CString");
        let fragmentSource =
            CString::new(fragmentCode).expect("Could not convert fragment shader to CString");

        let vertexShader: u32 = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertexShader, 1, &vertexSource.as_ptr(), std::ptr::null());
        gl::CompileShader(vertexShader);
        check_shader_compile(vertexShader);
        let fragmentShader: u32 = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(
            fragmentShader,
            1,
            &fragmentSource.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fragmentShader);
        check_shader_compile(fragmentShader);

        let ID = gl::CreateProgram();
        gl::AttachShader(ID, vertexShader);
        gl::AttachShader(ID, fragmentShader);
        gl::LinkProgram(ID);
        check_program_link(ID);
        gl::DeleteShader(vertexShader);
        gl::DeleteShader(fragmentShader);

        Shader { ID }
    }
}

impl Shader {
    pub fn getId(&self) -> GLuint {
        self.ID
    }
    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.ID);
        }
    }
    pub unsafe fn setBool(&self, name: &str, value: bool) {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(
                    self.ID,
                    CString::new(name)
                        .expect(format!("Could not convert {} to CString", name).as_str())
                        .as_ptr(),
                ),
                value as i32,
            );
        }
    }
    pub unsafe fn setInt(&self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.ID, name_to_ptr(name).1), value);
        }
    }
    pub unsafe fn setFloat(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.ID, name_to_ptr(name).1), value);
        }
    }
    pub unsafe fn setMat4(&self, name: &str, value: glm::TMat4<f32>, transpose: GLboolean) {
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.ID, name_to_ptr(name).1),
                1,
                transpose,
                value.as_ptr(),
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
                gl::GetUniformLocation(self.ID, name_to_ptr(name).1),
                v0,
                v1,
                v2,
                v3,
            );
        }
    }
    pub unsafe fn setVec4(&self, name: &str, value: glm::Vec4) {
        unsafe {
            gl::Uniform4f(
                gl::GetUniformLocation(self.ID, name_to_ptr(name).1),
                value.x,
                value.y,
                value.z,
                value.w,
            );
        }
    }
    pub unsafe fn setVec3(&self, name: &str, v: glm::Vec3) {
        unsafe {
            gl::Uniform3f(
                gl::GetUniformLocation(self.ID, name_to_ptr(name).1),
                v.x,
                v.y,
                v.z,
            );
        }
    }

    pub unsafe fn setVec3f(
        &self,
        name: &str,
        v0: types::GLfloat,
        v1: types::GLfloat,
        v2: types::GLfloat,
    ) {
        unsafe {
            gl::Uniform3f(
                gl::GetUniformLocation(self.ID, name_to_ptr(name).1),
                v0,
                v1,
                v2,
            );
        }
    }
}

fn name_to_ptr(name: &str) -> (CString, *const i8) {
    let cs =
        CString::new(name).unwrap_or_else(|_| panic!("Could not convert '{}' to CString", name));
    let ptr = cs.as_ptr();
    (cs, ptr)
}

fn check_shader_compile(shader: u32) {
    use gl::types::{GLchar, GLint};
    use std::ffi::CStr;
    use std::ptr;

    let mut success: GLint = 0;

    unsafe { gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success) };

    if success == 0 {
        let mut log_length: GLint = 0;

        unsafe { gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length) };

        if log_length > 0 {
            let mut buffer = Vec::with_capacity(log_length as usize);
            buffer.extend(std::iter::repeat(b' ' as i8).take(log_length as usize));
            let error_ptr = buffer.as_mut_ptr() as *mut GLchar;

            unsafe { gl::GetShaderInfoLog(shader, log_length, ptr::null_mut(), error_ptr) };

            let c_str = unsafe { CStr::from_ptr(error_ptr) };
            log::error!("Shader compilation failed:\n{}", c_str.to_string_lossy());
        } else {
            log::error!("Shader compilation failed:\n{}", "Unknown Error");
        }
    } else {
        log::debug!("Shader compiled successfully");
    }
}

fn check_program_link(program: u32) {
    use gl::types::{GLchar, GLint};
    use std::ffi::CStr;
    use std::ptr;

    let mut success: GLint = 0;

    unsafe { gl::GetProgramiv(program, gl::LINK_STATUS, &mut success) };

    if success == 0 {
        let mut log_length: GLint = 0;

        unsafe { gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length) };

        let mut buffer: Vec<i8> = Vec::with_capacity(log_length as usize);
        buffer.extend(std::iter::repeat(b' ' as i8).take(log_length as usize));
        let error_ptr = buffer.as_mut_ptr() as *mut GLchar;

        unsafe { gl::GetProgramInfoLog(program, log_length, ptr::null_mut(), error_ptr) };

        let c_str = unsafe { CStr::from_ptr(error_ptr) };
        panic!("Program linking failed:\n{}", c_str.to_string_lossy());
    } else {
        log::debug!("Program linked successfully");
    }
}
