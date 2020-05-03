#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate glfw;
use self::glfw::{Action, Context, Glfw, Key, Window, WindowEvent};
use gl;
use cgmath::{vec2, vec3};

use std::sync::mpsc::Receiver;
use std::path::Path;
use std::os::raw::c_void;
use rand::prelude::*;

use image;
use image::DynamicImage::*;
use image::GenericImage;

use super::camera::Camera;
use super::camera::Camera_Movement::*;
use super::mesh::{Mesh, Vertex, Texture};

const VERTEX_COUNT: u32 = 128;
const COUNT: usize = (VERTEX_COUNT * VERTEX_COUNT) as usize;
const ICOUNT: usize = (6 * (VERTEX_COUNT-1)*(VERTEX_COUNT-1)) as usize;
const SIZE: f32 = 800.0;

pub fn initGlfw() -> Glfw {
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
  #[cfg(target_os = "windows")]
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
  glfw
}

pub fn createAndInitWindow(glfw: Glfw, w: u32, h: u32) -> (Window, Receiver<(f64, WindowEvent)>) {
  let (mut window, events) = glfw
    .create_window(w, h, "LearnOpenGL", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window");
  window.make_current();
  window.set_framebuffer_size_polling(true);
  window.set_cursor_pos_polling(true);
  window.set_scroll_polling(true);
  window.set_cursor_mode(glfw::CursorMode::Disabled);
  (window, events)
}

pub fn initGl(window: &mut Window) -> () {
  unsafe {
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    gl::Enable(gl::DEPTH_TEST);
    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
  }
}

pub fn updateTimings(glfw: Glfw, deltaTime: &mut f32, lastFrame: &mut f32) -> () {
  let currentFrame = glfw.get_time() as f32;
  *deltaTime = currentFrame - *lastFrame;
  *lastFrame = currentFrame;
}

pub fn process_events(
  events: &Receiver<(f64, glfw::WindowEvent)>,
  firstMouse: &mut bool,
  lastX: &mut f32,
  lastY: &mut f32,
  camera: &mut Camera,
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

        camera.ProcessMouseMovement(xoffset, yoffset, true);
      }

      glfw::WindowEvent::Scroll(_xoffset, yoffset) => camera.ProcessMouseScroll(yoffset as f32),

      _ => {}
    }
  }
}

pub fn processInput(window: &mut glfw::Window, deltaTime: f32, camera: &mut Camera) {
  if window.get_key(Key::Escape) == Action::Press {
    window.set_should_close(true)
  }
  if window.get_key(Key::W) == Action::Press {
    camera.ProcessKeyboard(FORWARD, deltaTime);
  }
  if window.get_key(Key::S) == Action::Press {
    camera.ProcessKeyboard(BACKWARD, deltaTime);
  }
  if window.get_key(Key::A) == Action::Press {
    camera.ProcessKeyboard(LEFT, deltaTime);
  }
  if window.get_key(Key::D) == Action::Press {
    camera.ProcessKeyboard(RIGHT, deltaTime);
  }
}

pub fn genTerrain() -> Mesh {
  let mut vertices = [0.0; COUNT * 3];
  let mut normals = [0.0; COUNT * 3];
  let mut tCoords = [0.0; COUNT * 2];
  let mut indices = [0; ICOUNT]; 
  let mut vertexPointer = 0;

  let mut rng = rand::thread_rng();

  for i in 0..VERTEX_COUNT {
    for j in 0..VERTEX_COUNT {
      vertices[vertexPointer*3] = (j as f32)/((VERTEX_COUNT - 1) as f32) * SIZE;
      vertices[vertexPointer*3+1] = rng.gen_range(0.0, 2.0);
      vertices[vertexPointer*3+2] = (i as f32)/((VERTEX_COUNT - 1) as f32) * SIZE;

      normals[vertexPointer*3] = 0.0;
      normals[vertexPointer*3+1] = 1.0;
      normals[vertexPointer*3+2] = 0.0;

      tCoords[vertexPointer*2] = (j as f32)/((VERTEX_COUNT - 1) as f32);
      tCoords[vertexPointer*2+1] = (i as f32)/((VERTEX_COUNT - 1) as f32);

      vertexPointer += 1;
    }
  }

  let mut pointer = 0;
  for gz in 0..(VERTEX_COUNT-1) {
    for gx in 0..(VERTEX_COUNT-1) {
      let topLeft = (gz*VERTEX_COUNT)+gx;
      let topRight = topLeft + 1;
      let bottomLeft = ((gz+1)*VERTEX_COUNT)+gx;
      let bottomRight = bottomLeft + 1;
      indices[pointer] = topLeft;
      indices[pointer+1] = bottomLeft;
      indices[pointer+2] = topRight;
      indices[pointer+3] = topRight;
      indices[pointer+4] = bottomLeft;
      indices[pointer+5] = bottomRight;
      pointer += 6
    }
  }

  let mut vertexVec: Vec<Vertex> = Vec::with_capacity(COUNT);
  for i in 0..COUNT {
    vertexVec.push(
      Vertex {
        Position: vec3(vertices[i*3], vertices[i*3+1], vertices[i*3+2]), 
        Normal: vec3(normals[i*3], normals[i*3+1], normals[i*3+2]), 
        TexCoords: vec2(tCoords[i*2], tCoords[i*2+1]),
        ..Vertex::default()
      }
    )
  }

  let path = "grass2.jpg";
  let t = Texture { 
    id: unsafe { TextureFromFile(&path, "resources/textures") },
    type_: "texture_normal".into(),
    path: path.into(),
  };

  return Mesh::new(vertexVec, indices.to_vec(), vec![t]);
}

pub unsafe fn TextureFromFile(path: &str, directory: &str) -> u32 {
  let filename = format!("{}/{}", directory, path);

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
