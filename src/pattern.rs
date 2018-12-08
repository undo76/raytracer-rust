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

pub trait PatternMapping<T>
where T: Copy {
  fn get_transform_inverse(&self) -> &Transform;
  fn map_at(&self, pattern_point: &Point) -> T;
  fn map_at_object(&self, object: &Shape, world_point: &Point) -> T {
    let object_point = object.get_transform_inverse() * world_point;
    let pattern_point = self.get_transform_inverse() * object_point;
    self.map_at(&pattern_point)
  }
}

impl<T: Copy> PatternMapping<T> for StripePattern<T> {
  fn get_transform_inverse(&self) -> &Transform {
    &self.transform_inverse
  }
  fn map_at(&self, world_point: &Point) -> T {
    let n = self.values.len() as isize;
    let idx = (world_point.x.floor() as isize % n + n) % n ;
    self.values[idx as usize]
  }
}

#[derive(Debug, Clone)]
pub enum Pattern<T: Copy> {
  Uniform(UniformPattern<T>),
  Striped(StripePattern<T>),
}
impl<T: Copy> Pattern<T> {
  pub fn map_at_object(&self, object: &Shape, world_point: &Point) -> T {
    use self::Pattern::*;
    match self {
      Uniform(u) => u.value,
      Striped(s) => s.map_at_object(object, &world_point),
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
    let pattern: Pattern<ColorRgbFloat> = Pattern::Striped(StripePattern { values, transform_inverse });
    let sphere = Sphere::default();
    assert_eq!(pattern.map_at_object(&sphere, &point(0., 0., 0.)), WHITE);
    assert_eq!(pattern.map_at_object(&sphere, &point(0., 0., 2.)), WHITE);
    assert_eq!(pattern.map_at_object(&sphere, &point(0., 0., 3.)), WHITE);
  }

  #[test]
  fn stripe_pattern_alternates_in_z() {
    let values = vec![WHITE, BLACK];
    let transform_inverse = Transform::identity();
    let pattern: Pattern<ColorRgbFloat> = Pattern::Striped(StripePattern { values, transform_inverse });
    let sphere = Sphere::default();
    assert_eq!(pattern.map_at_object(&sphere, &point(0., 0., 0.)), WHITE);
    assert_eq!(pattern.map_at_object(&sphere, &point(0.9, 0., 0.)), WHITE);
    assert_eq!(pattern.map_at_object(&sphere, &point(1., 0., 0.)), BLACK);
    assert_eq!(pattern.map_at_object(&sphere, &point(-0.1, 0., 0.)), BLACK);
    assert_eq!(pattern.map_at_object(&sphere, &point(-1., 0., 0.)), BLACK);
    assert_eq!(pattern.map_at_object(&sphere, &point(-1.1, 0., 0.)), WHITE);
  }
}
