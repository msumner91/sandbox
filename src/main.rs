#![allow(non_snake_case)]
extern crate gl;
extern crate glfw;
use self::glfw::Context;

use cgmath::{perspective, Deg, Rad, Point3, vec3};

mod utils;
use utils::common::*;
use utils::model::Model;
use utils::camera::Camera;
use utils::shader::Shader;
use utils::terrain::Terrain;
use utils::mesh::Mesh;
use utils::maths::{BOUNDING_BOX, BOUNDING_BOX_INDICES};

mod entity;
use entity::Entity;

const SCR_WIDTH: u32 = 3840;
const SCR_HEIGHT: u32 = 2160;
const DRAW_DISTANCE: f32 = 1000.0;

pub fn main() {

  // Gl init
  let mut glfw = initGlfw();
  let (mut window, events) = createAndInitWindow(&mut glfw, SCR_WIDTH, SCR_HEIGHT);
  initGl(&mut window);

  // Camera/Mouse data
  let mut firstMouse = true;
  let mut lastX: f32 = SCR_WIDTH as f32 / 2.0;
  let mut lastY: f32 = SCR_HEIGHT as f32 / 2.0;
  let mut deltaTime: f32 = 0.0;
  let mut lastFrame: f32 = 0.0;
  let mut camera = Camera {
    Position: Point3::new(0.0, 160.0, 0.0),
    ..Camera::default()
  };

  // Shaders
  let mainShader = Shader::new("src/shaders/mainVertex.vs", "src/shaders/mainFragment.fs");
  let lineShader = Shader::new("src/shaders/lineVertex.vs", "src/shaders/lineFragment.fs");
  let terrainShader = Shader::new("src/shaders/terrVertex.vs", "src/shaders/terrFragment.fs");

  // Terrain/Entities
  let boundingMesh: Mesh = Mesh::new(BOUNDING_BOX.to_vec(), BOUNDING_BOX_INDICES.to_vec(), vec![]);
  let terrain = Terrain::new(Point3::new(0.0, 0.0, 0.0), vec3(Rad(0.0), Rad(0.0), Rad(0.0)), 1.0);
  let mut nanoEntity = Entity::new(Model::new("resources/objects/nanosuit/nanosuit.obj").meshes, Point3::new(20.0, terrain.getHeight(20.0, 20.0), 20.0), vec3(Rad(0.0), Rad(0.0), Rad(0.0)), 1.0);
  let mut lines = vec![];
  let mut cursor = 0;

  while !window.should_close() {
    updateTimings(glfw, &mut deltaTime, &mut lastFrame);
    process_events(&mut window, &events, &mut firstMouse, &mut lastX, &mut lastY, &mut camera);

    let projection = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, DRAW_DISTANCE);
    processInput(&mut window, deltaTime, &mut camera, &mut nanoEntity, &mut lines, lastX, lastY, &projection);
    
    unsafe {
      gl::ClearColor(0.1, 0.1, 0.1, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
      
      let view = camera.getViewMatrix();
      terrain.entity.draw(&terrainShader, &view, &projection);
      terrain.entity.drawBoundingBox(&lineShader, &boundingMesh, &view, &projection);

      nanoEntity.worldPos.y = terrain.getHeight(nanoEntity.worldPos.x, nanoEntity.worldPos.z);
      nanoEntity.draw(&mainShader, &view, &projection);
      nanoEntity.drawBoundingBox(&lineShader, &boundingMesh, &view, &projection);

      // Select new lines
      let old = cursor;
      cursor = lines.len();
      let newLines = &lines[old..cursor];

      let mut totalIntersections = 0;
      for l in newLines {
        totalIntersections += nanoEntity.intersect(l).len();
        println!("Total intersections: {}", totalIntersections);
      }
      for l in lines.iter() {
        l.draw(&lineShader, &view, &projection);
      }
    }

    window.swap_buffers();
    glfw.poll_events();
  }
}
