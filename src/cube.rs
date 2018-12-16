use crate::*;

#[derive(Debug)]
pub struct Cube {
  base: BaseShape,
}

impl Cube {
  pub fn new(transform: Transform, material: Material) -> Cube {
    Cube {
      base: BaseShape::new(transform, material),
    }
  }
}

impl Default for Cube {
  fn default() -> Cube {
    Cube::new(Transform::identity(), Material::default())
  }
}

impl Shape for Cube {
  fn get_base(&self) -> &BaseShape {
    &self.base
  }

  fn get_base_mut(&mut self) -> &mut BaseShape {
    &mut self.base
  }

  fn local_normal_at(&self, local_point: &Point) -> Vector {
    if local_point.x.abs() >= local_point.y.abs() {
      if local_point.x.abs() >= local_point.z.abs() {
        vector(local_point.x, 0., 0.)
      } else {
        vector(0., 0., local_point.z)
      }
    } else if local_point.y.abs() >= local_point.z.abs() {
      vector(0., local_point.y, 0.)
    } else {
      vector(0., 0., local_point.z)
    }
  }

  fn local_intersects(&self, ray: &Ray) -> Option<Intersection> {
    let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
    let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
    let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

    let tmin = xtmin.max(ytmin).max(ztmin);
    let tmax = xtmax.min(ytmax).min(ztmax);

    if tmin > tmax {
      None
    } else if tmin > 0.1 {
      Some(Intersection::new(tmin, self))
    } else if tmax > 0.1 {
      Some(Intersection::new(tmax, self))
    } else {
      None
    }
  }
}

fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
  let mut tmin;
  let mut tmax;
  let tmin_numerator = -1. - origin;
  let tmax_numerator = 1. - origin;

  if direction.abs() > std::f32::EPSILON {
    tmin = tmin_numerator / direction;
    tmax = tmax_numerator / direction;
  } else {
    tmin = tmin_numerator * std::f32::INFINITY;
    tmax = tmax_numerator * std::f32::INFINITY;
  }
  if tmin > tmax {
    std::mem::swap(&mut tmin, &mut tmax);
  }
  (tmin, tmax)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn intersect_cube() {
    let c = Cube::default();
    let r = Ray::new(point(5., 0.5, 0.), vector(-1., 0., 0.));
    let xs = c.local_intersects(&r).unwrap();
    assert_relative_eq!(xs.t, 4.);

    let r = Ray::new(point(-5., 0.5, 0.), vector(1., 0., 0.));
    let xs = c.local_intersects(&r).unwrap();
    assert_relative_eq!(xs.t, 4.);

    let r = Ray::new(point(0.5, 5., 0.), vector(0., -1., 0.));
    let xs = c.local_intersects(&r).unwrap();
    assert_relative_eq!(xs.t, 4.);

    let r = Ray::new(point(0.5, -5., 0.), vector(0., 1., 0.));
    let xs = c.local_intersects(&r).unwrap();
    assert_relative_eq!(xs.t, 4.);

    let r = Ray::new(point(0.5, 0., 5.), vector(0., 0., -1.));
    let xs = c.local_intersects(&r).unwrap();
    assert_relative_eq!(xs.t, 4.);

    let r = Ray::new(point(0.5, 0., -5.), vector(0., 0., 1.));
    let xs = c.local_intersects(&r).unwrap();
    assert_relative_eq!(xs.t, 4.);

    let r = Ray::new(point(0., 0.5, 0.), vector(0., 0., 1.));
    let xs = c.local_intersects(&r).unwrap();
    assert_relative_eq!(xs.t, 1.);
  }


  #[test]
  fn normals() {
      let c = Cube::default();

      let p = point(1., 0.5, -0.8);
      let n = c.local_normal_at(&p);
      assert_relative_eq!(n, vector(1., 0., 0.));

      let p = point(-1., -0.2, 0.9);
      let n = c.local_normal_at(&p);
      assert_relative_eq!(n, vector(-1., 0., 0.));

      let p = point(-0.4, 1., -0.1);
      let n = c.local_normal_at(&p);
      assert_relative_eq!(n, vector(0., 1., 0.));

      let p = point(0.3, -1., 0.);
      let n = c.local_normal_at(&p);
      assert_relative_eq!(n, vector(0., -1., 0.));

      let p = point(0.6, 0.3, 1.);
      let n = c.local_normal_at(&p);
      assert_relative_eq!(n, vector(0., 0., 1.));

      let p = point(0.4, 0.4, -1.);
      let n = c.local_normal_at(&p);
      assert_relative_eq!(n, vector(0., 0., -1.));

      let p = point(1., 1., 1.);
      let n = c.local_normal_at(&p);
      assert_relative_eq!(n, vector(1., 0., 0.));

      let p = point(-1., -1., -1.);
      let n = c.local_normal_at(&p);
      assert_relative_eq!(n, vector(-1., 0., 0.));      
  }
}
