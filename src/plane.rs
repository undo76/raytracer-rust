use crate::*;

#[derive(Debug)]
pub struct Plane {
  base: BaseShape,
}

impl Plane {
  pub fn new(transform: Transform, material: Material) -> Plane {
    Plane {
      base: BaseShape::new(transform, material),
    }
  }
}

impl Default for Plane {
  fn default() -> Plane {
    Plane::new(Transform::identity(), Material::default())
  }
}

impl Shape for Plane {
  fn get_base(&self) -> &BaseShape {
    &self.base
  }

  fn get_base_mut(&mut self) -> &mut BaseShape {
    &mut self.base
  }

  fn local_normal_at(&self, _local_point: &Point) -> Vector {
    vector(0., 1., 0.)
  }

  fn local_intersects(&self, ray: &Ray) -> Option<Intersection> {
    if f32::abs(ray.direction.y) < core::f32::EPSILON {
      None
    } else {
      let t = -ray.origin.y / ray.direction.y;
      if t > 1.0e-4{
        Some(Intersection::new(t, self))
      } else {
        None
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn intersect_parallel() {
    let p = Plane::default();
    let r = Ray::new(point(0., 10., 0.), vector(0., 0., 1.));
    let xs = p.local_intersects(&r);
    assert!(xs.is_none());
  }

  #[test]
  fn intersect_coplanar() {
    let p = Plane::default();
    let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
    let xs = p.local_intersects(&r);
    assert!(xs.is_none());
  }

  #[test]
  fn intersect_plane_above() {
    let p = Plane::default();
    let r = Ray::new(point(0., 1., 0.), vector(0., -1., 0.));
    let xs = p.local_intersects(&r).unwrap();
    assert_relative_eq!(xs.t, 1.);
  }

  #[test]
  fn intersect_plane_below() {
    let p = Plane::default();
    let r = Ray::new(point(0., -1., 0.), vector(0., 1., 0.));
    let xs = p.local_intersects(&r).unwrap();
    assert_relative_eq!(xs.t, 1.);
  }
}
