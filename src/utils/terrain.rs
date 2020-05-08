#![allow(non_snake_case)]
use std::collections::HashMap;
use std::path::Path;

use image;
use image::GenericImage;
use cgmath::{vec2, vec3, Vector2, Vector3, Point3, Rad};

use super::mesh::{Mesh, Vertex, Texture};
use super::common::*;
use crate::entity::Entity;

const SCALE: f32 = 40.0;
const MAX_PIXEL_COLOR: f32 = 128 as f32;
const SIZE: f32 = 800.0;
type Heights = HashMap<(u32, u32), f32>;

pub struct Terrain {
  pub entity: Entity,
  heights: Heights
}

impl Terrain {
  pub fn new(worldPos: Point3<f32>, orientation: Vector3<Rad<f32>>, scale: f32) -> Terrain {
    let (mesh, heightArr) = genTerrain("resources/textures/heightmap.png");
    let e = Entity::new(vec![mesh], Point3{ x: worldPos.x, y: worldPos.y, z: worldPos.z }, orientation, scale);
    Terrain { entity: e, heights: heightArr }
  }
  
  pub fn getHeight(&self, worldX: f32, worldZ: f32) -> f32 {
    let terrainX = worldX - self.entity.worldPos.x;
    let terrainZ = worldZ - self.entity.worldPos.z;
    let squareSize = SIZE / ((self.heights.len() as f64).sqrt() - 1.0) as f32;
    let x = (terrainX / squareSize).floor() as u32;
    let z = (terrainZ / squareSize).floor() as u32;

    let xCoordInSquare = (terrainX % squareSize)/squareSize;
    let zCoordInSquare = (terrainZ % squareSize)/squareSize;
    if xCoordInSquare <= (1.0 - zCoordInSquare) {
      barryCentric(
        vec3(0.0, *self.heights.get(&(x, z)).unwrap_or(&0.0), 0.0), 
        vec3(1.0, *self.heights.get(&(x+1, z)).unwrap_or(&0.0), 0.0), 
        vec3(0.0, *self.heights.get(&(x, z+1)).unwrap_or(&0.0), 1.0), vec2(xCoordInSquare, zCoordInSquare)
      )
    } else {
      barryCentric(
        vec3(1.0, *self.heights.get(&(x+1, z)).unwrap_or(&0.0), 0.0), 
        vec3(1.0, *self.heights.get(&(x+1,z+1)).unwrap_or(&0.0), 1.0), 
        vec3(0.0, *self.heights.get(&(x,z+1)).unwrap_or(&0.0), 1.0), vec2(xCoordInSquare, zCoordInSquare)
      )
    }
  }
}

fn genTerrain(heightMap: &str) -> (Mesh, Heights) {
  let img = image::open(&Path::new(&heightMap)).expect("Heightmap failed to load");
  let VERTEX_COUNT = img.height();

  let (vertices, heights) = genVertices(img, VERTEX_COUNT);
  let indices = genIndices(VERTEX_COUNT);
  let texture = Texture { 
    id: unsafe { textureFromFile("grass.jpg", "resources/textures") }, 
    type_: "texture_normal".into(), 
    path: "grass.jpg".into()
  };

  let mesh = Mesh::new(vertices, indices, vec![texture]);
  (mesh, heights)
}

fn getHeightFromImage(x: u32, z: u32, img: &image::DynamicImage) -> f32 {
  if x >= img.height() || z >= img.height() {
    0.0
  } else {
    let p = img.get_pixel(x, z).data;
    let mut height = p[0] as f32;
    height += MAX_PIXEL_COLOR/2.0;
    height /= MAX_PIXEL_COLOR/2.0;
    height *= SCALE;
    height
  }
}

fn genVertices(img: image::DynamicImage, VERTEX_COUNT: u32) -> (Vec<Vertex>, Heights) {
  let mut vertexVec: Vec<Vertex> = Vec::with_capacity((VERTEX_COUNT * VERTEX_COUNT) as usize);
  let mut heights: Heights = HashMap::default();

  for gz in 0..VERTEX_COUNT {
    for gx in 0..VERTEX_COUNT {
      let height = getHeightFromImage(gx, gz, &img);
      let x = (gx as f32)/((VERTEX_COUNT - 1) as f32) * SIZE;
      let y = height;
      let z = (gz as f32)/((VERTEX_COUNT - 1) as f32) * SIZE;
      let xN  = 0.0;
      let yN = 1.0;
      let zN = 0.0;
      let tX = (gx as f32)/((VERTEX_COUNT - 1) as f32);
      let tZ = (gz as f32)/((VERTEX_COUNT - 1) as f32);

      vertexVec.push(Vertex { Position: vec3(x, y, z), Normal: vec3(xN, yN, zN), TexCoords: vec2(tX, tZ), ..Vertex::default() });
      heights.insert((gx, gz), height);
    }
  }

  (vertexVec, heights)
}

fn genIndices(VERTEX_COUNT: u32) -> Vec<u32> {
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

fn barryCentric(p1: Vector3<f32>, p2: Vector3<f32>, p3: Vector3<f32>, pos: Vector2<f32>) -> f32 {
  let det = (p2.z - p3.z) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.z - p3.z);
  let l1 = ((p2.z - p3.z) * (pos.x - p3.x) + (p3.x - p2.x) * (pos.y - p3.z)) / det;
  let l2 = ((p3.z - p1.z) * (pos.x - p3.x) + (p1.x - p3.x) * (pos.y - p3.z)) / det;
  let l3 = 1.0 - l1 - l2;
  l1 * p1.y + l2 * p2.y + l3 * p3.y
}