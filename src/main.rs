use gl::{ARRAY_BUFFER, STATIC_DRAW};
use glfw::ffi::*;
use std::ffi::{CStr, CString};
use std::ptr;
mod util;
use log::{debug, error};
use util::*;
mod macros;
use macros::*;
// https://learnopengl.com/Getting-started/Hello-Triangle

// **constant shader test values**
const VERTICES: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

#[allow(non_camel_case_types, non_snake_case)]
fn main() -> LinuxExitCode {
    unsafe {
        if glfw::ffi::glfwInit() == 0 {
            panic!("GLFW failed to initialize!");
        }
        log4rs::init_file("log4rs.yml", Default::default()).unwrap();
        debug!("log4rs configured!");

        glfwWindowHint(CONTEXT_VERSION_MAJOR as i32, 3);
        glfwWindowHint(CONTEXT_VERSION_MINOR as i32, 3);
        glfwWindowHint(OPENGL_PROFILE as i32, OPENGL_CORE_PROFILE as i32);

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
        if window.is_null() {
            glfwTerminate();
            eprintln!("Failed to create GLFW window!");
            return LinuxExitCode::ERR(1);
        }

        glfwMakeContextCurrent(window);

        gl::load_with(|name| {
            let cname = CString::new(name).unwrap();
            glfwGetProcAddress(cname.as_ptr()) as *const _
        });
        gl::Viewport(0, 0, 800, 600);

        glfwSetFramebufferSizeCallback(window, Some(framebuffer_size_callback));

        let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
            .to_str()
            .unwrap();
        debug!("Using OpenGL version: {}", version);
        let window_title = CString::new(format!("{:#?} - {}", title, version)).unwrap();
        glfw::ffi::glfwSetWindowTitle(window, window_title.as_ptr());

        fn process_input(window: *mut GLFWwindow) {
            if unsafe { glfwGetKey(window, KEY_ESCAPE) } == PRESS {
                unsafe { glfwSetWindowShouldClose(window, TRUE) };
            }
        }

        // SHADER CREATION

        let mut vbo: u32 = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(ARRAY_BUFFER, vbo);
        gl::BufferData(
            ARRAY_BUFFER,
            sizeof_val!(VERTICES).try_into().unwrap(),
            as_c_void!(VERTICES),
            STATIC_DRAW,
        );

        // Vertex Shader Creation

        let vertexShaderSource = cstr!(
            "
            #version 330 core
            layout (location = 0) in vec3 aPos;

            void main()
            {
                gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
            }"
        );
        let vertexShader: u32 = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(
            vertexShader,
            1,
            cstr_to_ptr_array!(vertexShaderSource),
            std::ptr::null(),
        );
        gl::CompileShader(vertexShader);
        check_shader_compile!(vertexShader);

        // Fragment Shader Creation

        let fragmentShaderSource = cstr!(
            "#version 330 core
        out vec4 FragColor;

        void main()
        {
            FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
        } "
        );
        let fragmentShader: u32 = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(
            fragmentShader,
            1,
            cstr_to_ptr_array!(fragmentShaderSource),
            std::ptr::null(),
        );
        gl::CompileShader(fragmentShader);
        check_shader_compile!(fragmentShader);

        // Shader Program Creation

        let shaderProgram: u32 = gl::CreateProgram();
        gl::AttachShader(shaderProgram, vertexShader);
        gl::AttachShader(shaderProgram, fragmentShader);
        gl::LinkProgram(shaderProgram);
        check_program_link!(shaderProgram);
        gl::UseProgram(shaderProgram);
        gl::DeleteShader(vertexShader);
        gl::DeleteShader(fragmentShader);

        // Vertex Array Object Creation

        let mut vao: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * sizeof!(f32) as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        while glfwWindowShouldClose(window) == 0 {
            process_input(window);

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shaderProgram);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            glfwPollEvents();
            glfwSwapBuffers(window);
        }

        glfwDestroyWindow(window);
        glfwTerminate();
    }
    LinuxExitCode::OK
}

pub enum LinuxExitCode {
    OK,
    ERR(u8),
}

impl std::process::Termination for LinuxExitCode {
    fn report(self) -> std::process::ExitCode {
        match self {
            LinuxExitCode::OK => std::process::ExitCode::SUCCESS,
            LinuxExitCode::ERR(v) => std::process::ExitCode::from(v),
        }
    }
}
