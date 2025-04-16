use glfw::ffi::*;

pub extern "C" fn framebuffer_size_callback(_window: *mut GLFWwindow, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}

pub fn init_log4rs() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    log::debug!("log4rs configured!");
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
