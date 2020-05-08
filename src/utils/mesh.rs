#![allow(non_snake_case)]
use gl;

use std::ffi::CString;
use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;

use cgmath::prelude::*;
use cgmath::{Vector2, Vector3, vec3, Matrix4};

use super::shader::Shader;
use crate::c_str;

#[repr(C)]
#[derive(Clone)]
pub struct Vertex {
  pub Position: Vector3<f32>,
  pub Normal: Vector3<f32>,
  pub TexCoords: Vector2<f32>
}

impl Default for Vertex {
  fn default() -> Self {
    Vertex {
      Position: Vector3::zero(),
      Normal: Vector3::zero(),
      TexCoords: Vector2::zero()
    }
  }
}

#[derive(Clone)]
pub struct Texture {
  pub id: u32,
  pub type_: String,
  pub path: String,
}

pub struct Mesh {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u32>,
  pub textures: Vec<Texture>,
  VAO: u32,
  VBO: u32,
  EBO: u32
}

#[repr(C)]
pub struct Line {
  pub coords: Vec<Vector3<f32>>,
  VAO: u32,
  VBO: u32
}

impl Line {
  pub fn new(start: Vector3<f32>, end: Vector3<f32>) -> Line {
    let mut line = Line { coords: vec![start, end], VAO: 0, VBO: 0 };
    unsafe { line.setupLine() };
    line
  }

  pub fn dir(&self) -> Vector3<f32> { (self.coords[1] - self.coords[0]).normalize() }

  pub unsafe fn draw(&self, shader: &Shader, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
    let model = Matrix4::from_translation(vec3(0.0,0.0,0.0));
    shader.useProgram();
    shader.setMat4(c_str!("model"), &model);
    shader.setMat4(c_str!("view"), view);
    shader.setMat4(c_str!("projection"), projection);
    gl::BindVertexArray(self.VAO);
    gl::DrawArrays(gl::LINES, 0, self.coords.len() as i32);
    gl::BindVertexArray(0);
  }

  unsafe fn setupLine(&mut self) {
    gl::GenVertexArrays(1, &mut self.VAO);
    gl::GenBuffers(1, &mut self.VBO);

    gl::BindVertexArray(self.VAO);
    gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO);

    let size = (self.coords.len() * size_of::<Vector3<f32>>()) as isize;
    let data = &self.coords[0] as *const Vector3<f32> as *const c_void;
    gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

    let size = size_of::<Vector3<f32>>() as i32;
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size, 0 as *const c_void);
    gl::BindVertexArray(0);
  }
}

impl Mesh {
  pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh {
    let mut mesh = Mesh { vertices, indices, textures, VAO: 0, VBO: 0, EBO: 0 };
    unsafe { mesh.setupMesh() }
    mesh
  }

  unsafe fn setupMesh(&mut self) {
    gl::GenVertexArrays(1, &mut self.VAO);
    gl::GenBuffers(1, &mut self.VBO);
    gl::GenBuffers(1, &mut self.EBO);

    gl::BindVertexArray(self.VAO);
    gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO);
    let vSize = (self.vertices.len() * size_of::<Vertex>()) as isize;
    let vData = &self.vertices[0] as *const Vertex as *const c_void;
    gl::BufferData(gl::ARRAY_BUFFER, vSize, vData, gl::STATIC_DRAW);

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
    let eSize = (self.indices.len() * size_of::<u32>()) as isize;
    let eData = &self.indices[0] as *const u32 as *const c_void;
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, eSize, eData, gl::STATIC_DRAW);

    let attribSize = size_of::<Vertex>() as i32;
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, attribSize, offset_of!(Vertex, Position) as *const c_void);
    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, attribSize, offset_of!(Vertex, Normal) as *const c_void);
    gl::EnableVertexAttribArray(2);
    gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, attribSize, offset_of!(Vertex, TexCoords) as *const c_void);
    gl::BindVertexArray(0);
  }

  pub unsafe fn drawBoundingBox(&self) {
    gl::BindVertexArray(self.VAO);
    gl::DrawElements(gl::LINE_LOOP, 4, gl::UNSIGNED_INT, ptr::null());
    gl::DrawElements(gl::LINE_LOOP, 4, gl::UNSIGNED_INT, (4 * size_of::<u32>() as isize) as *const c_void);
    gl::DrawElements(gl::LINES, 8, gl::UNSIGNED_INT, (8 * size_of::<u32>() as isize) as *const c_void);
    gl::BindVertexArray(0);
  }

  pub unsafe fn draw(&self, shader: &Shader) {
    let mut diffuseNr = 0;
    let mut specularNr = 0;
    let mut normalNr = 0;
    let mut heightNr = 0;
    for (i, texture) in self.textures.iter().enumerate() {
      gl::ActiveTexture(gl::TEXTURE0 + i as u32); // active proper texture unit before binding
                                                  // retrieve texture number (the N in diffuse_textureN)
      let name = &texture.type_;
      let number = match name.as_str() {
        "texture_diffuse" => {
          diffuseNr += 1;
          diffuseNr
        }
        "texture_specular" => {
          specularNr += 1;
          specularNr
        }
        "texture_normal" => {
          normalNr += 1;
          normalNr
        }
        "texture_height" => {
          heightNr += 1;
          heightNr
        }
        _ => panic!("unknown texture type"),
      };

      let sampler = CString::new(format!("{}{}", name, number)).unwrap();
      gl::Uniform1i(gl::GetUniformLocation(shader.ID, sampler.as_ptr()), i as i32);
      gl::BindTexture(gl::TEXTURE_2D, texture.id);
    }

    gl::BindVertexArray(self.VAO);
    gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
    gl::BindVertexArray(0);
    gl::BindTexture(gl::TEXTURE_2D, 0);
    gl::ActiveTexture(gl::TEXTURE0);
  }
}
