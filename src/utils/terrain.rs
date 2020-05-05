
#![allow(dead_code)]
#![allow(unused_variables)]

use cgmath::{vec2, vec3, Vector2, Vector3, Point3, Rad};
use rand::prelude::*;

use super::mesh::{Mesh, Vertex, Texture};
use super::common::*;

use crate::entity::Entity;

const VERTEX_COUNT: u32 = 128;
const SIZE: f32 = 800.0;

type Heights = [[f32; VERTEX_COUNT as usize]; VERTEX_COUNT as usize];

pub struct Terrain {
  pub entity: Entity,
  heights: Heights
}

impl Terrain {
  pub fn new(worldPos: Point3<f32>, orientation: Vector3<Rad<f32>>, scale: f32) -> Terrain {
    let (mesh, heightArr) = genTerrain();
    let e = Entity::new(vec![mesh], worldPos, orientation, scale);
    Terrain { entity: e, heights: heightArr }
  }
  
  pub fn getHeight(&self, worldX: f32, worldZ: f32) -> f32 {
    let terrainX = worldX - self.entity.worldPos.x;
    let terrainZ = worldZ - self.entity.worldPos.z;
    let squareSize = SIZE / ((self.heights.len() - 1) as f32);
    let gX = (terrainX / squareSize).floor() as usize;
    let gZ = (terrainZ / squareSize).floor() as usize;
    if gX >= self.heights.len() - 1 || gZ >= self.heights.len() - 1 { 0.0 } 
    else {
      let xCoordInSquare = (terrainX % squareSize)/squareSize;
      let zCoordInSquare = (terrainZ % squareSize)/squareSize;
      if xCoordInSquare <= (1.0 - zCoordInSquare) {
        barryCentric(
          vec3(0.0, self.heights[gX][gZ], 0.0), 
          vec3(1.0, self.heights[gX + 1][gZ], 0.0), 
          vec3(0.0, self.heights[gX][gZ + 1], 1.0), vec2(xCoordInSquare, zCoordInSquare)
        )
      } else {
        barryCentric(
          vec3(1.0, self.heights[gX + 1][gZ], 0.0), 
          vec3(1.0, self.heights[gX + 1][gZ + 1], 1.0), 
          vec3(0.0, self.heights[gX][gZ + 1], 1.0), vec2(xCoordInSquare, zCoordInSquare)
        )
      }
    }
  }
}

fn barryCentric(p1: Vector3<f32>, p2: Vector3<f32>, p3: Vector3<f32>, pos: Vector2<f32>) -> f32 {
  let det = (p2.z - p3.z) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.z - p3.z);
  let l1 = ((p2.z - p3.z) * (pos.x - p3.x) + (p3.x - p2.x) * (pos.y - p3.z)) / det;
  let l2 = ((p3.z - p1.z) * (pos.x - p3.x) + (p1.x - p3.x) * (pos.y - p3.z)) / det;
  let l3 = 1.0 - l1 - l2;
  l1 * p1.y + l2 * p2.y + l3 * p3.y
}

fn genVertices() -> (Vec<Vertex>, Heights) {
  let mut vertexVec: Vec<Vertex> = Vec::with_capacity((VERTEX_COUNT * VERTEX_COUNT) as usize);
  let mut heights = [[0.0; VERTEX_COUNT as usize]; VERTEX_COUNT as usize]; 
  let mut rng = rand::thread_rng();

  for gz in 0..VERTEX_COUNT {
    for gx in 0..VERTEX_COUNT {
      let height = rng.gen_range(0.0, 5.0);
      heights[gx as usize][gz as usize] = height;
      let x = (gx as f32)/((VERTEX_COUNT - 1) as f32) * SIZE;
      let y = height;
      let z = (gz as f32)/((VERTEX_COUNT - 1) as f32) * SIZE;
      let xN  = 0.0;
      let yN = 1.0;
      let zN = 0.0;
      let tX = (gx as f32)/((VERTEX_COUNT - 1) as f32);
      let tZ = (gz as f32)/((VERTEX_COUNT - 1) as f32);

      vertexVec.push(Vertex { Position: vec3(x, y, z), Normal: vec3(xN, yN, zN), TexCoords: vec2(tX, tZ), ..Vertex::default() });
    }
  }

  (vertexVec, heights)
}

fn genIndices() -> Vec<u32> {
  let mut indices = Vec::with_capacity((6 * (VERTEX_COUNT-1)*(VERTEX_COUNT-1)) as usize);
  for gz in 0..(VERTEX_COUNT-1) {
    for gx in 0..(VERTEX_COUNT-1) {
      let topLeft = (gz*VERTEX_COUNT)+gx;
      let topRight = topLeft + 1;
      let bottomLeft = ((gz+1)*VERTEX_COUNT)+gx;
      let bottomRight = bottomLeft + 1;
      indices.push(topLeft);
      indices.push(bottomLeft);
      indices.push(topRight);
      indices.push(topRight);
      indices.push(bottomLeft);
      indices.push(bottomRight);
    }
  }

  indices
}

fn genTerrain() -> (Mesh, Heights) {
  let (vertices, heights) = genVertices();
  let indices = genIndices();
  let texture = Texture { 
    id: unsafe { TextureFromFile("grass2.jpg", "resources/textures") }, 
    type_: "texture_normal".into(), 
    path: "grass2.jpg".into()
  };

  let mesh = Mesh::new(vertices, 
                       indices, 
                       vec![texture]);
  (mesh, heights)
}