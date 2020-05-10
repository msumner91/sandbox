use crate::types::Vector3;

pub struct Light {
  pub position: Vector3,
  pub colour: Vector3,
  pub attenuation: Vector3
}

impl Light {
  pub fn new(position: Vector3, colour: Vector3, attenuation: Vector3, brightness: f32) -> Light {
    Light { position, colour: (colour * brightness), attenuation }
  }
}