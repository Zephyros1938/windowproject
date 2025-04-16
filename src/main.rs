use gl::{ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER, STATIC_DRAW};
use glfw::ffi::*;
use shader::Shader;
use std::ffi::{CStr, CString};
use std::ptr::{self};
mod util;
use log::{debug, error};
use util::*;
mod asset_management;
mod macros;
mod shader;
mod texture;
// https://learnopengl.com/Getting-started/Hello-Triangle

// **constant shader test values**
const VERTICES: [f32; 32] = [
    // positions          // colors           // texture coords
    0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
    0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
    -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
    -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
];
const INDICES: [u32; 6] = [
    0, 1, 3, // first triangle
    1, 2, 3, // second triangle
];

#[allow(non_camel_case_types, non_snake_case)]
fn main() -> LinuxExitCode {
    unsafe {
        util::init_log4rs();
        if glfw::ffi::glfwInit() == 0 {
            error!("GLFW failed to initialize!");
            panic!("GLFW failed to initialize!");
        }

        let width: i32 = 800;
        let height: i32 = 600;
        let title = CString::new("My GLFW Window").unwrap();

        let window = glfwCreateWindow(
            width,
            height,
            title.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );

        glfwWindowHint(CONTEXT_VERSION_MAJOR as i32, 3);
        glfwWindowHint(CONTEXT_VERSION_MINOR as i32, 3);
        glfwWindowHint(OPENGL_PROFILE as i32, OPENGL_CORE_PROFILE as i32);
        if window.is_null() {
            glfwTerminate();
            error!("Failed to create GLFW window!");
            return LinuxExitCode::ERR(1);
        }

        glfwMakeContextCurrent(window);

        gl::load_with(|name| {
            let cstr = CString::new(name).unwrap();
            glfwGetProcAddress(cstr.as_ptr()) as *const _
        });
        gl::Viewport(0, 0, 800, 600);

        glfwSetFramebufferSizeCallback(window, Some(framebuffer_size_callback));

        let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
            .to_str()
            .expect("Unknown Version");
        debug!("Using OpenGL version: {}", version);
        let window_title = CString::new(format!("{:#?} - {}", title, version)).unwrap();
        glfw::ffi::glfwSetWindowTitle(window, window_title.as_ptr());

        fn process_input(window: *mut GLFWwindow) {
            if unsafe { glfwGetKey(window, KEY_ESCAPE) } == PRESS {
                unsafe { glfwSetWindowShouldClose(window, TRUE) };
            }
        }

        let ourShader: Shader = shader::ShaderConstructor("shaders/test.vert", "shaders/test.frag");
        ourShader.useshader();

        let mut vao: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);
        let mut vbo: u32 = 0;
        gl::GenBuffers(1, &mut vbo);
        let mut ebo: u32 = 0;
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(ARRAY_BUFFER, vbo);
        gl::BufferData(
            ARRAY_BUFFER,
            sizeof_val!(VERTICES).try_into().unwrap(),
            as_c_void!(VERTICES),
            STATIC_DRAW,
        );
        gl::BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            ELEMENT_ARRAY_BUFFER,
            sizeof_val!(INDICES).try_into().unwrap(),
            as_c_void!(INDICES),
            STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * sizeof!(f32),
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        debug!("Enabled Vertex Attrib Array 0");
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * sizeof!(f32),
            (3 * sizeof!(f32)) as *const _,
        );
        gl::EnableVertexAttribArray(1);
        debug!("Enabled Vertex Attrib Array 1");
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            8 * sizeof!(f32),
            (6 * sizeof!(f32)) as *const _,
        );
        gl::EnableVertexAttribArray(2);
        debug!("Enabled Vertex Attrib Array 2");

        let tex_crate: texture::Texture =
            texture::TextureConstructor("textures/container.jpg", gl::RGB, false);
        let tex_awesome: texture::Texture =
            texture::TextureConstructor("textures/awesomeface.png", gl::RGBA, true);
        ourShader.setInt("texture1", 0);
        ourShader.setInt("texture2", 1);
        gl::BindVertexArray(0);

        while glfwWindowShouldClose(window) == 0 {
            process_input(window);

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            ourShader.useshader();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex_crate.get_texture());
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, tex_awesome.get_texture());
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

            glfwPollEvents();
            glfwSwapBuffers(window);
        }

        gl::DeleteVertexArrays(1, &mut vao);
        gl::DeleteBuffers(1, &mut vbo);
        gl::DeleteProgram(ourShader.getId());

        glfwDestroyWindow(window);
        glfwTerminate();
    }
    LinuxExitCode::OK
}
