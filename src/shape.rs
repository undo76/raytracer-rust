use core::fmt::Debug;
use crate::geom::*;
use crate::ray::*;
use crate::transform::*;
use nalgebra as na;

#[derive(Debug)]
pub struct Sphere {
  transform: na::Projective3<f32>
}

#[inline]
pub fn sphere() -> Sphere {
  Sphere {
    transform: na::Projective3::identity()
  }
}

#[derive(Debug)]
pub struct Intersection<'a> {
  pub t: f32,
  pub object: &'a Shape,
}

pub type Intersections<'a> = Vec<Intersection<'a>>;

pub trait Shape: Debug {
  fn intersects(&self, ray: &Ray) -> Intersections;
  fn set_transform(&mut self, trans: na::Projective3<f32>);
}

/// Returns the closest, not negative intersection
pub fn hit<'a>(xs: &'a Intersections) -> Option<&'a Intersection<'a>> {
  xs.iter()
    .filter(|&x| x.t > 0.)
    .min_by(|&x, &y| x.t.partial_cmp(&y.t).unwrap())
}

impl Shape for Sphere {
  fn intersects(&self, ray: &Ray) -> Intersections {

    // FIXME: Inverse transformation could be memoized
    let ray = ray.transform(&self.transform.inverse());

    let sphere_to_ray = ray.origin - point(0., 0., 0.);
    let a = dot(&ray.direction, &ray.direction);
    let b = 2. * dot(&ray.direction, &sphere_to_ray);
    let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.;
    let discriminant = b * b - 4. * a * c;
    if discriminant < 0. {
      return vec![];
    } else {
      let sqrt_disc = f32::sqrt(discriminant);
      let mut t1 = (-b - sqrt_disc) / (2. * a);
      let mut t2 = (-b + sqrt_disc) / (2. * a);
      if t1 > t2 {
        let aux = t2;
        t2 = t1;
        t1 = aux;
      }
      return vec![
        Intersection {
          t: t1,
          object: self,
        },
        Intersection {
          t: t2,
          object: self,
        },
      ];
    }
  }

  fn set_transform(&mut self, trans: na::Projective3<f32>) {
    self.transform = trans;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ray_intersects_sphere() {
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
    let s = sphere();
    let xs = s.intersects(&r);
    assert_eq!(xs.len(), 2);
  }

  #[test]
  fn ray_intersects_sphere_at_two_points() {
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
    let s = sphere();
    let xs = s.intersects(&r);
    assert_eq!(xs.len(), 2);
    assert_relative_eq!(xs[0].t, 4.);
    assert_relative_eq!(xs[1].t, 6.);
  }

  #[test]
  fn ray_intersects_sphere_at_a_tangent() {
    let r = ray(point(0., 1., -5.), vector(0., 0., 1.));
    let s = sphere();
    let xs = s.intersects(&r);
    assert_eq!(xs.len(), 2);
    assert_relative_eq!(xs[0].t, 5.);
    assert_relative_eq!(xs[1].t, 5.);
  }

  #[test]
  fn ray_misses_a_sphere() {
    let r = ray(point(0., 2., -5.), vector(0., 0., 1.));
    let s = sphere();
    let xs = s.intersects(&r);
    assert_eq!(xs.len(), 0);
  }

  #[test]
  fn ray_originates_inside_a_sphere() {
    let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
    let s = sphere();
    let xs = s.intersects(&r);
    assert_eq!(xs.len(), 2);
    assert_relative_eq!(xs[0].t, -1.);
    assert_relative_eq!(xs[1].t, 1.);
  }

  #[test]
  fn intesections() {
    let s1 = sphere();
    let s2 = sphere();
    let xs = vec![
      Intersection { t: -1., object: &s1 },
      Intersection { t: 1., object: &s2 },
      Intersection { t: 2., object: &s2 },
    ];
    let h = hit(&xs);
    assert!(std::ptr::eq(h.unwrap(), &xs[1]));
  }

  #[test]
  fn intesections_none() {
    let s1 = sphere();
    let s2 = sphere();
    let xs = vec![
      Intersection { t: -1., object: &s1 },
      Intersection { t: -2., object: &s2 },
      Intersection { t: -3., object: &s2 },
    ];
    let h = hit(&xs);
    assert!(h.is_none());
  }

  #[test]
  fn sphere_default_transformation() {
    let s = sphere();
    assert_eq!(s.transform, na::Projective3::identity())
  }

  #[test]
  fn sphere_transformation() {
    let mut s = sphere();
    s.set_transform(na::convert(translation(2., 3., 4.)));
    assert_eq!(s.transform, na::convert(translation(2., 3., 4.)))
  }

  #[test]
  fn intersect_a_scaled_sphere_with_a_ray() {
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
    let mut s = sphere();
    s.set_transform(na::convert(scaling(2., 2., 2.)));
    let xs = s.intersects(&r);
    assert_eq!(xs.len(), 2);
    assert_relative_eq!(xs[0].t, 3.);
    assert_relative_eq!(xs[1].t, 7.);
  }

  #[test]
  fn intersect_a_translated_sphere_with_a_ray() {
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
    let mut s = sphere();
    s.set_transform(na::convert(translation(5., 0., 0.)));
    let xs = s.intersects(&r);
    assert_eq!(xs.len(), 0);
  }
}
