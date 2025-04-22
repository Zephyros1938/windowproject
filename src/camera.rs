#![allow(dead_code)]
use nalgebra_glm as glm;

use crate::as_mut_expect;

const MAX_ZOOM: f32 = 89f32;
const MIN_ZOOM: f32 = 1f32;

const YAW: f32 = -90f32;
const PITCH: f32 = 0f32;
const SPEED: f32 = 2.5f32;
const SENSITIVITY: f32 = 0.1f32;
const ZOOM: f32 = 45f32;

pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

impl CameraMovement {
    pub fn val(&self) -> i32 {
        match self {
            CameraMovement::FORWARD => 0,
            CameraMovement::BACKWARD => 1,
            CameraMovement::LEFT => 2,
            CameraMovement::RIGHT => 3,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    position: glm::Vec3,
    front: glm::Vec3,
    up: glm::Vec3,
    right: glm::Vec3,
    world_up: glm::Vec3,
    yaw: f32,
    pitch: f32,
    mouse_sensitivity: f32,
    move_speed: f32,
    pub zoom: f32,
    constrain_pitch: bool,
    aspect_ratio: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn get_projection_matrix(&self) -> glm::Mat4 {
        glm::perspective(
            self.aspect_ratio,
            self.zoom.to_radians(),
            self.near,
            self.far,
        )
    }

    pub fn get_position(&self) -> glm::Vec3 {
        self.position
    }

    pub fn get_front(&self) -> glm::Vec3 {
        self.front
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, deltatime: f64) {
        let vel: f32 = self.move_speed * (deltatime as f32);
        if direction.val() == 0 {
            self.position += self.front * vel;
        }
        if direction.val() == 1 {
            self.position -= self.front * vel;
        }
        if direction.val() == 2 {
            self.position -= self.right * vel;
        }
        if direction.val() == 3 {
            self.position += self.right * vel;
        }
    }

    pub fn process_mouse(&mut self, mut xoffset: f32, mut yoffset: f32) {
        xoffset *= self.mouse_sensitivity;
        yoffset *= self.mouse_sensitivity;

        self.yaw += xoffset;
        self.pitch += yoffset;
        if self.constrain_pitch {
            if self.pitch > 89f32 {
                self.pitch = 89f32;
            }
            if self.pitch < -89f32 {
                self.pitch = -89f32;
            }
        }
        self.update_vectors();
    }

    pub fn process_scroll(&mut self, yoffset: f32) {
        self.zoom -= yoffset;
        if self.zoom < MIN_ZOOM {
            self.zoom = MIN_ZOOM;
        }
        if self.zoom > MAX_ZOOM {
            self.zoom = MAX_ZOOM;
        }
        println!("{}", self.zoom);
    }

    fn update_vectors(&mut self) {
        let front = glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = front.normalize();
        self.right = glm::cross(&self.front, &self.world_up).normalize();
        self.up = glm::cross(&self.right, &self.front).normalize();
    }

    pub fn set_aspect_ratio_xy(&mut self, x: f32, y: f32) {
        self.aspect_ratio = x / y;
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect_ratio = aspect;
    }
}

pub fn CameraConstructor(
    position: Option<glm::Vec3>,
    world_up: Option<glm::Vec3>,
    yaw: Option<f32>,
    pitch: Option<f32>,
    mouse_sensitivity: Option<f32>,
    move_speed: Option<f32>,
    zoom: Option<f32>,
    constrain_pitch: Option<bool>,
    aspect_ratio: Option<f32>,
    near: Option<f32>,
    far: Option<f32>,
) -> Camera {
    let yaw = match yaw {
        Some(n) => n,
        None => YAW,
    };
    let pitch = match pitch {
        Some(n) => n,
        None => PITCH,
    };
    let world_up = match world_up {
        Some(n) => n,
        None => glm::vec3(0f32, 1f32, 0f32),
    };
    let mut front = glm::vec3(0f32, 0f32, 0f32);
    front.x = (yaw.to_radians() * pitch.to_radians().cos()).cos();
    front.y = pitch.to_radians().sin();
    front.z = (yaw.to_radians() * pitch.to_radians().cos()).sin();
    front = front.normalize();
    let right = glm::cross(&front, &world_up);
    let up = glm::cross(&right, &front).normalize();
    Camera {
        position: match position {
            Some(n) => n,
            None => glm::vec3(0f32, 0f32, 0f32),
        },
        front,
        up,
        right,
        world_up,
        yaw,
        pitch,
        mouse_sensitivity: match mouse_sensitivity {
            Some(n) => n,
            None => SENSITIVITY,
        },
        move_speed: match move_speed {
            Some(n) => n,
            None => SPEED,
        },
        zoom: match zoom {
            Some(n) => n,
            None => ZOOM,
        },
        constrain_pitch: match constrain_pitch {
            Some(n) => n,
            None => true,
        },
        aspect_ratio: match aspect_ratio {
            Some(n) => n,
            None => 800f32 / 600f32,
        },
        near: match near {
            Some(n) => n,
            None => 0.01f32,
        },
        far: match far {
            Some(n) => n,
            None => 100f32,
        },
    }
}

#[derive(Debug)]
pub struct cpp_camera {
    pub camera: Option<Camera>,
}

impl cpp_camera {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera: Some(camera),
        }
    }

    pub fn get_view_matrix(&self) -> glm::Mat4 {
        self.camera
            .expect("Camera not initialized")
            .get_view_matrix()
    }

    pub fn get_projection_matrix(&self) -> glm::Mat4 {
        self.camera
            .expect("Camera not initialized")
            .get_projection_matrix()
    }

    pub fn get_position(&self) -> glm::Vec3 {
        self.camera.expect("Camera not initialized").get_position()
    }

    pub fn get_front(&self) -> glm::Vec3 {
        self.camera.expect("Camera not initialized").get_front()
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, deltatime: f64) {
        as_mut_expect!(self.camera, "Camera not initialized")
            .process_keyboard(direction, deltatime);
    }

    pub fn process_mouse(&mut self, xoffset: f32, yoffset: f32) {
        as_mut_expect!(self.camera, "Camera not initialized").process_mouse(xoffset, yoffset);
    }

    pub fn process_scroll(&mut self, yoffset: f32) {
        as_mut_expect!(self.camera, "Camera not initialized").process_scroll(yoffset);
    }

    fn update_vectors(&mut self) {
        as_mut_expect!(self.camera, "Camera not initialized").update_vectors();
    }

    pub fn set_aspect_ratio_xy(&mut self, x: f32, y: f32) {
        as_mut_expect!(self.camera, "Camera not initialized").set_aspect_ratio(x / y);
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        as_mut_expect!(self.camera, "Camera not initialized").set_aspect_ratio(aspect);
    }
}
