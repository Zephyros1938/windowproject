use std::ffi::c_void;

use log::error;

pub mod glmaddon;

pub fn init_logging() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    log::debug!("log4rs configured!");
}

pub extern "system" fn glfw_error_callback(err: glfw::Error, description: String) {
    error!("GLFW error {:?}: {:?}", err, description);
}

pub extern "system" fn debug_callback(
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: i32,
    f: *const i8,
    g: *mut c_void,
) {
    panic!("{} {} {} {} {} {:#?} {:?}", a, b, c, d, e, f, g)
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
