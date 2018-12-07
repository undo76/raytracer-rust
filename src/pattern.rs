use crate::*;

#[derive(Debug, Copy, Clone)]
pub enum Pattern {
  Uniform(ColorRgbFloat),
  Striped(ColorRgbFloat, ColorRgbFloat),
}
impl Pattern {
  pub fn color_at(&self, point: &Point) -> ColorRgbFloat {
    use self::Pattern::*;
    match &self {
      Uniform(c) => *c,
      Striped(c1, c2) => {
        if point.x.floor() as i32 % 2 == 0 {
          *c1
        } else {
          *c2
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn stripe_pattern_is_constant_in_z() {
    let pattern = Pattern::Striped(WHITE, BLACK);
    assert_eq!(pattern.color_at(&point(0., 0., 0.)), WHITE);
    assert_eq!(pattern.color_at(&point(0., 0., 2.)), WHITE);
    assert_eq!(pattern.color_at(&point(0., 0., 3.)), WHITE);
  }

  #[test]
  fn stripe_pattern_alternates_in_z() {
    let pattern = Pattern::Striped(WHITE, BLACK);
    assert_eq!(pattern.color_at(&point(0., 0., 0.)), WHITE);
    assert_eq!(pattern.color_at(&point(0.9, 0., 0.)), WHITE);
    assert_eq!(pattern.color_at(&point(1., 0., 0.)), BLACK);
    assert_eq!(pattern.color_at(&point(-0.1, 0., 0.)), BLACK);
    assert_eq!(pattern.color_at(&point(-1., 0., 0.)), BLACK);
    assert_eq!(pattern.color_at(&point(-1.1, 0., 0.)), WHITE);
  }
}
