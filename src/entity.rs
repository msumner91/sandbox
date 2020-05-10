#![allow(non_snake_case)]
extern crate glfw;
use self::glfw::Key;

use cgmath::{vec3, Rad};

use crate::mesh::*;
use crate::utils::maths::{computeBoundingBox, computeBoundingBoxTransform};
use crate::utils::shader::Shader;
use crate::terrain::{Terrain, DEADZONE, BOUND_MAX};
use crate::types::*;

pub struct Entity {
  meshes: Vec<Mesh>,
  boundingBoxes: Vec<BoundingBox>,
  boundingTransforms: Vec<Matrix4>,
  pub worldPos: Point3,
  pub orientation: cgmath::Vector3<Rad<f32>>,
  pub scale: f32,
  speed: f32
}

struct BoundingBox {
  min: Vector4,
  max: Vector4
}

impl BoundingBox {
  pub fn new(min: Vector4, max: Vector4) -> BoundingBox { BoundingBox { min: min, max: max } }
}

impl Entity {
  pub fn new(meshes: Vec<Mesh>, worldPos: Point3, orientation: cgmath::Vector3<Rad<f32>>, scale: f32, speed: f32) -> Entity {
    let mut boundingBoxes = Vec::with_capacity(meshes.len());
    let mut boundingTransforms = Vec::with_capacity(meshes.len());
    for m in &meshes { 
      let (min, max) = computeBoundingBox(&m.vertices);
      boundingBoxes.push(BoundingBox::new(min, max));
      boundingTransforms.push(computeBoundingBoxTransform(min, max)); 
    }
    Entity { meshes, boundingBoxes, boundingTransforms, worldPos, orientation, scale, speed: speed }
  }

  fn getModelMatrix(&self) -> Matrix4 {
    Matrix4::from_translation(vec3(self.worldPos.x, self.worldPos.y, self.worldPos.z)) * 
    Matrix4::from_angle_x(self.orientation.x) *
    Matrix4::from_angle_z(self.orientation.z) *
    Matrix4::from_angle_y(self.orientation.y) *
    Matrix4::from_scale(self.scale)
  }

  pub fn draw(&self, shader: &Shader, view: &Matrix4, projection: &Matrix4) {
    shader.initShader(&self.getModelMatrix(), view, projection);
    for mesh in &self.meshes { unsafe { mesh.draw(shader) } }
  }

  pub fn drawBoundingBox(&self, shader: &Shader, mesh: &Mesh, view: &Matrix4, projection: &Matrix4) {
    let model = self.getModelMatrix();
    shader.initShader(&model, view, projection);

    for (_, boundingTransform) in self.meshes.iter().zip(self.boundingTransforms.iter()) {
      unsafe {
        shader.updateModel(&(model * boundingTransform));
        mesh.drawBoundingBox();
      }
    }
  }

  pub fn intersect(&self, ray: &Line) -> Vec<(f32, f32)> {
    let model = self.getModelMatrix();
    let mut intersections = Vec::with_capacity(self.boundingBoxes.len());

    for boundingBox in self.boundingBoxes.iter() {
      let vMin = model * boundingBox.min;
      let vMax = model * boundingBox.max;

      let mut tMin = (vMin.x - ray.coords[0].x) / ray.dir.x;
      let mut tMax = (vMax.x - ray.coords[0].x) / ray.dir.x;
      if tMin > tMax { std::mem::swap(&mut tMin, &mut tMax) }

      let mut tYMin = (vMin.y - ray.coords[0].y) / ray.dir.y;
      let mut tYMax = (vMax.y - ray.coords[0].y) / ray.dir.y;
      if tYMin > tYMax { std::mem::swap(&mut tYMin, &mut tYMax) } 

      if tMin > tYMax || tYMin > tMax { continue }
      tMin = tMin.max(tYMin);
      tMax = tMax.min(tYMax);

      let mut tZMin = (vMin.z - ray.coords[0].z) / ray.dir.z;
      let mut tZMax = (vMax.z - ray.coords[0].z) / ray.dir.z;
      if tZMin > tZMax { std::mem::swap(&mut tZMin, &mut tZMax) }

      if tMin > tZMax || tZMin > tMax { continue }
      tMin = tMin.max(tZMin);
      tMax = tMax.min(tZMax);
      intersections.push((tMin, tMax));
    }

    intersections
  }

  pub fn processMouse(&mut self, dir: Vector3, terrain: &Terrain, deltaTime: f32) {
    let velocity = self.speed * deltaTime;
    self.worldPos.x = (self.worldPos.x + (dir.x * velocity)).max(DEADZONE).min(BOUND_MAX);
    self.worldPos.z = (self.worldPos.z + (dir.z * velocity)).max(DEADZONE).min(BOUND_MAX);
    self.worldPos.y = terrain.getHeight(self.worldPos.x, self.worldPos.z);
  }

  pub fn processKeyboard(&mut self, key: Key, terrain: &Terrain, deltaTime: f32) {
    let velocity = self.speed * deltaTime;
    match key {
      Key::Up => { self.worldPos.z = (self.worldPos.z + velocity).max(DEADZONE).min(BOUND_MAX) },
      Key::Down => { self.worldPos.z = (self.worldPos.z - velocity).max(DEADZONE).min(BOUND_MAX) },
      Key::Left => { self.worldPos.x = (self.worldPos.x + velocity).max(DEADZONE).min(BOUND_MAX) },
      Key::Right => { self.worldPos.x = (self.worldPos.x - velocity).max(DEADZONE).min(BOUND_MAX) },
      _ => {} 
    }

    self.worldPos.y = terrain.getHeight(self.worldPos.x, self.worldPos.z);
  }
}