use core::fmt::Debug;
use crate::*;

pub trait Pattern: Debug {
  fn color_at(&self, point: &Point) -> ColorRgbFloat;
}

#[derive(Debug)]
pub struct StripePattern {
  a: ColorRgbFloat,
  b: ColorRgbFloat,
}

impl StripePattern {
  pub fn new(a: ColorRgbFloat, b: ColorRgbFloat) -> StripePattern {
    StripePattern { a, b }
  }
}

impl Pattern for StripePattern {
  fn color_at(&self, point: &Point) -> ColorRgbFloat {
    if point.x.floor() as i32 % 2 == 0 {
      self.a
    } else {
      self.b
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn creating_a_stripe_pattern() {
    let pattern = StripePattern::new(WHITE, BLACK);
    assert_eq!(pattern.a, WHITE);
    assert_eq!(pattern.b, BLACK);
  }

  #[test]
  fn stripe_pattern_is_constant_in_z() {
    let pattern = StripePattern::new(WHITE, BLACK);
    assert_eq!(pattern.color_at(&point(0., 0., 0.)), WHITE);
    assert_eq!(pattern.color_at(&point(0., 0., 2.)), WHITE);
    assert_eq!(pattern.color_at(&point(0., 0., 3.)), WHITE);
  }

  #[test]
  fn stripe_pattern_alternates_in_z() {
    let pattern = StripePattern::new(WHITE, BLACK);
    assert_eq!(pattern.color_at(&point(0., 0., 0.)), WHITE);
    assert_eq!(pattern.color_at(&point(0.9, 0., 0.)), WHITE);
    assert_eq!(pattern.color_at(&point(1., 0., 0.)), BLACK);
    assert_eq!(pattern.color_at(&point(-0.1, 0., 0.)), BLACK);
    assert_eq!(pattern.color_at(&point(-1., 0., 0.)), BLACK);
    assert_eq!(pattern.color_at(&point(-1.1, 0., 0.)), WHITE);
  }
}
