extern crate glfw;
use self::glfw::Key;

use std::ffi::CStr;

use crate::c_str;
use crate::utils::mesh::*;
use crate::utils::shader::Shader;
use crate::{BOUNDING_BOX, BOUNDING_BOX_INDICES};
use cgmath::{vec3, Matrix4, Point3, Vector3, Rad};

pub struct Entity {
  meshes: Vec<Mesh>,
  boundingMesh: Mesh,
  boundingTransforms: Vec<Matrix4<f32>>,
  pub worldPos: Point3<f32>,
  pub orientation: Vector3<Rad<f32>>,
  pub scale: f32,
  speed: f32
}

impl Entity {
  pub fn new(meshes: Vec<Mesh>, worldPos: Point3<f32>, orientation: Vector3<Rad<f32>>, scale: f32) -> Entity {
    let boundingMesh = Mesh::new(BOUNDING_BOX.to_vec(), BOUNDING_BOX_INDICES.to_vec(), vec![]);
    let mut boundingTransforms = Vec::with_capacity(meshes.len());
    for m in &meshes { boundingTransforms.push(computeBoundingBoxTransform(&m.vertices)); }
    Entity { meshes, boundingMesh, boundingTransforms, worldPos, orientation, scale, speed: 2.5 }
  }

  fn getModelMatrix(&self) -> Matrix4<f32> {
    Matrix4::from_translation(vec3(self.worldPos.x, self.worldPos.y, self.worldPos.z)) * 
    Matrix4::from_angle_x(self.orientation.x) *
    Matrix4::from_angle_z(self.orientation.z) *
    Matrix4::from_angle_y(self.orientation.y) *
    Matrix4::from_scale(self.scale)
  }

  pub fn draw(&self, shader: &Shader, view: Matrix4<f32>) {
    updateModel(shader, self.getModelMatrix());
    updateViewAndProjection(shader, view);

    for mesh in &self.meshes {
      unsafe {
        mesh.draw(shader);
      }
    }
  }

  pub fn drawWithBoundingBox(&self, shader: &Shader, view: Matrix4<f32>) {
    let model = self.getModelMatrix();
    updateModel(shader, model);
    updateViewAndProjection(shader, view);

    for (mesh, boundingTransform) in self.meshes.iter().zip(self.boundingTransforms.iter()) {
      unsafe {
        mesh.draw(shader);
        updateModel(shader, model * boundingTransform);
        self.boundingMesh.drawBoundingBox();
      }
    }
  }

  pub fn processKeyboard(&mut self, key: Key, deltaTime: f32) {
    let velocity = self.speed * deltaTime;
    match key {
      Key::Q => { self.worldPos.x = self.worldPos.x + velocity; self.worldPos.z = self.worldPos.z + velocity },
      _ => {} 
    }
  }
}

fn updateModel(shader: &Shader, model: Matrix4<f32>) {
  unsafe {
    shader.useProgram();
    shader.setMat4(c_str!("model"), &model);
  }
}

fn updateViewAndProjection(shader: &Shader, view: Matrix4<f32>) {
  unsafe {
    shader.useProgram();
    shader.setMat4(c_str!("view"), &view);
    shader.setMat4(c_str!("projection"), &shader.projection);
  }
}