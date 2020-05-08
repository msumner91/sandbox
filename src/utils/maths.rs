#![allow(non_snake_case)]
use cgmath::{vec3, Vector2, Vector3, Vector4, Matrix4, SquareMatrix, InnerSpace};

use super::mesh::Vertex;
use super::camera::Camera;
use crate::{SCR_WIDTH, SCR_HEIGHT, DRAW_DISTANCE};

// Bounding box
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

fn getNormalisedDeviceCoords(mouseX: f32, mouseY: f32) -> Vector2<f32> {
  Vector2 { x: (mouseX*2.0 / SCR_WIDTH as f32) - 1.0, y: 1.0 - (mouseY*2.0 / SCR_HEIGHT as f32) }
}

fn toEyeCoords(clipCoords: Vector4<f32>, projectionMatrix: Matrix4<f32>) -> Vector4<f32> {
  let invProjection = projectionMatrix.invert().unwrap();
  let transformedV = invProjection * clipCoords;
  Vector4{ x: transformedV.x, y: transformedV.y, z: -1.0, w: 0.0 }
}

fn toWorldCoords(eyeCoords: Vector4<f32>, viewMatrix: Matrix4<f32>) -> Vector3<f32> {
  let invView = viewMatrix.invert().unwrap();
  let transformedV = invView * eyeCoords;
  let result = Vector3{ x: transformedV.x, y: transformedV.y, z: transformedV.z };
  result.normalize()
}

pub fn translateCoords(xpos: f32, ypos: f32, projectionMatrix: &Matrix4<f32>, cam: &Camera) -> (Vector3<f32>, Vector3<f32>) {
  println!("xpos: {}, ypos: {}", xpos, ypos);
  let normalisedCoords = getNormalisedDeviceCoords(xpos, ypos);
  println!("nXPos: {}, nYPos: {}", normalisedCoords.x, normalisedCoords.y);
  let clipCoords = Vector4 { x: normalisedCoords.x, y: normalisedCoords.y, z: -1.0, w: 1.0 };
  let eyeCoords = toEyeCoords(clipCoords, *projectionMatrix);
  println!("Eye: {}, {}, {}, {}", eyeCoords.x, eyeCoords.y, eyeCoords.z, eyeCoords.w);
  let worldCoords = toWorldCoords(eyeCoords, cam.getViewMatrix());
  println!("World: {}, {}, {}", worldCoords.x, worldCoords.y, worldCoords.z);

  let scaledWorld = worldCoords * DRAW_DISTANCE;
  let start = Vector3 { x: cam.Position.x, y: cam.Position.y, z: cam.Position.z };
  let end = Vector3 { x: cam.Position.x + scaledWorld.x, y: cam.Position.y + scaledWorld.y, z: cam.Position.z + scaledWorld.z };
  println!{"Line start/end: {}, {}, {} -> {}, {}, {}", start.x, start.y, start.z, end.x, end.y, end.z};
  (start, end)
}

pub fn computeBoundingBox(vertices: &[Vertex]) -> (Vector4<f32>, Vector4<f32>) {
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

pub fn computeBoundingBoxTransform(min: Vector4<f32>, max: Vector4<f32>) -> Matrix4<f32> {
  let size = vec3(max.x-min.x, max.y-min.y, max.z-min.z);
  let center = vec3((min.x+max.x)/2.0, (min.y+max.y)/2.0, (min.z+max.z)/2.0);
  Matrix4::from_translation(center) * Matrix4::from_nonuniform_scale(size[0], size[1], size[2])
}