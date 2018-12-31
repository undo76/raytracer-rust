use approx::{AbsDiffEq, RelativeEq};
use std::ops::{Add, Mul, Sub};

const DEFAULT_GAMMA: f32 = 2.2;

type Byte = u8;

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

/// Common colors
pub const RED: ColorRgbFloat = color(1., 0., 0.);
pub const GREEN: ColorRgbFloat = color(0., 1., 0.);
pub const BLUE: ColorRgbFloat = color(0., 0., 1.);
pub const YELLOW: ColorRgbFloat = color(1., 1., 0.);
pub const PURPLE: ColorRgbFloat = color(1., 0., 1.);
pub const BLACK: ColorRgbFloat = color(0., 0., 0.);
pub const WHITE: ColorRgbFloat = color(1., 1., 1.);

#[inline]
fn clamp(x: f32) -> f32 {
    let x_gamma = x.powf(1. / DEFAULT_GAMMA);
    1.0_f32.min(0.0_f32.max(x_gamma))
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
            b: to_byte(self.b),
        }
    }
}

#[inline]
pub const fn color(r: f32, g: f32, b: f32) -> ColorRgbFloat {
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

impl std::iter::Sum for ColorRgbFloat {
    fn sum<I: Iterator<Item = ColorRgbFloat>>(iter: I) -> Self {
        iter.fold(BLACK, |acc, c| acc + c)
    }
}

impl AbsDiffEq for ColorRgbFloat {
    type Epsilon = f32;

    fn default_epsilon() -> f32 {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &ColorRgbFloat, epsilon: f32) -> bool {
        f32::abs_diff_eq(&self.r, &other.r, epsilon)
            && f32::abs_diff_eq(&self.g, &other.g, epsilon)
            && f32::abs_diff_eq(&self.b, &other.b, epsilon)
    }
}

impl RelativeEq for ColorRgbFloat {
    fn default_max_relative() -> f32 {
        f32::default_max_relative()
    }

    fn relative_eq(&self, other: &ColorRgbFloat, epsilon: f32, max_relative: f32) -> bool {
        f32::relative_eq(&self.r, &other.r, epsilon, max_relative)
            && f32::relative_eq(&self.g, &other.g, epsilon, max_relative)
            && f32::relative_eq(&self.b, &other.b, epsilon, max_relative)
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
