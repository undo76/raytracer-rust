use crate::*;

#[derive(Debug, Clone)]
pub struct UniformPattern<T> {
  pub value: T,
}

#[derive(Debug, Clone)]
pub struct StripePattern<T> {
  pub values: Vec<T>,
  pub transform_inverse: Transform,
}

#[derive(Debug, Clone)]
pub struct CheckersPattern<T> {
  pub values: Vec<T>,
  pub transform_inverse: Transform,
}

#[derive(Debug, Clone)]
pub struct GradientPattern<T> {
  pub values: (T, T),
  pub transform_inverse: Transform,
}

#[derive(Debug, Clone)]
pub struct RingPattern<T> {
  pub values: Vec<T>,
  pub transform_inverse: Transform,
}

pub trait PatternMapping<T>
where
  T: Copy,
{
  fn get_transform_inverse(&self) -> &Transform;
  fn map_at(&self, pattern_point: &Point) -> T;
  fn map_at_object(&self, object: &Shape, world_point: &Point) -> T {
    let object_point = object.get_transform_inverse() * world_point;
    let pattern_point = self.get_transform_inverse() * object_point;
    self.map_at(&pattern_point)
  }
}

impl<T> PatternMapping<T> for StripePattern<T>
where
  T: Copy,
{
  fn get_transform_inverse(&self) -> &Transform {
    &self.transform_inverse
  }
  fn map_at(&self, pattern_point: &Point) -> T {
    let n = self.values.len() as isize;
    let idx = (pattern_point.x.floor() as isize % n + n) % n;
    self.values[idx as usize]
  }
}

impl<T> PatternMapping<T> for CheckersPattern<T>
where
  T: Copy,
{
  fn get_transform_inverse(&self) -> &Transform {
    &self.transform_inverse
  }
  fn map_at(&self, pattern_point: &Point) -> T {
    let n = self.values.len() as isize;
    let idx_x = pattern_point.x.round().floor() as isize % n + n;
    let idx_y = pattern_point.y.round().floor() as isize % n + n;
    let idx_z = pattern_point.z.round().floor() as isize % n + n;
    let idx = (idx_x + idx_y + idx_z) % n;
    self.values[idx as usize]
  }
}

impl<T> PatternMapping<T> for GradientPattern<T>
where
  T: Copy
    + core::ops::Sub<Output = T>
    + core::ops::Add<Output = T>
    + core::ops::Mul<f32, Output = T>,
{
  fn get_transform_inverse(&self) -> &Transform {
    &self.transform_inverse
  }
  fn map_at(&self, pattern_point: &Point) -> T {
    let distance = self.values.1 - self.values.0;
    let fraction = pattern_point.x - pattern_point.x.floor();
    self.values.0 + distance * fraction
  }
}

impl<T> PatternMapping<T> for RingPattern<T>
where
  T: Copy
    + core::ops::Sub<Output = T>
    + core::ops::Add<Output = T>
    + core::ops::Mul<f32, Output = T>,
{
  fn get_transform_inverse(&self) -> &Transform {
    &self.transform_inverse
  }
  fn map_at(&self, pattern_point: &Point) -> T {
    let (x, z) = (pattern_point.x, pattern_point.z);
    let n = self.values.len() as isize;
    let distance = (x * x + z * z).sqrt().floor();
    let idx = distance as isize % n;
    self.values[idx as usize]
  }
}

#[derive(Debug, Clone)]
pub enum Pattern<T: Copy> {
  Uniform(UniformPattern<T>),
  Striped(StripePattern<T>),
  Gradient(GradientPattern<T>),
  Ring(RingPattern<T>),
  Checkered(CheckersPattern<T>),
}
impl<T> Pattern<T>
where
  T: Copy
    + core::ops::Sub<Output = T>
    + core::ops::Add<Output = T>
    + core::ops::Mul<f32, Output = T>,
{
  pub fn map_at_object(&self, object: &Shape, world_point: &Point) -> T {
    use self::Pattern::*;
    match self {
      Uniform(u) => u.value,
      Striped(s) => s.map_at_object(object, &world_point),
      Gradient(g) => g.map_at_object(object, &world_point),
      Ring(r) => r.map_at_object(object, &world_point),
      Checkered(c) => c.map_at_object(object, &world_point),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn stripe_pattern_is_constant_in_z() {
    let values = vec![WHITE, BLACK];
    let transform_inverse = Transform::identity();
    let pattern: Pattern<ColorRgbFloat> = Pattern::Striped(StripePattern {
      values,
      transform_inverse,
    });
    let sphere = Sphere::default();
    assert_eq!(pattern.map_at_object(&sphere, &point(0., 0., 0.)), WHITE);
    assert_eq!(pattern.map_at_object(&sphere, &point(0., 0., 2.)), WHITE);
    assert_eq!(pattern.map_at_object(&sphere, &point(0., 0., 3.)), WHITE);
  }

  #[test]
  fn stripe_pattern_alternates_in_z() {
    let values = vec![WHITE, BLACK];
    let transform_inverse = Transform::identity();
    let pattern: Pattern<ColorRgbFloat> = Pattern::Striped(StripePattern {
      values,
      transform_inverse,
    });
    let sphere = Sphere::default();
    assert_eq!(pattern.map_at_object(&sphere, &point(0., 0., 0.)), WHITE);
    assert_eq!(pattern.map_at_object(&sphere, &point(0.9, 0., 0.)), WHITE);
    assert_eq!(pattern.map_at_object(&sphere, &point(1., 0., 0.)), BLACK);
    assert_eq!(pattern.map_at_object(&sphere, &point(-0.1, 0., 0.)), BLACK);
    assert_eq!(pattern.map_at_object(&sphere, &point(-1., 0., 0.)), BLACK);
    assert_eq!(pattern.map_at_object(&sphere, &point(-1.1, 0., 0.)), WHITE);
  }
}
