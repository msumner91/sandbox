#![allow(non_snake_case)]
use cgmath;
use cgmath::prelude::*;
use cgmath::vec3;

use crate::types::*;

const YAW: f32 = 45.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 80.0;
const SENSITIVTY: f32 = 0.1;
const ZOOM: f32 = 45.0;

pub enum CameraMovement {
  FORWARD,
  BACKWARD,
  LEFT,
  RIGHT,
  UP,
  DOWN
}

pub struct Camera {
  pub position: Point3,
  pub front: Vector3,
  pub up: Vector3,
  pub right: Vector3,
  pub worldUp: Vector3,
  pub yaw: f32,
  pub pitch: f32,
  pub movementSpeed: f32,
  pub mouseSensitivity: f32,
  pub zoom: f32,
}

impl Default for Camera {
  fn default() -> Camera {
    let mut camera = Camera {
      position: Point3::new(0.0, 0.0, 0.0),
      front: vec3(0.0, 0.0, -1.0),
      up: Vector3::zero(), 
      right: Vector3::zero(),
      worldUp: Vector3::unit_y(),
      yaw: YAW,
      pitch: PITCH,
      movementSpeed: SPEED,
      mouseSensitivity: SENSITIVTY,
      zoom: ZOOM,
    };
    camera.updateCameraVectors();
    camera
  }
}

use CameraMovement::*;
impl Camera {
  pub fn getViewMatrix(&self) -> Matrix4 {
    Matrix4::look_at(self.position, self.position + self.front, self.up)
  }

  pub fn processKeyboard(&mut self, direction: CameraMovement, deltaTime: f32) {
    let velocity = self.movementSpeed * deltaTime;
    match direction {
      FORWARD => self.position += self.front * velocity,
      BACKWARD => self.position += -(self.front * velocity),
      LEFT => self.position += -(self.right * velocity),
      RIGHT => self.position += self.right * velocity,
      UP => self.position += self.up * velocity,
      DOWN => self.position -= self.up * velocity
    }
  }

  pub fn processMouseMovement(&mut self, mut xoffset: f32, mut yoffset: f32, constrainPitch: bool) {
    xoffset *= self.mouseSensitivity;
    yoffset *= self.mouseSensitivity;
    self.yaw += xoffset;
    self.pitch += yoffset;

    if constrainPitch {
      if self.pitch > 89.0 {
        self.pitch = 89.0;
      }
      if self.pitch < -89.0 {
        self.pitch = -89.0;
      }
    }

    self.updateCameraVectors();
  }

  pub fn processMouseScroll(&mut self, yoffset: f32) {
    if self.zoom >= 1.0 && self.zoom <= 45.0 {
      self.zoom -= yoffset;
    }
    if self.zoom <= 1.0 {
      self.zoom = 1.0;
    }
    if self.zoom >= 45.0 {
      self.zoom = 45.0;
    }
  }

  fn updateCameraVectors(&mut self) {
    let front = Vector3 {
      x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
      y: self.pitch.to_radians().sin(),
      z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
    };
    self.front = front.normalize();
    self.right = self.front.cross(self.worldUp).normalize();
    self.up = self.right.cross(self.front).normalize();
  }
}
