use crate::*;
use core::fmt::Debug;
use nalgebra as na;

#[derive(Debug)]
pub struct Sphere {
  transform: na::Projective3<f32>,
  transform_inverse: na::Projective3<f32>,
  material: Material,
}

#[inline]
pub fn sphere() -> Sphere {
  Sphere {
    transform: na::Projective3::identity(),
    transform_inverse: na::Projective3::identity(),
    material: material()
  }
}

#[derive(Debug)]
pub struct Intersection<'a> {
  pub t: f32,
  pub object: &'a Shape,
}

pub type Intersections<'a> = Vec<Intersection<'a>>;

/// Returns the closest, not negative intersection
pub fn hit<'a>(xs: &'a Intersections) -> Option<&'a Intersection<'a>> {
  xs.iter()
    .filter(|&x| x.t > 0.)
    .min_by(|&x, &y| x.t.partial_cmp(&y.t).unwrap())
}

pub trait Shape: Debug {
  fn intersects(&self, ray: &Ray) -> Intersections;
  fn set_color(&mut self, color: ColorRgbFloat);
  fn get_color(&self) -> ColorRgbFloat;
  fn get_material(&self) -> &Material;
  fn set_transform(&mut self, trans: na::Projective3<f32>);
  fn get_transform(&self) -> &na::Projective3<f32>;
  fn get_transform_inverse(&self) -> &na::Projective3<f32>;
  fn normal_at(&self, p: &Point) -> na::Unit<Vector>;
}

impl Shape for Sphere {
  fn normal_at(&self, p: &Point) -> na::Unit<Vector> {
    let t_inv = &self.get_transform_inverse();
    let object_point = *t_inv * p;
    let object_normal = object_point - point(0., 0., 0.);
    let mut world_normal = (*t_inv).matrix().transpose() * object_normal.to_homogeneous();
    world_normal[3] = 0.;
    na::Unit::new_normalize(Vector::from_homogeneous(world_normal).unwrap())
  }

  fn intersects(&self, ray: &Ray) -> Intersections {
    let ray = ray.transform(self.get_transform_inverse());
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

  fn set_color(&mut self, color: ColorRgbFloat) {
    self.material.color = color;
  }

  fn get_color(&self) -> ColorRgbFloat {
    self.material.color
  }

  fn get_material(&self) -> &Material {
    &self.material
  }

  fn set_transform(&mut self, trans: na::Projective3<f32>) {
    self.transform = trans;
    self.transform_inverse = trans.inverse();
  }

  fn get_transform(&self) -> &na::Projective3<f32> {
    &self.transform
  }

  fn get_transform_inverse(&self) -> &na::Projective3<f32> {
    &self.transform_inverse
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
      Intersection {
        t: -1.,
        object: &s1,
      },
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
      Intersection {
        t: -1.,
        object: &s1,
      },
      Intersection {
        t: -2.,
        object: &s2,
      },
      Intersection {
        t: -3.,
        object: &s2,
      },
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

  #[test]
  fn normal_sphere_axis() {
    let s = sphere();
    let n = s.normal_at(&point(1., 0., 0.));
    assert_relative_eq!(n.unwrap(), vector(1., 0., 0.));
    let n = s.normal_at(&point(0., 1., 0.));
    assert_relative_eq!(n.unwrap(), vector(0., 1., 0.));
    let n = s.normal_at(&point(0., 0., 1.));
    assert_relative_eq!(n.unwrap(), vector(0., 0., 1.));
  }

  #[test]
  fn normal_sphere_translated() {
    let mut s = sphere();
    s.set_transform(na::convert(translation(0., 1., 0.)));
    let n = s.normal_at(&point(0., 1.70710677, -0.70710677));
    assert_relative_eq!(n.unwrap(), vector(0., 0.70710677, -0.70710677));
  }

  #[test]
  fn normal_sphere_scaled() {
    let mut s = sphere();
    s.set_transform(na::convert(scaling(1., 0.5, 1.)));
    let n = s.normal_at(&point(0., 0.70710677, -0.70710677));
    assert_relative_eq!(n.unwrap(), vector(0., 0.97014254, -0.24253564));
  }

  #[test]
  fn assign_material() {
      let mut s = sphere();
      let mut m = material();
      m.ambient = 1.;
      s.material = m;
      assert_eq!(s.material.ambient, 1.);
  }
}
