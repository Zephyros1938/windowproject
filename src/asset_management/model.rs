extern crate nalgebra_glm as glm;

use crate::texture::TextureConstructor;
use crate::{shader::Shader, texture::Texture};

use super::get_asset_path;
use super::mesh::{Mesh, Vertex};
use log::debug;
use russimp::material::Material as AIMaterial;
use russimp::material::TextureType as AITextureType;
use russimp::mesh::Mesh as AIMesh;
use russimp::node::Node;
use russimp::scene::PostProcess as AIProcess;
use russimp::scene::Scene as AIScene;

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub directory: String,
    pub textures_loaded: Vec<Texture>,
}

impl Model {
    pub fn new(path: &str) -> Self {
        let mut result = Self {
            textures_loaded: Vec::new(),
            meshes: Vec::new(),
            directory: String::new(),
        };
        result.load_model(path);
        result
    }
    pub fn draw(&self, shader: &Shader) {
        for mesh in self.meshes.iter() {
            mesh.draw(shader);
        }
    }

    fn load_model(&mut self, path: &str) {
        debug!("Loading Model: {}", path);

        let scene = AIScene::from_file(
            get_asset_path(path).unwrap().as_str(),
            vec![AIProcess::Triangulate, AIProcess::FlipUVs],
        )
        .unwrap();
        let full_path = get_asset_path(path).unwrap();
        self.directory = std::path::Path::new(&full_path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        if let Some(root) = &scene.root {
            self.process_node(root, &scene);
        }
        debug!("Model Loaded: {}", path);
    }
    fn process_node(&mut self, node: &Node, scene: &AIScene) {
        for &mesh_i in node.meshes.iter() {
            let mesh_i = mesh_i as usize;

            let mesh = &scene.meshes[mesh_i];
            let result = self.process_mesh(mesh, scene);
            self.meshes.push(result);
        }

        for child in node.children.borrow().iter() {
            self.process_node(child, scene);
        }
    }
    fn process_mesh(&mut self, mesh: &AIMesh, scene: &AIScene) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut textures: Vec<Texture> = Vec::new();

        for (i, vertice) in mesh.vertices.iter().enumerate() {
            let mut vertex = Vertex::default();
            let mut vector: glm::Vec3 = glm::vec3(0f32, 0.0, 0.0);
            vector.x = vertice.x;
            vector.y = vertice.y;
            vector.z = vertice.z;
            vertex.position = vector.clone();

            if mesh.normals.len() > 0 {
                vector.x = mesh.normals[i].x;
                vector.y = mesh.normals[i].y;
                vector.z = mesh.normals[i].z;
                vertex.normal = vector.clone();
            }

            if mesh.texture_coords.len() > 0 {
                let mut vec = glm::vec2(0f32, 0.0);
                vec.x = mesh.texture_coords[0].clone().unwrap()[i].x;
                vec.y = mesh.texture_coords[0].clone().unwrap()[i].y;
                vertex.texcoords = vec.clone();
            } else {
                vertex.texcoords = glm::vec2(0f32, 0.0);
            }
            vertices.push(vertex);
        }

        for face in mesh.faces.iter() {
            for &index in face.0.iter() {
                indices.push(index);
            }
        }

        let material = &scene.materials[mesh.material_index as usize];
        let diffuse_maps = self.load_material_textures(
            material,
            AITextureType::Diffuse,
            "texture_diffuse".to_string(),
        );
        diffuse_maps.iter().for_each(|it| {
            textures.push(it.clone());
        });
        let specular_maps = self.load_material_textures(
            material,
            AITextureType::Specular,
            "texture_specular".to_string(),
        );
        specular_maps.iter().for_each(|it| {
            textures.push(it.clone());
        });

        Mesh::new(vertices, indices, textures)
    }
    fn load_material_textures(
        &mut self,
        mat: &AIMaterial,
        t_type: AITextureType,
        typename: String,
    ) -> Vec<Texture> {
        let mut textures: Vec<Texture> = Vec::new();

        for prop in &mat.properties {
            if prop.key != "$tex.file" || prop.semantic != t_type {
                continue;
            }
            let mut skip = false;
            let full_path = format!(
                "{}/{}",
                self.directory,
                format!("{:?}", prop.data)
                    .replace("String(\"", "")
                    .replace("\")", "")
            );
            for texture_loaded in &self.textures_loaded {
                if texture_loaded.path == full_path {
                    textures.push(texture_loaded.clone());
                    skip = true;
                    break;
                }
            }

            if !skip {
                debug!("Trying to load texture at: {}", full_path);

                let mut texture_load = unsafe {
                    TextureConstructor(
                        full_path.clone(),
                        gl::RGBA,
                        true,
                        None,
                        None,
                        Some(gl::NEAREST_MIPMAP_NEAREST),
                        Some(gl::NEAREST),
                        typename.clone(),
                    )
                };
                texture_load.path = full_path.clone();

                textures.push(texture_load.clone());
                self.textures_loaded.push(texture_load);
            }
        }

        textures
    }
}
