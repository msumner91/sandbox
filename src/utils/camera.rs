#![allow(non_snake_case)]
use cgmath;
use cgmath::prelude::*;
use cgmath::vec3;

type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

pub enum CameraMovement {
  FORWARD,
  BACKWARD,
  LEFT,
  RIGHT,
  UP,
  DOWN
}

const YAW: f32 = 45.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 40.0;
const SENSITIVTY: f32 = 0.1;
const ZOOM: f32 = 45.0;

pub struct Camera {
  pub Position: Point3,
  pub Front: Vector3,
  pub Up: Vector3,
  pub Right: Vector3,
  pub WorldUp: Vector3,

  pub Yaw: f32,
  pub Pitch: f32,

  pub MovementSpeed: f32,
  pub MouseSensitivity: f32,
  pub Zoom: f32,
}

impl Default for Camera {
  fn default() -> Camera {
    let mut camera = Camera {
      Position: Point3::new(0.0, 0.0, 0.0),
      Front: vec3(0.0, 0.0, -1.0),
      Up: Vector3::zero(), 
      Right: Vector3::zero(),
      WorldUp: Vector3::unit_y(),
      Yaw: YAW,
      Pitch: PITCH,
      MovementSpeed: SPEED,
      MouseSensitivity: SENSITIVTY,
      Zoom: ZOOM,
    };
    camera.updateCameraVectors();
    camera
  }
}

use CameraMovement::*;
impl Camera {
  pub fn getViewMatrix(&self) -> Matrix4 {
    Matrix4::look_at(self.Position, self.Position + self.Front, self.Up)
  }

  pub fn processKeyboard(&mut self, direction: CameraMovement, deltaTime: f32) {
    let velocity = self.MovementSpeed * deltaTime;
    match direction {
      FORWARD => self.Position += self.Front * velocity,
      BACKWARD => self.Position += -(self.Front * velocity),
      LEFT => self.Position += -(self.Right * velocity),
      RIGHT => self.Position += self.Right * velocity,
      UP => self.Position += self.Up * velocity,
      DOWN => self.Position -= self.Up * velocity
    }
  }

  pub fn processMouseMovement(&mut self, mut xoffset: f32, mut yoffset: f32, constrainPitch: bool) {
    xoffset *= self.MouseSensitivity;
    yoffset *= self.MouseSensitivity;
    self.Yaw += xoffset;
    self.Pitch += yoffset;

    if constrainPitch {
      if self.Pitch > 89.0 {
        self.Pitch = 89.0;
      }
      if self.Pitch < -89.0 {
        self.Pitch = -89.0;
      }
    }

    self.updateCameraVectors();
  }

  pub fn processMouseScroll(&mut self, yoffset: f32) {
    if self.Zoom >= 1.0 && self.Zoom <= 45.0 {
      self.Zoom -= yoffset;
    }
    if self.Zoom <= 1.0 {
      self.Zoom = 1.0;
    }
    if self.Zoom >= 45.0 {
      self.Zoom = 45.0;
    }
  }

  fn updateCameraVectors(&mut self) {
    let front = Vector3 {
      x: self.Yaw.to_radians().cos() * self.Pitch.to_radians().cos(),
      y: self.Pitch.to_radians().sin(),
      z: self.Yaw.to_radians().sin() * self.Pitch.to_radians().cos(),
    };
    self.Front = front.normalize();
    self.Right = self.Front.cross(self.WorldUp).normalize(); // Normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
    self.Up = self.Right.cross(self.Front).normalize();
  }
}
