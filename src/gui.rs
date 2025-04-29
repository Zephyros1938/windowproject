mod font;
pub use font::Font;

use gl;
use glfw::ffi::{self, GLFWwindow};
use log::debug;
use nalgebra_glm as glm;

pub struct GUI {}

impl GUI {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct TextLabel {
    bound_gui: GUI,
    font: Font,
}

impl TextLabel {
    pub fn new(gui: GUI, font: Option<Font>) -> Self {
        Self {
            bound_gui: gui,
            font: match font {
                Some(n) => n,
                None => Font::default(),
            },
        }
    }
}
