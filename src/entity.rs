extern crate glfw;
use self::glfw::Key;

use std::ffi::CStr;

use crate::c_str;
use crate::utils::mesh::*;
use crate::utils::shader::Shader;
use cgmath::{vec3, Matrix4, Point3, Vector3, Rad};

pub struct Entity {
  meshes: Vec<Mesh>,
  pub worldPos: Point3<f32>,
  pub orientation: Vector3<Rad<f32>>,
  pub scale: f32,
  speed: f32
}

impl Entity {
  pub fn new(meshes: Vec<Mesh>, worldPos: Point3<f32>, orientation: Vector3<Rad<f32>>, scale: f32) -> Entity {
    Entity { meshes, worldPos, orientation, scale, speed: 2.5 }
  }

  pub fn draw(&self, shader: &Shader, view: Matrix4<f32>) {
    let model = 
      Matrix4::from_translation(vec3(self.worldPos.x, self.worldPos.y, self.worldPos.z)) * 
      Matrix4::from_angle_x(self.orientation.x) *
      Matrix4::from_angle_z(self.orientation.z) *
      Matrix4::from_angle_y(self.orientation.y) *
      Matrix4::from_scale(self.scale);

    unsafe {
      shader.useProgram();
      shader.setMat4(c_str!("model"), &model);
      shader.setMat4(c_str!("view"), &view);
      shader.setMat4(c_str!("projection"), &shader.projection);
    }

    for mesh in &self.meshes {
      unsafe {
        mesh.draw(shader);
      }
    }
  }

  pub fn processKeyboard(&mut self, key: Key, deltaTime: f32) {
    let velocity = self.speed * deltaTime;
    match key {
      Key::Q => { self.worldPos.x = self.worldPos.x + velocity; self.worldPos.z = self.worldPos.z + velocity },
      _ => {} 
      // BACKWARD => self.Position += -(self.Front * velocity),
      // LEFT => self.Position += -(self.Right * velocity),
      // RIGHT => self.Position += self.Right * velocity,
    }
  }
}