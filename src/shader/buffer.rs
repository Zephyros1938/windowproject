use gl::types::{GLenum, GLuint};
use glfw::ffi::*;
#[allow(non_snake_case, non_camel_case_types)]
pub struct Buffer {
    id: GLuint,
    target: GLenum,
    usage: GLenum,
}

impl Buffer {
    pub unsafe fn new(target: GLenum, usage: GLenum) -> Self {
        if unsafe { glfwInit() } == 0 {
            panic!("GLFW Not Initialized!")
        }
        if unsafe { gl::GetString(gl::VERSION) } == std::ptr::null() {
            panic!("OpenGL is not loaded!")
        }
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self { id, target, usage }
    }

    pub unsafe fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }
    }
}
