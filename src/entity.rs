#![allow(non_snake_case)]
extern crate glfw;
use self::glfw::Key;

use cgmath::{vec3, Matrix4, Point3, Vector3, Vector4, Rad};

use crate::utils::mesh::*;
use crate::utils::maths::{computeBoundingBox, computeBoundingBoxTransform};
use crate::utils::shader::Shader;

pub struct Entity {
  meshes: Vec<Mesh>,
  boundingBoxes: Vec<BoundingBox>,
  boundingTransforms: Vec<Matrix4<f32>>,
  pub worldPos: Point3<f32>,
  pub orientation: Vector3<Rad<f32>>,
  pub scale: f32,
  speed: f32
}

struct BoundingBox {
  min: Vector4<f32>,
  max: Vector4<f32>
}

impl BoundingBox {
  pub fn new(min: Vector4<f32>, max: Vector4<f32>) -> BoundingBox { BoundingBox { min: min, max: max } }
}

impl Entity {
  pub fn new(meshes: Vec<Mesh>, worldPos: Point3<f32>, orientation: Vector3<Rad<f32>>, scale: f32) -> Entity {
    let mut boundingBoxes = Vec::with_capacity(meshes.len());
    let mut boundingTransforms = Vec::with_capacity(meshes.len());
    for m in &meshes { 
      let (min, max) = computeBoundingBox(&m.vertices);
      boundingBoxes.push(BoundingBox::new(min, max));
      boundingTransforms.push(computeBoundingBoxTransform(min, max)); 
    }
    Entity { meshes, boundingBoxes, boundingTransforms, worldPos, orientation, scale, speed: 20.0 }
  }

  fn getModelMatrix(&self) -> Matrix4<f32> {
    Matrix4::from_translation(vec3(self.worldPos.x, self.worldPos.y, self.worldPos.z)) * 
    Matrix4::from_angle_x(self.orientation.x) *
    Matrix4::from_angle_z(self.orientation.z) *
    Matrix4::from_angle_y(self.orientation.y) *
    Matrix4::from_scale(self.scale)
  }

  pub fn draw(&self, shader: &Shader, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
    shader.initShader(&self.getModelMatrix(), view, projection);
    for mesh in &self.meshes { unsafe { mesh.draw(shader) } }
  }

  pub fn drawBoundingBox(&self, shader: &Shader, mesh: &Mesh, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
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
      let rayDir = ray.dir();

      let mut tMin = (vMin.x - ray.coords[0].x) / rayDir.x;
      let mut tMax = (vMax.x - ray.coords[0].x) / rayDir.x;
      if tMin > tMax { std::mem::swap(&mut tMin, &mut tMax) }

      let mut tYMin = (vMin.y - ray.coords[0].y) / rayDir.y;
      let mut tYMax = (vMax.y - ray.coords[0].y) / rayDir.y;
      if tYMin > tYMax { std::mem::swap(&mut tYMin, &mut tYMax) } 

      if tMin > tYMax || tYMin > tMax { continue }
      tMin = tMin.max(tYMin);
      tMax = tMax.min(tYMax);

      let mut tZMin = (vMin.z - ray.coords[0].z) / rayDir.z;
      let mut tZMax = (vMax.z - ray.coords[0].z) / rayDir.z;
      if tZMin > tZMax { std::mem::swap(&mut tZMin, &mut tZMax) }

      if tMin > tZMax || tZMin > tMax { continue }
      tMin = tMin.max(tZMin);
      tMax = tMax.min(tZMax);
      intersections.push((tMin, tMax));
    }

    intersections
  }

  pub fn processKeyboard(&mut self, key: Key, deltaTime: f32) {
    let velocity = self.speed * deltaTime;
    match key {
      Key::Up => { self.worldPos.x = self.worldPos.x + velocity; },
      Key::Down => { self.worldPos.x = self.worldPos.x - velocity; },
      Key::Left => { self.worldPos.z = self.worldPos.z + velocity },
      Key::Right => { self.worldPos.z = self.worldPos.z - velocity },
      _ => {} 
    }
  }
}