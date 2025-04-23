use crate::name_struct;

use super::Shader;

pub trait LightImpl {
    fn set_uniform(&self, _shader: &Shader, _light_uniform_basename: &str) {
        todo!(
            "{} Did Not Implement LightImpl::set_uniform",
            name_struct!(self)
        )
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BasicLight {
    pub position: nalgebra_glm::TVec3<f32>,

    pub ambient: nalgebra_glm::TVec3<f32>,
    pub diffuse: nalgebra_glm::TVec3<f32>,
    pub specular: nalgebra_glm::TVec3<f32>,
}
#[derive(Debug, Clone, Copy)]
pub struct DirectionalLight {
    pub direction: nalgebra_glm::TVec3<f32>,

    pub ambient: nalgebra_glm::TVec3<f32>,
    pub diffuse: nalgebra_glm::TVec3<f32>,
    pub specular: nalgebra_glm::TVec3<f32>,
}
#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub position: nalgebra_glm::TVec3<f32>,

    pub ambient: nalgebra_glm::TVec3<f32>,
    pub diffuse: nalgebra_glm::TVec3<f32>,
    pub specular: nalgebra_glm::TVec3<f32>,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}
#[derive(Debug, Clone, Copy)]
pub struct SpotLight {
    pub position: nalgebra_glm::TVec3<f32>,
    pub direction: nalgebra_glm::TVec3<f32>,

    pub ambient: nalgebra_glm::TVec3<f32>,
    pub diffuse: nalgebra_glm::TVec3<f32>,
    pub specular: nalgebra_glm::TVec3<f32>,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    pub cutOff: f32,
    pub outerCutOff: f32,
}

impl LightImpl for BasicLight {
    fn set_uniform(&self, shader: &Shader, light_uniform_basename: &str) {
        unsafe {
            let prefix = light_uniform_basename;
            macro_rules! set_vec3 {
                ($field:ident) => {
                    shader.setVec3(&format!("{}.{}", prefix, stringify!($field)), self.$field);
                };
            }
            set_vec3!(position);

            set_vec3!(ambient);
            set_vec3!(diffuse);
            set_vec3!(specular);
        }
    }
}

impl LightImpl for DirectionalLight {
    fn set_uniform(&self, shader: &Shader, light_uniform_basename: &str) {
        unsafe {
            let prefix = light_uniform_basename;
            macro_rules! set_vec3 {
                ($field:ident) => {
                    shader.setVec3(&format!("{}.{}", prefix, stringify!($field)), self.$field);
                };
            }
            set_vec3!(direction);

            set_vec3!(ambient);
            set_vec3!(diffuse);
            set_vec3!(specular);
        }
    }
}

impl LightImpl for PointLight {
    fn set_uniform(&self, shader: &Shader, light_uniform_basename: &str) {
        unsafe {
            let prefix = light_uniform_basename;
            macro_rules! set_vec3 {
                ($field:ident) => {
                    shader.setVec3(&format!("{}.{}", prefix, stringify!($field)), self.$field);
                };
            }
            macro_rules! set_float {
                ($field:ident) => {
                    shader.setFloat(&format!("{}.{}", prefix, stringify!($field)), self.$field);
                };
            }
            set_vec3!(position);
            set_vec3!(ambient);
            set_vec3!(diffuse);
            set_vec3!(specular);

            set_float!(constant);
            set_float!(linear);
            set_float!(quadratic);
        }
    }
}

impl LightImpl for SpotLight {
    fn set_uniform(&self, shader: &Shader, light_uniform_basename: &str) {
        unsafe {
            let prefix = light_uniform_basename;
            macro_rules! set_vec3 {
                ($field:ident) => {
                    shader.setVec3(&format!("{}.{}", prefix, stringify!($field)), self.$field);
                };
            }
            macro_rules! set_float {
                ($field:ident) => {
                    shader.setFloat(&format!("{}.{}", prefix, stringify!($field)), self.$field);
                };
            }
            set_vec3!(position);
            set_vec3!(direction);

            set_vec3!(ambient);
            set_vec3!(diffuse);
            set_vec3!(specular);

            set_float!(constant);
            set_float!(linear);
            set_float!(quadratic);

            set_float!(cutOff);
            set_float!(outerCutOff);
        }
    }
}

pub struct LightCollection<T: LightImpl, const N: usize> {
    pub lights: [T; N],
}

impl<T: LightImpl, const N: usize> LightCollection<T, N> {
    pub fn set_uniform(&self, shader: &Shader, light_uniform_basename: &str) {
        for i in 0..N {
            self.lights[i].set_uniform(
                shader,
                format!("{}[{}]", light_uniform_basename, i).as_str(),
            );
        }
    }
}
