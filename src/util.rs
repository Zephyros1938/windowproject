use log::error;

pub mod glmaddon;

pub fn init_logging() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    log::debug!("log4rs configured!");
}

pub fn glfw_error_callback(err: glfw::Error, description: String) {
    error!("GLFW error {:?}: {:?}", err, description);
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
