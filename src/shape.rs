use crate::*;

#[derive(Debug)]
pub struct Sphere {
  transform_inverse: Transform,
  material: Material,
}

impl Sphere {
  pub fn new(transform: Transform, material: Material) -> Sphere {
    Sphere {
      transform_inverse: transform.inverse(),
      material,
    }
  }
}

impl Default for Sphere {
  fn default() -> Sphere {
    Sphere::new(Transform::identity(), Material::default())
  }
}

#[derive(Debug)]
pub struct Hit<'a> {
  pub intersection: &'a Intersection<'a>,
  pub point: Point,
  pub eyev: UnitVector,
  pub normalv: UnitVector,
  pub inside: bool,
}

#[derive(Debug)]
pub struct Intersection<'a> {
  pub t: f32,
  pub object: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
  pub fn new(t: f32, object: &'a dyn Shape) -> Intersection<'a> {
    Intersection { t, object }
  }

  pub fn prepare_hit(&self, ray: &Ray) -> Hit {
    let point = ray.position(self.t);
    let eyev = UnitVector::new_normalize(-ray.direction);
    let normalv = self.object.normal_at(&point);
    let inside = dot(&normalv, &eyev) < 0.;
    Hit {
      intersection: &self,
      point,
      eyev,
      inside,
      normalv: if inside { -normalv } else { normalv },
    }
  }
}

pub type Intersections<'a> = Vec<Intersection<'a>>;

/// Returns the closest, not negative intersection
pub fn hit<'a>(xs: &'a Intersections) -> Option<&'a Intersection<'a>> {
  xs.iter()
    .filter(|&x| x.t > 0.)
    .min_by(|&x, &y| x.t.partial_cmp(&y.t).unwrap())
}

pub trait Shape: core::fmt::Debug {
  fn intersects(&self, ray: &Ray) -> Option<Intersections>;
  fn set_color(&mut self, color: ColorRgbFloat);
  fn get_color(&self) -> ColorRgbFloat;
  fn get_material(&self) -> &Material;
  fn set_transform(&mut self, trans: Transform);
  fn get_transform(&self) -> Transform;
  fn get_transform_inverse(&self) -> &Transform;
  fn normal_at(&self, p: &Point) -> UnitVector;
}

impl Shape for Sphere {
  fn normal_at(&self, p: &Point) -> UnitVector {
    let t_inv = &self.get_transform_inverse();
    let object_point = *t_inv * p;
    let object_normal = object_point - point(0., 0., 0.);
    let mut world_normal = (*t_inv).matrix().transpose() * object_normal.to_homogeneous();
    world_normal[3] = 0.;
    UnitVector::new_normalize(Vector::from_homogeneous(world_normal).unwrap())
  }

