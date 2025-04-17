#![allow(non_snake_case, non_camel_case_types)]

use gl::FALSE;
use glfw::ffi::*;
use log::{debug, error};
use nalgebra_glm as glm;
use shader::Shader;
use std::ffi::{CStr, CString};
use std::ptr::{self};
use util::{LinuxExitCode, framebuffer_size_callback};
mod asset_management;
mod macros;
mod shader;
mod texture;
mod util;
// https://learnopengl.com/Getting-started/Hello-Triangle

// **constant shader test values**
const VERTICES: [f32; 180] = [
    -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5,
    -0.5, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -0.5, -0.5, 0.5, 0.0,
    0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 1.0, -0.5, 0.5,
    0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, -0.5, 1.0, 1.0,
    -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5,
    0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0,
    0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, -0.5, -0.5,
    0.0, 1.0, 0.5, -0.5, -0.5, 1.0, 1.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, -0.5,
    -0.5, 0.5, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.5, 0.5, -0.5,
    1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, 0.5, 0.0, 0.0, -0.5,
    0.5, -0.5, 0.0, 1.0,
];

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
        gl::Enable(gl::DEPTH_TEST);

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
        // let mut ebo: u32 = 0;
        // gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            sizeof_val!(VERTICES).try_into().unwrap(),
            as_c_void!(VERTICES),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            5 * sizeof!(f32),
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        debug!("Enabled Vertex Attrib Array 0");
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * sizeof!(f32),
            (3 * sizeof!(f32)) as *const _,
        );
        gl::EnableVertexAttribArray(1);
        debug!("Enabled Vertex Attrib Array 1");

        let tex_crate: texture::Texture = texture::TextureConstructor(
            "textures/container.jpg",
            gl::RGB,
            false,
            None,
            None,
            None,
            None,
        );
        let tex_awesome: texture::Texture = texture::TextureConstructor(
            "textures/awesomeface.png",
            gl::RGBA,
            true,
            None,
            None,
            None,
            None,
        );
        ourShader.setInt("texture1", 0);
        ourShader.setInt("texture2", 1);

        // https://learnopengl.com/Getting-started/Coordinate-Systems

        let CUBE_POSITIONS = [
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(2.0, 5.0, -15.0),
            glm::vec3(-1.5, -2.2, -2.5),
            glm::vec3(-3.8, -2.0, -12.3),
            glm::vec3(2.4, -0.4, -3.5),
            glm::vec3(-1.7, 3.0, -7.5),
            glm::vec3(1.3, -2.0, -2.5),
            glm::vec3(1.5, 2.0, -2.5),
            glm::vec3(1.5, 0.2, -1.5),
            glm::vec3(-1.3, 1.0, -1.5),
        ];

        let mut view = crate::util::glmaddon::mat4(1.032);
        let projection = glm::perspective(800f32 / 600f32, 45f32.to_radians(), 0.1f32, 100f32);
        let mut model = util::glmaddon::mat4(1.032);
        model = glm::rotate(&model, -55f32.to_radians(), &glm::vec3(1f32, 0.0, 0.0));
        view = glm::translate(&view, &glm::vec3(0f32, 0f32, -3f32));
        ourShader.setMat4f("view", view, FALSE);
        ourShader.setMat4f("projection", projection, FALSE);
        ourShader.setMat4f("model", model, FALSE);
        gl::BindVertexArray(0);

        while glfwWindowShouldClose(window) == 0 {
            process_input(window);

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

            ourShader.useshader();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex_crate.get_texture());
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, tex_awesome.get_texture());
            gl::BindVertexArray(vao);
            for i in 0..10 {
                let mut model = util::glmaddon::mat4(1.032);
                model = glm::translate(&model, &CUBE_POSITIONS[i]);
                let angle = 20f32 * (i as f32);
                model = glm::rotate(&model, angle.to_radians(), &glm::vec3(1f32, 0.0, 0.0));
                ourShader.setMat4f("model", model, FALSE);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

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
