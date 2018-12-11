use crate::*;

#[derive(Debug)]
pub struct PointLight {
  pub position: Point,
  pub intensity: ColorRgbFloat,
}

impl PointLight {
  pub fn new(position: Point, intensity: ColorRgbFloat) -> PointLight {
    PointLight {
      position,
      intensity,
    }
  }
}

#[cfg(tests)]
mod tests {
  use super::*;

  #[test]
  fn point_light_creation() {
    let light = PointLight::new(point(0., 0., 0.), WHITE);
    assert_eq!(light.position, point(0., 0., 0.,));
    assert_eq!(light.intesity, WHITE);
  }
}