  fn intersects(&self, ray: &Ray) -> Option<Intersections> {
    let ray = ray.transform(self.get_transform_inverse());
    let sphere_to_ray = ray.origin - point(0., 0., 0.);
    let a = dot(&ray.direction, &ray.direction);
    let b = 2. * dot(&ray.direction, &sphere_to_ray);
    let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.;
    let discriminant = b * b - 4. * a * c;
    if discriminant < 0. {
      return None;
    } else {
      let sqrt_disc = f32::sqrt(discriminant);
      let mut t1 = (-b - sqrt_disc) / (2. * a);
      let mut t2 = (-b + sqrt_disc) / (2. * a);
      if t1 > t2 {
        let aux = t2;
        t2 = t1;
        t1 = aux;
      }
      return Some(vec![Intersection::new(t1, self), Intersection::new(t2, self)]);
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

  fn set_transform(&mut self, trans: Transform) {
    self.transform_inverse = trans.inverse();
  }

  fn get_transform(&self) -> Transform {
    self.transform_inverse.inverse()
  }

  fn get_transform_inverse(&self) -> &Transform {
    &self.transform_inverse
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use nalgebra as na;

  #[test]
  fn ray_intersects_sphere_default() {
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let s = Sphere::default();
    let xs = s.intersects(&r).unwrap();
    assert_eq!(xs.len(), 2);
  }

  #[test]
  fn ray_intersects_sphere_at_two_points() {
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let s = Sphere::default();
    let xs = s.intersects(&r).unwrap();
    assert_eq!(xs.len(), 2);
    assert_relative_eq!(xs[0].t, 4.);
    assert_relative_eq!(xs[1].t, 6.);
  }

  #[test]
  fn ray_intersects_sphere_at_a_tangent() {
    let r = Ray::new(point(0., 1., -5.), vector(0., 0., 1.));
    let s = Sphere::default();
    let xs = s.intersects(&r).unwrap();
    assert_eq!(xs.len(), 2);
    assert_relative_eq!(xs[0].t, 5.);
    assert_relative_eq!(xs[1].t, 5.);
  }

  #[test]
  fn ray_misses_a_sphere() {
    let r = Ray::new(point(0., 2., -5.), vector(0., 0., 1.));
    let s = Sphere::default();
    let xs = s.intersects(&r);
    assert!(xs.is_none());
  }

  #[test]
  fn ray_originates_inside_a_sphere() {
    let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
    let s = Sphere::default();
    let xs = s.intersects(&r).unwrap();
    assert_eq!(xs.len(), 2);
    assert_relative_eq!(xs[0].t, -1.);
    assert_relative_eq!(xs[1].t, 1.);
  }

  #[test]
  fn intesections() {
    let s1 = Sphere::default();
    let s2 = Sphere::default();
    let xs = vec![
      Intersection::new(-1., &s1),
      Intersection::new(1., &s2),
      Intersection::new(2., &s2),
    ];
    let h = hit(&xs);
    assert!(std::ptr::eq(h.unwrap(), &xs[1]));
  }

  #[test]
  fn intesections_none() {
    let s1 = Sphere::default();
    let s2 = Sphere::default();
    let xs = vec![
      Intersection::new(-1., &s1),
      Intersection::new(-2., &s2),
      Intersection::new(-3., &s2),
    ];
    let h = hit(&xs);
    assert!(h.is_none());
  }

  #[test]
  fn sphere_default_transform() {
    let s = Sphere::default();
    assert_eq!(s.get_transform(), Transform::identity())
  }

  #[test]
  fn sphere_transform() {
    let mut s = Sphere::default();
    s.set_transform(na::convert(translation(2., 3., 4.)));
    assert_eq!(s.get_transform(), na::convert(translation(2., 3., 4.)))
  }

  #[test]
  fn intersect_a_scaled_sphere_with_a_ray() {
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let mut s = Sphere::default();
    s.set_transform(na::convert(scaling(2., 2., 2.)));
    let xs = s.intersects(&r).unwrap();
    assert_eq!(xs.len(), 2);
    assert_relative_eq!(xs[0].t, 3.);
    assert_relative_eq!(xs[1].t, 7.);
  }

  #[test]
  fn intersect_a_translated_sphere_with_a_ray() {
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let mut s = Sphere::default();
    s.set_transform(na::convert(translation(5., 0., 0.)));
    let xs = s.intersects(&r);
    assert!(xs.is_none());
  }

  #[test]
  fn normal_sphere_axis() {
    let s = Sphere::default();
    let n = s.normal_at(&point(1., 0., 0.));
    assert_relative_eq!(n.unwrap(), vector(1., 0., 0.));
    let n = s.normal_at(&point(0., 1., 0.));
    assert_relative_eq!(n.unwrap(), vector(0., 1., 0.));
    let n = s.normal_at(&point(0., 0., 1.));
    assert_relative_eq!(n.unwrap(), vector(0., 0., 1.));
  }

  #[test]
  fn normal_sphere_translated() {
    let mut s = Sphere::default();
    s.set_transform(na::convert(translation(0., 1., 0.)));
    let n = s.normal_at(&point(0., 1.70710677, -0.70710677));
    assert_relative_eq!(n.unwrap(), vector(0., 0.70710677, -0.70710677));
  }

  #[test]
  fn normal_sphere_scaled() {
    let mut s = Sphere::default();
    s.set_transform(na::convert(scaling(1., 0.5, 1.)));
    let n = s.normal_at(&point(0., 0.70710677, -0.70710677));
    assert_relative_eq!(n.unwrap(), vector(0., 0.97014254, -0.24253564));
  }

  #[test]
  fn assign_material() {
    let mut s = Sphere::default();
    let mut m = Material::default();
    m.ambient = 1.;
    s.material = m;
    assert_eq!(s.material.ambient, 1.);
  }

  #[test]
  fn precompute_state_of_intersection() {
    let ray = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let shape = Sphere::default();
    let intersection = Intersection::new(4., &shape);
    let hit = intersection.prepare_hit(&ray);
    assert_relative_eq!(hit.point, point(0., 0., -1.));
    assert_relative_eq!(hit.eyev.unwrap(), vector(0., 0., -1.));
    assert_relative_eq!(hit.normalv.unwrap(), vector(0., 0., -1.));
  }

  #[test]
  fn precompute_state_of_intersection_inside() {
    let ray = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
    let shape = Sphere::default();
    let intersection = Intersection::new(1., &shape);
    let hit = intersection.prepare_hit(&ray);
    assert_relative_eq!(hit.point, point(0., 0., 1.));
    assert_relative_eq!(hit.eyev.unwrap(), vector(0., 0., -1.));
    assert_relative_eq!(hit.normalv.unwrap(), vector(0., 0., -1.));
    assert_eq!(hit.inside, true);
  }
}
