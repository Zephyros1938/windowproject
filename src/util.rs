use glfw::ffi::GLFWwindow;

pub extern "C" fn framebuffer_size_callback(_window: *mut GLFWwindow, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}
