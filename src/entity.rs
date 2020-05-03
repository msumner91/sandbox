use crate::c_str;
use crate::utils::mesh::*;
use crate::utils::shader::Shader;
use cgmath::{vec3, Matrix4, Point3, Vector3, Rad};

use std::ffi::CStr;

pub struct Entity {
  meshes: Vec<Mesh>,
  pub worldPos: Point3<f32>,
  pub orientation: Vector3<Rad<f32>>,
  pub scale: f32
}

impl Entity {
  pub fn new(meshes: Vec<Mesh>, worldPos: Point3<f32>, orientation: Vector3<Rad<f32>>, scale: f32) -> Entity {
    Entity { meshes, worldPos, orientation, scale }
  }

  pub fn draw(&self, shader: &Shader, view: Matrix4<f32>) {
    let model = 
      Matrix4::from_scale(self.scale) *
      Matrix4::from_translation(vec3(self.worldPos.x, self.worldPos.y, self.worldPos.z)) * 
      Matrix4::from_angle_z(self.orientation.z) *
      Matrix4::from_angle_x(self.orientation.x) *
      Matrix4::from_angle_y(self.orientation.y);

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
}
