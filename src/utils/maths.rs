#![allow(non_snake_case)]
use cgmath::{vec3, SquareMatrix, InnerSpace};

use crate::mesh::Vertex;
use crate::camera::Camera;
use crate::types::*;
use crate::{SCR_WIDTH, SCR_HEIGHT, DRAW_DISTANCE};

pub static BOUNDING_BOX: [Vertex; 8] = [
  Vertex { Position: Vector3 { x: -0.5, y: -0.5, z: -0.5 }, Normal: Vector3 { x: 0.0, y: 0.0, z: 0.0 }, TexCoords: Vector2 { x: 0.0, y: 0.0 } },
  Vertex { Position: Vector3 { x: 0.5, y: -0.5, z: -0.5 }, Normal:  Vector3 { x: 0.0, y: 0.0, z: 0.0 }, TexCoords: Vector2 { x: 0.0, y: 0.0 } },
  Vertex { Position: Vector3 { x: 0.5, y: 0.5, z: -0.5 }, Normal:   Vector3 { x: 0.0, y: 0.0, z: 0.0 }, TexCoords: Vector2 { x: 0.0, y: 0.0 } },
  Vertex { Position: Vector3 { x: -0.5, y: 0.5, z: -0.5 }, Normal:  Vector3 { x: 0.0, y: 0.0, z: 0.0 }, TexCoords: Vector2 { x: 0.0, y: 0.0 } },
  Vertex { Position: Vector3 { x: -0.5, y: -0.5, z: 0.5 }, Normal:  Vector3 { x: 0.0, y: 0.0, z: 0.0 }, TexCoords: Vector2 { x: 0.0, y: 0.0 } },
  Vertex { Position: Vector3 { x: 0.5, y: -0.5, z: 0.5 }, Normal:   Vector3 { x: 0.0, y: 0.0, z: 0.0 }, TexCoords: Vector2 { x: 0.0, y: 0.0 } },
  Vertex { Position: Vector3 { x: 0.5, y: 0.5, z: 0.5 }, Normal:    Vector3 { x: 0.0, y: 0.0, z: 0.0 }, TexCoords: Vector2 { x: 0.0, y: 0.0 } },
  Vertex { Position: Vector3 { x: -0.5, y: 0.5, z: 0.5 }, Normal:   Vector3 { x: 0.0, y: 0.0, z: 0.0 }, TexCoords: Vector2 { x: 0.0, y: 0.0 } }
];

pub static BOUNDING_BOX_INDICES: [u32; 16] = [
  0, 1, 2, 3,
  4, 5, 6, 7,
  0, 4, 1, 5, 2, 6, 3, 7
];

fn getNormalisedDeviceCoords(mouseX: f32, mouseY: f32) -> Vector2 {
  Vector2 { x: (mouseX*2.0 / SCR_WIDTH as f32) - 1.0, y: 1.0 - (mouseY*2.0 / SCR_HEIGHT as f32) }
}

fn toEyeCoords(clipCoords: Vector4, projectionMatrix: Matrix4) -> Vector4 {
  let invProjection = projectionMatrix.invert().unwrap();
  let transformedV = invProjection * clipCoords;
  Vector4{ x: transformedV.x, y: transformedV.y, z: -1.0, w: 0.0 }
}

fn toWorldCoords(eyeCoords: Vector4, viewMatrix: Matrix4) -> Vector3 {
  let invView = viewMatrix.invert().unwrap();
  let transformedV = invView * eyeCoords;
  let result = Vector3{ x: transformedV.x, y: transformedV.y, z: transformedV.z };
  result.normalize()
}

pub fn translateCoords(xpos: f32, ypos: f32, projectionMatrix: &Matrix4, cam: &Camera) -> (Vector3, Vector3) {
  let normalisedCoords = getNormalisedDeviceCoords(xpos, ypos);
  let clipCoords = Vector4 { x: normalisedCoords.x, y: normalisedCoords.y, z: -1.0, w: 1.0 };
  let eyeCoords = toEyeCoords(clipCoords, *projectionMatrix);
  let worldCoords = toWorldCoords(eyeCoords, cam.getViewMatrix());

  let scaledWorld = worldCoords * DRAW_DISTANCE;
  let start = Vector3 { x: cam.position.x, y: cam.position.y, z: cam.position.z };
  let end = Vector3 { x: cam.position.x + scaledWorld.x, y: cam.position.y + scaledWorld.y, z: cam.position.z + scaledWorld.z };
  (start, end)
}

pub fn computeBoundingBox(vertices: &[Vertex]) -> (Vector4, Vector4) {
  let mut min = Vector4 { x: f32::MAX, y: f32::MAX, z: f32::MAX, w: 1.0 };
  let mut max = Vector4 { x: f32::MIN, y: f32::MIN, z: f32::MIN, w: 1.0 };
  for v in vertices {
    if v.Position.x < min.x { min.x = v.Position.x };
    if v.Position.y < min.y { min.y = v.Position.y };
    if v.Position.z < min.z { min.z = v.Position.z };
    if v.Position.x > max.x { max.x = v.Position.x };
    if v.Position.y > max.y { max.y = v.Position.y };
    if v.Position.z > max.z { max.z = v.Position.z };
  }

  (min, max)
}

pub fn computeBoundingBoxTransform(min: Vector4, max: Vector4) -> Matrix4 {
  let size = vec3(max.x-min.x, max.y-min.y, max.z-min.z);
  let center = vec3((min.x+max.x)/2.0, (min.y+max.y)/2.0, (min.z+max.z)/2.0);
  Matrix4::from_translation(center) * Matrix4::from_nonuniform_scale(size[0], size[1], size[2])
}