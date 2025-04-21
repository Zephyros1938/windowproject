#![allow(non_snake_case, non_camel_case_types)]

use camera::{Camera, CameraConstructor};
use glfw::ffi::*;
use lazy_static::lazy_static;
use log::{debug, error};
use nalgebra_glm::{self as glm};
use shader::Shader;
use std::ffi::{CStr, CString};
use std::ptr::{self};
use std::sync::Mutex;
use util::LinuxExitCode;
mod asset_management;
mod camera;
mod macros;
mod shader;
mod texture;
mod util;

static mut SCREEN_WIDTH: i32 = 800;
static mut SCREEN_HEIGHT: i32 = 600;

static mut LASTFRAME: f64 = 0f64;
static mut DELTATIME: f64 = 0f64;
static mut FIRST_MOUSE: bool = true;
static mut LAST_X: f32 = 0f32;
static mut LAST_Y: f32 = 0f32;
lazy_static! {
    static ref CAMERA: Mutex<Camera> =
        Mutex::new(CameraConstructor(None, None, None, None, None, None, None));
}

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
        glfwSetCursorPosCallback(window, Some(mouse_callback));
        glfwSetScrollCallback(window, Some(scroll_callback));
        glfwSetInputMode(window, CURSOR, CURSOR_DISABLED);

        let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
            .to_str()
            .expect("Unknown Version");
        debug!("Using OpenGL version: {}", version);
        let window_title = CString::new(format!("{:#?} - {}", title, version)).unwrap();
        glfw::ffi::glfwSetWindowTitle(window, window_title.as_ptr());

        let ourShader: Shader = shader::ShaderConstructor("shaders/test.vert", "shaders/test.frag");
        ourShader.useshader();

        let mut vao: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);
        let mut vbo: u32 = 0;
        gl::GenBuffers(1, &mut vbo);

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
        let projection =
            glm::perspective(800f32 / 600f32, CAMERA.lock().unwrap().zoom, 0.1f32, 100f32);
        let mut model = util::glmaddon::mat4(1.032);
        model = glm::rotate(&model, -55f32.to_radians(), &glm::vec3(1f32, 0.0, 0.0));
        view = glm::translate(&view, &glm::vec3(0f32, 0f32, -3f32));
        ourShader.setMat4f("view", view, gl::FALSE);
        ourShader.setMat4f("projection", projection, gl::FALSE);
        ourShader.setMat4f("model", model, gl::FALSE);
        ourShader.setFloat("depthFactor", 2.0);
        gl::BindVertexArray(0);

        while glfwWindowShouldClose(window) == 0 {
            UPDATE_DELTATIME();
            process_input(window);

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

            ourShader.useshader();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex_crate.get_texture());
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, tex_awesome.get_texture());
            gl::BindVertexArray(vao);
            ourShader.setMat4f("view", CAMERA.lock().unwrap().get_view_matrix(), gl::FALSE);
            ourShader.setMat4f(
                "projection",
                glm::perspective(
                    (SCREEN_WIDTH as f32) / (SCREEN_HEIGHT as f32),
                    CAMERA.lock().unwrap().zoom.to_radians(),
                    0.1f32,
                    100f32,
                ),
                gl::FALSE,
            );
            for i in 0..10 {
                let mut model = util::glmaddon::mat4(1.032);
                model = glm::translate(&model, &CUBE_POSITIONS[i]);
                let angle = 20f32 * (i as f32);
                model = glm::rotate(&model, angle.to_radians(), &glm::vec3(1f32, 0.0, 0.0));
                ourShader.setMat4f("model", model, gl::FALSE);
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

extern "C" fn framebuffer_size_callback(_window: *mut GLFWwindow, width: i32, height: i32) {
    unsafe {
        SCREEN_WIDTH = width;
        SCREEN_HEIGHT = height;
        gl::Viewport(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    }
}
extern "C" fn process_input(window: *mut GLFWwindow) {
    if unsafe { glfwGetKey(window, KEY_ESCAPE) } == PRESS {
        unsafe { glfwSetWindowShouldClose(window, TRUE) };
    }
    if unsafe { glfwGetKey(window, KEY_W) } == PRESS {
        unsafe {
            CAMERA
                .lock()
                .unwrap()
                .process_keyboard(camera::CameraMovement::FORWARD, DELTATIME);
        };
    }
    if unsafe { glfwGetKey(window, KEY_S) } == PRESS {
        unsafe {
            CAMERA
                .lock()
                .unwrap()
                .process_keyboard(camera::CameraMovement::BACKWARD, DELTATIME);
        };
    }
    if unsafe { glfwGetKey(window, KEY_A) } == PRESS {
        unsafe {
            CAMERA
                .lock()
                .unwrap()
                .process_keyboard(camera::CameraMovement::LEFT, DELTATIME);
        };
    }
    if unsafe { glfwGetKey(window, KEY_D) } == PRESS {
        unsafe {
            CAMERA
                .lock()
                .unwrap()
                .process_keyboard(camera::CameraMovement::RIGHT, DELTATIME);
        };
    }
}

extern "C" fn mouse_callback(_window: *mut GLFWwindow, xposIn: f64, yposIn: f64) {
    let xpos = xposIn as f32;
    let ypos = yposIn as f32;
    unsafe {
        if FIRST_MOUSE {
            LAST_X = xpos;
            LAST_Y = ypos;
            FIRST_MOUSE = false;
        }

        let xoffset = xpos - LAST_X;
        let yoffset = LAST_Y - ypos;

        LAST_X = xpos;
        LAST_Y = ypos;

        CAMERA.lock().unwrap().process_mouse(xoffset, yoffset, true);
    }
}

extern "C" fn scroll_callback(_window: *mut GLFWwindow, _: f64, yoffset: f64) {
    CAMERA.lock().unwrap().process_scroll(yoffset as f32);
}

fn UPDATE_DELTATIME() {
    unsafe {
        let currentframe = glfwGetTime();
        DELTATIME = currentframe - LASTFRAME;
        LASTFRAME = currentframe;
    }
}
