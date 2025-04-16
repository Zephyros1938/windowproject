#[allow(unused_macros)]
#[macro_export]
macro_rules! sizeof {
    ($t:ty) => {
        ::std::mem::size_of::<$t>() as i32
    };
}
#[macro_export]
macro_rules! sizeof_val {
    ($val:expr) => {
        ::std::mem::size_of_val(&$val)
    };
}
#[macro_export]
macro_rules! as_c_void {
    ($val:expr) => {
        $val.as_ptr() as *const std::os::raw::c_void
    };
}
#[macro_export]
macro_rules! cstr {
    ($s:expr) => {{
        let s: &str = $s.as_ref();
        std::ffi::CString::new(s).expect("CString cannot contain interior null bytes")
    }};
}

#[macro_export]
macro_rules! cstr_compiletime {
    ($s:literal) => {{
        const C_STRING: &::std::ffi::CStr = unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($s, "\0").as_bytes())
        };
        C_STRING
    }};
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! cstr_ptr {
    ($s:expr) => {{
        let c_string = crate::cstr!($s);
        let ptr = c_string.as_ptr();
        (c_string, ptr)
    }};
}
#[macro_export]
macro_rules! cstr_to_ptr_array {
    ($cstr:expr) => {{
        let ptr = $cstr.as_ptr();
        let arr = [ptr];
        arr.as_ptr()
    }};
}
#[macro_export]
macro_rules! check_shader_compile {
    ($shader:expr) => {{
        use gl::types::{GLchar, GLint};
        use std::ffi::CStr;
        use std::ptr;

        let mut success: GLint = 0;

        gl::GetShaderiv($shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut log_length: GLint = 0;

            gl::GetShaderiv($shader, gl::INFO_LOG_LENGTH, &mut log_length);

            if log_length > 0 {
                let mut buffer = Vec::with_capacity(log_length as usize);
                buffer.extend(std::iter::repeat(b' ' as i8).take(log_length as usize));
                let error_ptr = buffer.as_mut_ptr() as *mut GLchar;

                gl::GetShaderInfoLog($shader, log_length, ptr::null_mut(), error_ptr);

                let c_str = CStr::from_ptr(error_ptr);
                log::error!("Shader compilation failed:\n{}", c_str.to_string_lossy());
            } else {
                log::error!("Shader compilation failed:\n{}", "Unknown Error");
            }
        } else {
            log::debug!("Shader compiled successfully");
        }
    }};
}
#[macro_export]
macro_rules! check_program_link {
    ($program:expr) => {{
        use gl::types::{GLchar, GLint};
        use std::ffi::CStr;
        use std::ptr;

        let mut success: GLint = 0;

        gl::GetProgramiv($program, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut log_length: GLint = 0;

            gl::GetProgramiv($program, gl::INFO_LOG_LENGTH, &mut log_length);

            let mut buffer: Vec<i8> = Vec::with_capacity(log_length as usize);
            buffer.extend(std::iter::repeat(b' ' as i8).take(log_length as usize));
            let error_ptr = buffer.as_mut_ptr() as *mut GLchar;

            gl::GetProgramInfoLog($program, log_length, ptr::null_mut(), error_ptr);

            let c_str = CStr::from_ptr(error_ptr);
            log::error!("Program linking failed:\n{}", c_str.to_string_lossy());
        } else {
            log::debug!("Program linked successfully");
        }
    }};
}
