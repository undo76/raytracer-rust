use crate::geom::*;

pub struct Ray {
  pub origin: Point,
  pub direction: Vector,
}

impl Ray {
  pub fn position(&self, t: f32) -> Point {
    self.origin + self.direction * t
  }
}

pub fn ray(origin: Point, direction: Vector) -> Ray {
  Ray { origin, direction }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn creating_querying_ray() {
    let origin = point(1., 2., 3.);
    let direction = vector(4., 5., 6.);
    let r = ray(origin, direction);
    assert_eq!(r.origin, origin);
    assert_eq!(r.direction, direction);
  }

  #[test]
  fn point_from_a_distance() {
    let origin = point(2., 3., 4.);
    let direction = vector(1., 0., 0.);
    let r = ray(origin, direction);
    assert_relative_eq!(r.position(0.), origin);
    assert_relative_eq!(r.position(1.), point(3., 3., 4.));
    assert_relative_eq!(r.position(-1.), point(1., 3., 4.));
    assert_relative_eq!(r.position(2.5), point(4.5, 3., 4.));
  }
}
