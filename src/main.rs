#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
extern crate glfw;
extern crate gl;
use self::glfw::{Context, Glfw, Window, WindowEvent};

use cgmath::{perspective, vec3, Deg, Matrix4, Point3};
use std::ffi::CStr;
use std::sync::mpsc::Receiver;

mod utils;
use utils::camera::Camera as Camera;
use utils::shader::Shader as Shader;
use utils::model::Model as Model;
use utils::common::*;

const SCR_WIDTH: u32 = 1920;
const SCR_HEIGHT: u32 = 1080;

fn initGlfw() -> Glfw {
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
  #[cfg(target_os = "windows")]
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
  glfw
}

fn createAndInitWindow(glfw: Glfw) -> (Window, Receiver<(f64, WindowEvent)>) {
  let (mut window, events) = glfw
    .create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window");
  window.make_current();
  window.set_framebuffer_size_polling(true);
  window.set_cursor_pos_polling(true);
  window.set_scroll_polling(true);
  window.set_cursor_mode(glfw::CursorMode::Disabled);
  (window, events)
}

fn initGl(window: &mut Window) -> () {
  unsafe {
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    gl::Enable(gl::DEPTH_TEST);
    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
  }
}

fn updateTimings(glfw: Glfw, deltaTime: &mut f32, lastFrame: &mut f32) -> () {
  let currentFrame = glfw.get_time() as f32;
  *deltaTime = currentFrame - *lastFrame;
  *lastFrame = currentFrame;
}

pub fn main() {
  let mut glfw = initGlfw();
  let (mut window, events) = createAndInitWindow(glfw);
  initGl(&mut window);

  let mainShader = Shader::new("src/shaders/vertex.vs", "src/shaders/fragment.fs");
  let mainModel = Model::new("resources/objects/nanosuit/nanosuit.obj");

  let mut camera = Camera {
    Position: Point3::new(0.0, 0.0, 3.0),
    ..Camera::default()
  };
  let mut firstMouse = true;
  let mut lastX: f32 = SCR_WIDTH as f32 / 2.0;
  let mut lastY: f32 = SCR_HEIGHT as f32 / 2.0;
  let mut deltaTime: f32 = 0.0;
  let mut lastFrame: f32 = 0.0;

  while !window.should_close() {
    updateTimings(glfw, &mut deltaTime, &mut lastFrame);
    process_events(&events, &mut firstMouse, &mut lastX, &mut lastY, &mut camera);
    processInput(&mut window, deltaTime, &mut camera);

    unsafe {
      gl::ClearColor(0.1, 0.1, 0.1, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

      mainShader.useProgram();

      let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
      let view = camera.GetViewMatrix();
      let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -1.75, 0.0));
      model = model * Matrix4::from_scale(0.2);
      mainShader.setMat4(c_str!("projection"), &projection);
      mainShader.setMat4(c_str!("view"), &view);
      mainShader.setMat4(c_str!("model"), &model);

      mainModel.Draw(&mainShader);
    }

    window.swap_buffers();
    glfw.poll_events();
  }
}
