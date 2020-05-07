#![allow(non_snake_case)]
#![allow(dead_code)]
extern crate glfw;
use self::glfw::{Context, Glfw, Window, WindowEvent, Action, Key, MouseButtonLeft, MouseButtonRight};
use gl;

use std::sync::mpsc::Receiver;
use std::os::raw::c_void;
use std::path::Path;

use cgmath::{Vector2, Vector3, Vector4, Matrix4, SquareMatrix, InnerSpace};
use image::*;

use super::camera::Camera;
use super::camera::Camera_Movement::*;
use super::mesh::Line;
use crate::entity::Entity;
use crate::{SCR_WIDTH, SCR_HEIGHT, DRAW_DISTANCE};

pub fn initGlfw() -> Glfw {
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
  #[cfg(target_os = "windows")]
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
  glfw
}

pub fn createAndInitWindow(glfw: &mut Glfw, w: u32, h: u32) -> (Window, Receiver<(f64, WindowEvent)>) {
  let (mut window, events) = glfw.with_primary_monitor(|g, m| {
    g.create_window(w, h, "Rust Sandbox", m.map_or(glfw::WindowMode::Windowed, |m| glfw::WindowMode::FullScreen(m)))
  }).expect("Failed to create GLFW window");
  window.make_current();
  window.set_framebuffer_size_polling(true);
  window.set_cursor_pos_polling(true);
  window.set_scroll_polling(true);
  // window.set_cursor_mode(glfw::CursorMode::Disabled);
  (window, events)
}

pub fn initGl(window: &mut Window) -> () {
  unsafe {
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    gl::Enable(gl::DEPTH_TEST);
    // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
  }
}

pub fn updateTimings(glfw: Glfw, deltaTime: &mut f32, lastFrame: &mut f32) -> () {
  let currentFrame = glfw.get_time() as f32;
  *deltaTime = currentFrame - *lastFrame;
  *lastFrame = currentFrame;
}

pub fn process_events(
  window: &mut glfw::Window,
  events: &Receiver<(f64, glfw::WindowEvent)>,
  firstMouse: &mut bool,
  lastX: &mut f32,
  lastY: &mut f32,
  camera: &mut Camera,
  projectionMatrix: &Matrix4<f32>
) {
  for (_, event) in glfw::flush_messages(events) {
    match event {
      glfw::WindowEvent::FramebufferSize(width, height) => {
        // make sure the viewport matches the new window dimensions; note that width and
        // height will be significantly larger than specified on retina displays.
        unsafe { gl::Viewport(0, 0, width, height) }
      }
      glfw::WindowEvent::CursorPos(xpos, ypos) => {
        let (xpos, ypos) = (xpos as f32, ypos as f32);

        // Only run once for initial pos
        if *firstMouse {
          *lastX = xpos;
          *lastY = ypos;
          *firstMouse = false;
        }

        let xoffset = xpos - *lastX;
        let yoffset = *lastY - ypos; // reversed since y-coordinates go from bottom to top

        *lastX = xpos;
        *lastY = ypos;

        if window.get_mouse_button(MouseButtonLeft) == Action::Press {
          camera.processMouseMovement(xoffset, yoffset, true);
        }

        translateCoords(xpos, ypos, projectionMatrix, camera);
      }

      glfw::WindowEvent::Scroll(_xoffset, yoffset) => camera.processMouseScroll(yoffset as f32),

      _ => {}
    }
  }
}

pub fn processInput(window: &mut glfw::Window, deltaTime: f32, camera: &mut Camera, nanoEntity: &mut Entity, lines: &mut Vec<Line>, lastX: f32, lastY: f32, projectionMatrix: &Matrix4<f32>) {
  if window.get_key(Key::Escape) == Action::Press {
    window.set_should_close(true)
  }
  if window.get_key(Key::W) == Action::Press {
    camera.processKeyboard(FORWARD, deltaTime);
  }
  if window.get_key(Key::S) == Action::Press {
    camera.processKeyboard(BACKWARD, deltaTime);
  }
  if window.get_key(Key::A) == Action::Press {
    camera.processKeyboard(LEFT, deltaTime);
  }
  if window.get_key(Key::D) == Action::Press {
    camera.processKeyboard(RIGHT, deltaTime);
  }
  if window.get_key(Key::Q) == Action::Press {
    nanoEntity.processKeyboard(Key::Q, deltaTime);
  }

  if window.get_mouse_button(MouseButtonRight) == Action::Press {
    let (start, end) = translateCoords(lastX, lastY, projectionMatrix, camera);
    lines.push(Line::new(start, end));
  }
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

pub unsafe fn TextureFromFile(path: &str, directory: &str) -> u32 {
  let filename = format!("{}/{}", directory, path);
  println!("Filename: {}", filename);

  let mut textureID = 0;
  gl::GenTextures(1, &mut textureID);

  let img = image::open(&Path::new(&filename)).expect("Texture failed to load");
  let img = img.flipv();
  let format = match img {
    ImageLuma8(_) => gl::RED,
    ImageLumaA8(_) => gl::RG,
    ImageRgb8(_) => gl::RGB,
    ImageRgba8(_) => gl::RGBA,
  };

  let data = img.raw_pixels();

  gl::BindTexture(gl::TEXTURE_2D, textureID);
  gl::TexImage2D(
    gl::TEXTURE_2D,
    0,
    format as i32,
    img.width() as i32,
    img.height() as i32,
    0,
    format,
    gl::UNSIGNED_BYTE,
    &data[0] as *const u8 as *const c_void,
  );
  gl::GenerateMipmap(gl::TEXTURE_2D);

  gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
  gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
  gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
  gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

  textureID
}
