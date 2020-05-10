#![allow(non_snake_case)]
use std::collections::HashMap;
use std::path::Path;

use cgmath::{vec2, vec3};
use tobj;

use super::common::*;
use crate::mesh::{Mesh, Texture, Vertex};

pub struct Model {
  pub meshes: Vec<Mesh>,
  pub texturesLoaded: HashMap<String, Texture>,
  directory: String,
}

impl Model {
  pub fn new(path: &str) -> Model {
    let mut model = Model { meshes: vec![], texturesLoaded: HashMap::default(), directory: String::default() };
    model.loadModel(path);
    model
  }

  fn loadModel(&mut self, path: &str) {
    let path = Path::new(path);
    self.directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
    let obj = tobj::load_obj(path);

    let (models, materials) = obj.unwrap();
    for model in models {
      let mesh = &model.mesh;
      let num_vertices = mesh.positions.len() / 3;

      let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
      let indices: Vec<u32> = mesh.indices.clone();

      let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
      for i in 0..num_vertices {
        vertices.push(Vertex {
          Position: vec3(p[i*3], p[i*3+1], p[i*3+2]),
          Normal: vec3(n[i * 3], n[i * 3 + 1], n[i * 3 + 2]),
          TexCoords: vec2(t[i * 2], t[i * 2 + 1]),
          ..Vertex::default()
        })
      }

      let mut textures = Vec::new();
      if let Some(material_id) = mesh.material_id {
        let material = &materials[material_id];

        if !material.diffuse_texture.is_empty() {
          let texture = self.loadMaterialTexture(&material.diffuse_texture, "texture_diffuse");
          textures.push(texture);
        }

        if !material.specular_texture.is_empty() {
          let texture = self.loadMaterialTexture(&material.specular_texture, "texture_specular");
          textures.push(texture);
        }

        if !material.normal_texture.is_empty() {
          let texture = self.loadMaterialTexture(&material.normal_texture, "texture_normal");
          textures.push(texture);
        }
      }

      self.meshes.push(Mesh::new(vertices, indices, textures));
    }
  }

  fn loadMaterialTexture(&mut self, path: &str, typeName: &str) -> Texture {
    {
      let texOpt = self.texturesLoaded.get(path);
      if let Some(tex) = texOpt {
        return tex.clone();
      }
    }

    let texture = Texture {
      id: unsafe { textureFromFile(path, &self.directory) },
      type_: typeName.into(),
      path: path.into(),
    };
    self.texturesLoaded.insert(path.into(), texture.clone());
    texture
  }
}
