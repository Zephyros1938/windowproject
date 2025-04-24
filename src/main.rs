#![allow(non_snake_case, non_camel_case_types, static_mut_refs)]
/*
    Please ignore the shitty code, im new to this :plead:
*/

use asset_management::model::Model;
use camera::{CameraConstructor, cpp_camera};
use glfw::ffi::*;
use log::{debug, error};
use nalgebra_glm::{self as glm};
use shader::Shader;
use shader::light::LightImpl;
use std::ffi::{CStr, CString};
use std::ptr::{self};
use util::LinuxExitCode;
mod asset_management;
mod camera;
mod macros;
mod shader;
mod texture;
mod util;

static mut SCREEN_WIDTH: i32 = 1920;
static mut SCREEN_HEIGHT: i32 = 1080;

static mut LASTFRAME: f64 = 0f64;
static mut DELTATIME: f64 = 0f64;
static mut FIRST_MOUSE: bool = true;
static mut LAST_X: f32 = 0f32;
static mut LAST_Y: f32 = 0f32;
static mut CAMERA: cpp_camera = cpp_camera { camera: None };

fn main() -> LinuxExitCode {
    unsafe {
        util::init_logging();
        if glfwInit() == 0 {
            error!("GLFW failed to initialize!");
            panic!("GLFW failed to initialize!");
        }

        let title = CString::new("My GLFW Window").unwrap();

        let window: *mut GLFWwindow = glfwCreateWindow(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            title.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        glfwWindowHint(CONTEXT_VERSION_MAJOR as i32, 3);
        glfwWindowHint(CONTEXT_VERSION_MINOR as i32, 3);
        glfwWindowHint(OPENGL_PROFILE as i32, OPENGL_CORE_PROFILE as i32);
        glfwWindowHint(OPENGL_DEBUG_CONTEXT as i32, TRUE as i32);
        if window.is_null() {
            glfwTerminate();
            error!("Failed to create GLFW window!");
            return LinuxExitCode::ERR(1);
        }

        glfwMakeContextCurrent(window);

        let mut _procname = CString::new("");
        gl::load_with(|name| {
            _procname = CString::new(name);
            glfwGetProcAddress(_procname.as_mut().unwrap().as_ptr()) as *const _
        });
        debug!(
            "gl_proc: {:#?}",
            glfwGetProcAddress(_procname.as_mut().unwrap().as_ptr())
        );
        gl::Viewport(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);

        glfwSetFramebufferSizeCallback(window, Some(framebuffer_size_callback));
        glfwSetCursorPosCallback(window, Some(mouse_callback));
        glfwSetScrollCallback(window, Some(scroll_callback));
        glfwSetInputMode(window, CURSOR, CURSOR_DISABLED);
        CAMERA = cpp_camera::new(CameraConstructor(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32),
            None,
            None,
        ));

        let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
            .to_str()
            .expect("Unknown Version");
        debug!("Using OpenGL version: {}", version);
        let window_title = CString::new(format!("{:#?} - {}", title, version)).unwrap();
        glfw::ffi::glfwSetWindowTitle(window, window_title.as_ptr());

        let shader: Shader =
            shader::ShaderConstructor("shaders/model_loading.vert", "shaders/model_loading.frag");

        let mut view = crate::util::glmaddon::mat4(1.032);
        let projection = CAMERA.get_projection_matrix();
        let mut model = util::glmaddon::mat4(1.032);

        model = glm::rotate(&model, -0f32.to_radians(), &glm::vec3(1f32, 0.0, 0.0));
        view = glm::translate(&view, &glm::vec3(0f32, 0f32, -3f32));

        let lightPositions = [
            glm::vec3(0.7f32, 0.2f32, 2.0f32),
            glm::vec3(2.3f32, -3.3f32, -4.0f32),
            glm::vec3(-4.0f32, 2.0f32, -12.0f32),
            glm::vec3(0.0f32, 0.0f32, -3.0f32),
        ];

        let directionalLight = shader::light::DirectionalLight {
            direction: glm::vec3(-0.2f32, -1.0, -0.3),
            ambient: glm::vec3(0.05f32, 0.05, 0.05),
            diffuse: glm::vec3(0.4f32, 0.4, 0.4),
            specular: glm::vec3(0.5f32, 0.5, 0.5),
        };

        let pointLightCollection = shader::light::LightCollection {
            lights: [
                shader::light::PointLight {
                    position: lightPositions[0],
                    ambient: glm::vec3(0.05f32, 0.05, 0.05),
                    diffuse: glm::vec3(0.8f32, 0.8, 0.8),
                    specular: glm::vec3(1f32, 1f32, 1f32),
                    constant: 1f32,
                    linear: 0.09f32,
                    quadratic: 0.32f32,
                },
                shader::light::PointLight {
                    position: lightPositions[1],
                    ambient: glm::vec3(0.05f32, 0.05, 0.05),
                    diffuse: glm::vec3(0.8f32, 0.8, 0.8),
                    specular: glm::vec3(1f32, 1f32, 1f32),
                    constant: 1f32,
                    linear: 0.09f32,
                    quadratic: 0.32f32,
                },
                shader::light::PointLight {
                    position: lightPositions[2],
                    ambient: glm::vec3(0.05f32, 0.05, 0.05),
                    diffuse: glm::vec3(0.8f32, 0.8, 0.8),
                    specular: glm::vec3(1f32, 1f32, 1f32),
                    constant: 1f32,
                    linear: 0.09f32,
                    quadratic: 0.32f32,
                },
                shader::light::PointLight {
                    position: lightPositions[3],
                    ambient: glm::vec3(0.05f32, 0.05, 0.05),
                    diffuse: glm::vec3(0.8f32, 0.8, 0.8),
                    specular: glm::vec3(1f32, 1f32, 1f32),
                    constant: 1f32,
                    linear: 0.09f32,
                    quadratic: 0.32f32,
                },
            ],
        };

        let mut cameraSpotLight: shader::light::SpotLight = shader::light::SpotLight {
            position: CAMERA.get_position(),
            direction: CAMERA.get_front(),

            ambient: glm::vec3(0.2f32, 0.2f32, 0.2f32),
            diffuse: glm::vec3(0.5f32, 0.5f32, 0.5f32),
            specular: glm::vec3(1.0f32, 1.0f32, 1.0f32),

            cutOff: 12.5f32.to_radians().cos(),
            outerCutOff: 17.5f32.to_radians().cos(),

            constant: 1f32,
            linear: 0.09f32,
            quadratic: 0.032f32,
        };

        shader.activate();
        let backpack: Model = Model::new(
            "models/backpack/backpack.obj",
            None,
            Some(glm::vec3(10.0, 0.0, 10.0)),
        );
        let nanosuit: Model = Model::new(
            "models/nanosuit/nanosuit.obj",
            None,
            Some(glm::vec3(0.0, 0.0, 10.0)),
        );

        shader.setMat4("view", view, gl::FALSE);
        shader.setMat4("projection", projection, gl::FALSE);
        shader.setMat4("model", model, gl::FALSE);

        pointLightCollection.set_uniform(&shader, "pointLights");
        directionalLight.set_uniform(&shader, "dirLight");

        while glfwWindowShouldClose(window) == 0 {
            UPDATE_DELTATIME();
            process_input(window);

            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            cameraSpotLight.position = CAMERA.get_position();
            cameraSpotLight.direction = CAMERA.get_front();

            shader.activate();

            cameraSpotLight.set_uniform(&shader, "spotLight");
            shader.setVec3("viewPos", CAMERA.get_position());
            shader.setMat4("view", CAMERA.get_view_matrix(), gl::FALSE);
            shader.setMat4("projection", CAMERA.get_projection_matrix(), gl::FALSE);

            backpack.draw(&shader);
            nanosuit.draw(&shader);

            glfwSwapBuffers(window);
            glfwPollEvents();
        }

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
        CAMERA.set_aspect_ratio(SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32);
    }
}
extern "C" fn process_input(window: *mut GLFWwindow) {
    unsafe {
        if glfwGetKey(window, KEY_ESCAPE) == PRESS {
            glfwSetWindowShouldClose(window, TRUE)
        }

        if glfwGetKey(window, KEY_1) == PRESS {
            glfwSetInputMode(window, CURSOR, CURSOR_NORMAL);
            FIRST_MOUSE = true;
        }
        if glfwGetKey(window, KEY_2) == PRESS {
            glfwSetInputMode(window, CURSOR, CURSOR_DISABLED);
        }
        if glfwGetKey(window, KEY_W) == PRESS {
            CAMERA.process_keyboard(camera::CameraMovement::FORWARD, DELTATIME);
        }
        if glfwGetKey(window, KEY_S) == PRESS {
            CAMERA.process_keyboard(camera::CameraMovement::BACKWARD, DELTATIME);
        }
        if glfwGetKey(window, KEY_A) == PRESS {
            CAMERA.process_keyboard(camera::CameraMovement::LEFT, DELTATIME);
        }
        if glfwGetKey(window, KEY_D) == PRESS {
            CAMERA.process_keyboard(camera::CameraMovement::RIGHT, DELTATIME);
        }
    }
}

extern "C" fn mouse_callback(_window: *mut GLFWwindow, xposIn: f64, yposIn: f64) {
    if unsafe { glfwGetInputMode(_window, CURSOR) } == CURSOR_DISABLED {
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

            CAMERA.process_mouse(xoffset, yoffset);
        }
    }
}

extern "C" fn scroll_callback(_window: *mut GLFWwindow, _: f64, yoffset: f64) {
    unsafe {
        CAMERA.process_scroll(yoffset as f32);
    }
}

fn UPDATE_DELTATIME() {
    unsafe {
        let currentframe = glfwGetTime();
        DELTATIME = currentframe - LASTFRAME;
        LASTFRAME = currentframe;
    }
}
