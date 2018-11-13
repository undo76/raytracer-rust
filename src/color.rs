use std::ops::{Add, Mul, Sub};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Color {
  r: f32,
  g: f32,
  b: f32,
}

#[inline]
pub fn color(r: f32, g: f32, b: f32) -> Color {
  Color { r, g, b }
}

impl Mul<Color> for Color {
  type Output = Self;

  // Haddamard product
  fn mul(self, rhs: Self) -> Self {
    color(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
  }
}

impl Mul<f32> for Color {
  type Output = Self;
  fn mul(self, rhs: f32) -> Self {
    color(self.r * rhs, self.g * rhs, self.b * rhs)
  }
}

impl Add for Color {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    color(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
  }
}

impl Sub for Color {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    color(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn adding_colors() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 + c2, color(0.9 + 0.7, 0.6 + 0.1, 0.75 + 0.25));
  }

  #[test]
  fn substracting_colors() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 - c2, color(0.9 - 0.7, 0.6 - 0.1, 0.75 - 0.25));
  }

  #[test]
  fn multiplying_colors() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 * c2, color(0.9 * 0.7, 0.6 * 0.1, 0.75 * 0.25));
  }

  #[test]
  fn multiplying_colors_by_a_scalar() {
    let c1 = color(0.9, 0.6, 0.75);
    let s = 2.;
    assert_eq!(c1 * s, color(0.9 * s, 0.6 * s, 0.75 * s));
  }
}
