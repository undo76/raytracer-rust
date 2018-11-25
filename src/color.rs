use std::ops::{Add, Mul, Sub};
use approx::{AbsDiffEq, RelativeEq};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ColorRgbFloat {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ColorRgbByte {
  pub r: Byte,
  pub g: Byte,
  pub b: Byte,
}

type Byte = u8;

#[inline]
fn clamp(x: f32) -> f32 {
  1.0_f32.min(0.0_f32.max(x))
}

#[inline]
fn to_byte(x: f32) -> Byte {
  (clamp(x) * 255.).round() as Byte
}

impl Into<ColorRgbByte> for ColorRgbFloat {
  fn into(self) -> ColorRgbByte {
    ColorRgbByte {
      r: to_byte(self.r),
      g: to_byte(self.g),
      b: to_byte(self.b)
    }
  }
}

#[inline]
pub fn color(r: f32, g: f32, b: f32) -> ColorRgbFloat {
  ColorRgbFloat { r, g, b }
}

impl Mul<ColorRgbFloat> for ColorRgbFloat {
  type Output = Self;

  // Haddamard product
  fn mul(self, rhs: Self) -> Self {
    color(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
  }
}

impl Mul<f32> for ColorRgbFloat {
  type Output = Self;
  fn mul(self, rhs: f32) -> Self {
    color(self.r * rhs, self.g * rhs, self.b * rhs)
  }
}

impl Add for ColorRgbFloat {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    color(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
  }
}

impl Sub for ColorRgbFloat {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    color(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
  }
}


impl AbsDiffEq for ColorRgbFloat{
  type Epsilon = f32;

  #[inline]
  fn default_epsilon() -> f32 {
      f32::default_epsilon()
  }

  #[inline]
  fn abs_diff_eq(&self, other: &ColorRgbFloat, epsilon: f32) -> bool {
      f32::abs_diff_eq(&self.r, &other.r, epsilon.clone())
          && f32::abs_diff_eq(&self.g, &other.g, epsilon.clone())
          && f32::abs_diff_eq(&self.b, &other.b, epsilon.clone())
  }
}

impl RelativeEq for ColorRgbFloat {
  #[inline]
  fn default_max_relative() -> f32 {
      f32::default_max_relative()
  }

  #[inline]
  fn relative_eq(
      &self,
      other: &ColorRgbFloat,
      epsilon: f32,
      max_relative: f32,
  ) -> bool {
      f32::relative_eq(&self.r, &other.r, epsilon.clone(), max_relative.clone())
          && f32::relative_eq(&self.g, &other.g, epsilon.clone(), max_relative.clone())
          && f32::relative_eq(&self.b, &other.b, epsilon.clone(), max_relative.clone())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn adding_colors() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_relative_eq!(c1 + c2, color(0.9 + 0.7, 0.6 + 0.1, 0.75 + 0.25));
  }

  #[test]
  fn substracting_colors() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_relative_eq!(c1 - c2, color(0.9 - 0.7, 0.6 - 0.1, 0.75 - 0.25));
  }

  #[test]
  fn multiplying_colors() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_relative_eq!(c1 * c2, color(0.9 * 0.7, 0.6 * 0.1, 0.75 * 0.25));
  }

  #[test]
  fn multiplying_colors_by_a_scalar() {
    let c1 = color(0.9, 0.6, 0.75);
    let s = 2.;
    assert_relative_eq!(c1 * s, color(0.9 * s, 0.6 * s, 0.75 * s));
  }
}
