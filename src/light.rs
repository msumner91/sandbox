use cgmath::Vector3;

pub struct Light {
  pub position: Vector3<f32>,
  pub colour: Vector3<f32>,
  pub attenuation: Vector3<f32>
}

impl Light {
  pub fn new(position: Vector3<f32>, colour: Vector3<f32>, attenuation: Vector3<f32>, brightness: f32) -> Light {
    Light { position, colour: (colour * brightness), attenuation }
  }
}