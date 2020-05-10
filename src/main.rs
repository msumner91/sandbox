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
use utils::terrain::{Terrain, SIZE};
use utils::mesh::Mesh;
use utils::maths::{BOUNDING_BOX, BOUNDING_BOX_INDICES};

mod entity;
use entity::Entity;

mod light;
use light::Light;

const SCR_WIDTH: u32 = 3840;
const SCR_HEIGHT: u32 = 2160;
const DRAW_DISTANCE: f32 = 1500.0;

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
  let mut nanoEntity = Entity::new(Model::new("resources/objects/nanosuit/nanosuit.obj").meshes, Point3::new(20.0, terrain.getHeight(20.0, 20.0), 20.0), vec3(Rad(0.0), Rad(0.0), Rad(0.0)), 1.0, 80.0);

  // Lights
  let light = Light::new(vec3(SIZE / 2.0, terrain.getHeight(35.0, 35.0) + 1000.0, SIZE / 2.0), vec3(255.0, 241.0, 224.0), vec3(1.0, 0.01, 0.0), 0.05);
  mainShader.loadLight(&light.position, &light.colour, &light.attenuation);
  mainShader.loadShine(10000.0, 5.0);
  terrainShader.loadLight(&light.position, &light.colour, &light.attenuation);

  while !window.should_close() {
    updateTimings(glfw, &mut deltaTime, &mut lastFrame);

    let projection = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, DRAW_DISTANCE);
    process_events(&mut window, &events, &mut firstMouse, &mut lastX, &mut lastY, &mut camera);
    processInput(&mut window, deltaTime, &mut camera, &mut nanoEntity, lastX, lastY, &terrain, &projection);
    
    unsafe {
      gl::ClearColor(0.1, 0.1, 0.1, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
      
      let view = camera.getViewMatrix();
      terrain.entity.draw(&terrainShader, &view, &projection);
      terrain.entity.drawBoundingBox(&lineShader, &boundingMesh, &view, &projection);

      nanoEntity.draw(&mainShader, &view, &projection);
      nanoEntity.drawBoundingBox(&lineShader, &boundingMesh, &view, &projection);
    }

    window.swap_buffers();
    glfw.poll_events();
  }
}
