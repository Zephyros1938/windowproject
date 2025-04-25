use gl;
use nalgebra_glm as glm;

use crate::{shader::Shader, texture::Texture};

#[repr(C)]
#[derive(Clone)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub texcoords: glm::Vec2,
    pub tangent: glm::Vec3,
    pub bit_tangent: glm::Vec3,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: glm::vec3(0f32, 0.0, 0.0),
            normal: glm::vec3(0f32, 0.0, 0.0),
            texcoords: glm::vec2(0f32, 0.0),
            tangent: glm::vec3(0f32, 0.0, 0.0),
            bit_tangent: glm::vec3(0f32, 0.0, 0.0),
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    VAO: u32,
    VBO: u32,
    EBO: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        let mut m = Mesh {
            vertices,
            indices,
            textures,
            VAO: 0,
            VBO: 0,
            EBO: 0,
        };
        m.setup_mesh();
        m
    }
    fn setup_mesh(&mut self) {
        unsafe {
            use crate::{as_c_void, sizeof};
            use gl::*;
            GenVertexArrays(1, &mut self.VAO);
            GenBuffers(1, &mut self.VBO);
            GenBuffers(1, &mut self.EBO);

            BindVertexArray(self.VAO);

            BindBuffer(ARRAY_BUFFER, self.VBO);
            BufferData(
                ARRAY_BUFFER,
                self.vertices.len() as isize * sizeof!(Vertex) as isize,
                as_c_void!(self.vertices),
                STATIC_DRAW,
            );

            BindBuffer(ELEMENT_ARRAY_BUFFER, self.EBO);
            BufferData(
                ELEMENT_ARRAY_BUFFER,
                self.indices.len() as isize * sizeof!(u32) as isize,
                as_c_void!(self.indices),
                STATIC_DRAW,
            );
            VertexAttribPointer(0, 3, FLOAT, FALSE, sizeof!(Vertex), std::ptr::null());
            EnableVertexAttribArray(0);
            VertexAttribPointer(
                1,
                3,
                FLOAT,
                FALSE,
                sizeof!(Vertex),
                (3 * sizeof!(f32)) as *const _,
            );
            EnableVertexAttribArray(1);
            VertexAttribPointer(
                2,
                2,
                FLOAT,
                FALSE,
                sizeof!(Vertex),
                (6 * sizeof!(f32)) as *const _,
            );
            EnableVertexAttribArray(2);
            VertexAttribPointer(
                3,
                3,
                FLOAT,
                FALSE,
                sizeof!(Vertex),
                (8 * sizeof!(f32)) as *const _,
            );
            EnableVertexAttribArray(3);
            VertexAttribPointer(
                4,
                3,
                FLOAT,
                FALSE,
                sizeof!(Vertex),
                (11 * sizeof!(f32)) as *const _,
            );
            EnableVertexAttribArray(4);
            BindVertexArray(0);
        }
    }
    pub fn draw(&self, shader: &Shader) {
        unsafe {
            use gl::*;
            use std::ffi::CString;
            let mut diffuseNr = 1u32;
            let mut specularNr = 1u32;
            let mut normalNr = 1u32;
            let mut heightNr = 1u32;
            // debug!("Drawing mesh!");
            for i in 0..self.textures.len() {
                ActiveTexture(TEXTURE0 + i as u32);
                // debug!("Activated Texture {}", TEXTURE0 + i as u32);
                let number: CString;
                let name = &self.textures[i].type_s;
                if name == "texture_diffuse" {
                    number = CString::new(diffuseNr.to_string()).unwrap();
                    diffuseNr += 1;
                } else if name == "texture_specular" {
                    number = CString::new(specularNr.to_string()).unwrap();
                    specularNr += 1;
                } else if name == "texture_normal" {
                    number = CString::new(normalNr.to_string()).unwrap();
                    normalNr += 1;
                } else if name == "texture_height" {
                    number = CString::new(heightNr.to_string()).unwrap();
                    heightNr += 1;
                } else {
                    panic!("Could not get texture!");
                }
                shader.setInt(
                    format!("material.{}{}", name, number.to_str().unwrap()).as_str(),
                    i as i32,
                );
                BindTexture(TEXTURE_2D, self.textures[i].ID);
            }

            BindVertexArray(self.VAO);
            DrawElements(
                TRIANGLES,
                self.indices.len() as i32,
                UNSIGNED_INT,
                0 as *const _,
            );
            BindVertexArray(0);
            ActiveTexture(TEXTURE0);
        }
    }
}
