use crate::{as_c_void, shader::vertexattrib::VertexAttrib, sizeof, sizeof_val};

pub struct Object {
    vertices: &'static [f32],
    data_stride: i32,
    attribs: Vec<VertexAttrib>,
}

impl Object {
    pub fn upload_to_gpu(&self) -> (u32, u32) {
        let mut vbo = 0;
        let mut vao = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                sizeof_val!(self.vertices).try_into().unwrap(),
                as_c_void!(self.vertices),
                gl::STATIC_DRAW,
            );

            for attrib in &self.attribs {
                attrib.enable(self.data_stride);
            }

            gl::BindVertexArray(0);
        }

        (vao, vbo)
    }
}

pub struct ObjectBuilder {
    vertices: Option<&'static [f32]>,
    data_stride: Option<i32>,
    attribs: Vec<VertexAttrib>,
}

impl ObjectBuilder {
    pub fn new() -> Self {
        Self {
            vertices: None,
            data_stride: None,
            attribs: Vec::new(),
        }
    }

    pub fn vertices(mut self, verts: &'static [f32]) -> Self {
        self.vertices = Some(verts);
        self
    }

    pub fn data_stride(mut self, elements: i32) -> Self {
        self.data_stride = Some(elements);
        self
    }

    pub fn add_attrib(mut self, attrib: VertexAttrib) -> Self {
        self.attribs.push(attrib);
        self
    }

    pub fn build(self) -> Object {
        Object {
            vertices: self.vertices.expect("Vertices must be provided"),
            data_stride: self.data_stride.expect("Stride must be provided"),
            attribs: self.attribs,
        }
    }
}
