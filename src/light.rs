use crate::color::*;
use crate::geom::*;

pub struct PointLight {
  pub position: Point,
  pub intensity: ColorRgbFloat,
}

pub fn point_light(position: Point, intensity: ColorRgbFloat) -> PointLight {
  PointLight {
    position,
    intensity,
  }
}

#[cfg(tests)]
mod tests {
  use super::*;

  #[test]
  fn point_light_creation() {
    let light = point_light(point(0., 0., 0.), WHITE);
    assert_eq!(light.position, point(0., 0., 0.,));
    assert_eq!(light.intesity, WHITE);
  }
}