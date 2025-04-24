use crate::texture::Texture;

pub struct Material {
    pub ambient: nalgebra_glm::Vec3,
    pub diffuse: nalgebra_glm::Vec3,
    pub specular: nalgebra_glm::Vec3,
}
pub struct MaterialTexture {
    pub diffuse: Texture,
    pub specular: Texture,
    pub shininess: f32,
}
