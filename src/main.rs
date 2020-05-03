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

mod entity;
use entity::Entity;

const SCR_WIDTH: u32 = 3840;
const SCR_HEIGHT: u32 = 2160;

pub fn main() {

  // Gl init
  let mut glfw = initGlfw();
  let (mut window, events) = createAndInitWindow(glfw, SCR_WIDTH, SCR_HEIGHT);
  initGl(&mut window);

  // Camera/Mouse data
  let mut firstMouse = true;
  let mut lastX: f32 = SCR_WIDTH as f32 / 2.0;
  let mut lastY: f32 = SCR_HEIGHT as f32 / 2.0;
  let mut deltaTime: f32 = 0.0;
  let mut lastFrame: f32 = 0.0;
  let mut camera = Camera {
    Position: Point3::new(0.0, 0.0, 3.0),
    ..Camera::default()
  };

  // Shaders
  let projection = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
  let mainShader = Shader::new("src/shaders/mainVertex.vs", "src/shaders/mainFragment.fs", projection);
  let terrainShader = Shader::new("src/shaders/terrVertex.vs", "src/shaders/terrFragment.fs", projection);

  // Entities
  let mut nanoEntity = Entity::new(Model::new("resources/objects/nanosuit/nanosuit.obj").meshes, Point3::new(10.0, -1.75, 10.0), vec3(Rad(0.0), Rad(0.0), Rad(0.0)), 0.2);
  let terrainEntity = Entity::new(vec![genTerrain()], Point3::new(0.0, -1.75, 0.0), vec3(Rad(0.0), Rad(0.0), Rad(0.0)), 1.0);

  while !window.should_close() {
    updateTimings(glfw, &mut deltaTime, &mut lastFrame);
    process_events(&events, &mut firstMouse, &mut lastX, &mut lastY, &mut camera);
    processInput(&mut window, deltaTime, &mut camera);
    
    let view = camera.GetViewMatrix();
    nanoEntity.orientation.y = Rad(glfw.get_time().sin() as f32);

    unsafe {
      gl::ClearColor(0.1, 0.1, 0.1, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
      
      terrainEntity.draw(&terrainShader, view);
      nanoEntity.draw(&mainShader, view);
    }

    window.swap_buffers();
    glfw.poll_events();
  }
}
