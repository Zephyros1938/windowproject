#![allow(non_snake_case, non_camel_case_types, static_mut_refs)]
/*
    Please ignore the shitty code, im new to this :plead:
*/

use camera::{CameraConstructor, cpp_camera};
use glfw::ffi::*;
use log::{debug, error};
use nalgebra_glm::{self as glm};
use shader::Shader;
use shader::light::LightImpl;
use std::ffi::{CStr, CString};
use std::ptr::{self};
use texture::TextureConstructor;
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
static mut CAMERA: cpp_camera = cpp_camera { camera: None };

use asset_management::cube::VERTICES;

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
            shader::ShaderConstructor("shaders/basic_lighting.vert", "shaders/basic_lighting.frag");
        let lightCubeShader: Shader =
            shader::ShaderConstructor("shaders/light_cube.vert", "shaders/light_cube.frag");

        let mut cube_VAO: u32 = 0;
        gl::GenVertexArrays(1, &mut cube_VAO);
        let mut light_cube_VAO: u32 = 0;
        gl::GenVertexArrays(1, &mut light_cube_VAO);
        let mut VBO: u32 = 0;
        gl::GenBuffers(1, &mut VBO);

        gl::BindVertexArray(cube_VAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
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
            8 * sizeof!(f32),
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * sizeof!(f32),
            (3 * sizeof!(f32)) as *const _,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            8 * sizeof!(f32),
            (6 * sizeof!(f32)) as *const _,
        );
        gl::EnableVertexAttribArray(2);

        gl::BindVertexArray(light_cube_VAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * sizeof!(f32),
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

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

        let m: shader::material::MaterialTexture = shader::material::MaterialTexture {
            diffuse: TextureConstructor(
                "textures/container.png".to_string(),
                gl::RGBA,
                true,
                None,
                None,
                Some(gl::NEAREST),
                Some(gl::NEAREST),
                "diffuse".to_string(),
            ),
            specular: TextureConstructor(
                "textures/container_specular.png".to_string(),
                gl::RGBA,
                true,
                None,
                None,
                Some(gl::NEAREST),
                Some(gl::NEAREST),
                "specular".to_string(),
            ),
            shininess: 32f32,
        };

        let cubePositions: [glm::TVec3<f32>; 10] = [
            glm::vec3(0.0f32, 0.0f32, 0.0f32),
            glm::vec3(2.0f32, 5.0f32, -15.0f32),
            glm::vec3(-1.5f32, -2.2f32, -2.5f32),
            glm::vec3(-3.8f32, -2.0f32, -12.3f32),
            glm::vec3(2.4f32, -0.4f32, -3.5f32),
            glm::vec3(-1.7f32, 3.0f32, -7.5f32),
            glm::vec3(1.3f32, -2.0f32, -2.5f32),
            glm::vec3(1.5f32, 2.0f32, -2.5f32),
            glm::vec3(1.5f32, 0.2f32, -1.5f32),
            glm::vec3(-1.3f32, 1.0f32, -1.5f32),
        ];

        shader.activate();
        shader.setInt("material.diffuse", 0);
        shader.setInt("material.specular", 1);
        shader.setFloat("material.shininess", m.shininess);

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
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, m.diffuse.get_texture());
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, m.specular.get_texture());

            gl::BindVertexArray(cube_VAO);
            for i in 0..10 {
                let mut model = glm::Mat4::identity();
                model = glm::translate(&model, &cubePositions[i]);
                let angle: f32 = 20f32 * i as f32;
                model = glm::rotate(&model, angle.to_radians(), &glm::vec3(1f32, 0.3f32, 0.5f32));
                shader.setMat4("model", model, gl::FALSE);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            lightCubeShader.activate();
            lightCubeShader.setMat4("view", CAMERA.get_view_matrix(), gl::FALSE);
            lightCubeShader.setMat4("projection", CAMERA.get_projection_matrix(), gl::FALSE);

            gl::BindVertexArray(light_cube_VAO);
            for i in 0..3 {
                let mut model = glm::Mat4::identity();
                model = glm::translate(&model, &lightPositions[i]);
                model = glm::scale(&model, &glm::vec3(0.2f32, 0.2, 0.2));
                lightCubeShader.setMat4("model", model, gl::FALSE);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            glfwSwapBuffers(window);
            glfwPollEvents();
        }

        gl::DeleteVertexArrays(1, &mut cube_VAO);
        gl::DeleteBuffers(1, &mut VBO);
        gl::DeleteProgram(shader.getId());

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
