use log::debug;

pub struct VertexAttrib {
    pub index: u32,
    pub size: i32,
    pub type_: u32,
    pub normalized: bool,
    pub offset: usize,
}

impl VertexAttrib {
    pub fn enable(&self, stride: i32) {
        unsafe {
            gl::VertexAttribPointer(
                self.index,
                self.size,
                self.type_,
                if self.normalized { gl::TRUE } else { gl::FALSE },
                stride,
                match self.offset {
                    0 => std::ptr::null(),
                    _ => self.offset as *const _,
                },
            );
            gl::EnableVertexAttribArray(self.index);
            debug!(
                "Enabled Vertex Attrib {}:\n\r\tSIZE: {}\n\r\tTYPE: {}\n\r\tNORMALIZED: {}\n\r\tSTRIDE: {}\n\r\tOFFSET: {}",
                self.index, self.size, self.type_, self.normalized, stride, self.offset
            )
        }
    }
}
