pub struct BasicLight {
    pub position: nalgebra_glm::TVec3<f32>,
    pub ambient: nalgebra_glm::TVec3<f32>,
    pub diffuse: nalgebra_glm::TVec3<f32>,
    pub specular: nalgebra_glm::TVec3<f32>,
}

pub struct DirectionalLight {
    pub direction: nalgebra_glm::TVec3<f32>,
    pub ambient: nalgebra_glm::TVec3<f32>,
    pub diffuse: nalgebra_glm::TVec3<f32>,
    pub specular: nalgebra_glm::TVec3<f32>,
}

pub struct PointLight {
    pub position: nalgebra_glm::TVec3<f32>,
    pub ambient: nalgebra_glm::TVec3<f32>,
    pub diffuse: nalgebra_glm::TVec3<f32>,
    pub specular: nalgebra_glm::TVec3<f32>,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

pub struct SpotLight {
    pub position: nalgebra_glm::TVec3<f32>,
    pub direction: nalgebra_glm::TVec3<f32>,

    pub cutOff: f32,

    pub ambient: nalgebra_glm::TVec3<f32>,
    pub diffuse: nalgebra_glm::TVec3<f32>,
    pub specular: nalgebra_glm::TVec3<f32>,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}
