#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate glfw;
use self::glfw::{Key, Action};
use gl;

use std::sync::mpsc::Receiver;
use std::time::SystemTime;

use crate::utils::camera::Camera;
use crate::utils::camera::Camera_Movement::*;

#[allow(dead_code)]
pub fn elapsed(start_time: &SystemTime) -> String {
    let elapsed = start_time.elapsed().unwrap();
    format!("{}s {:.*}ms", elapsed.as_secs(), 1, elapsed.subsec_nanos() as f64 / 1_000_000.0)
}

pub fn process_events(events: &Receiver<(f64, glfw::WindowEvent)>,
                      firstMouse: &mut bool,
                      lastX: &mut f32,
                      lastY: &mut f32,
                      camera: &mut Camera) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            },
            
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
            },

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