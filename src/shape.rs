use crate::*;

#[derive(Debug)]
pub struct BaseShape {
  transform_inverse: Transform,
  material: Material,
}

impl BaseShape {
  pub fn new(transform: Transform, material: Material) -> BaseShape {
    BaseShape {
      transform_inverse: transform.inverse(),
      material,
    }
  }
}

#[derive(Debug)]
pub struct Hit<'a> {
  pub intersection: &'a Intersection<'a>,
  pub point: Point,
  pub eyev: UnitVector,
  pub normalv: UnitVector,
  pub inside: bool,
  pub reflectv: UnitVector,
  pub n1: f32,
  pub n2: f32,
}

impl Hit<'_> {
  pub fn schlick(&self) -> f32 {
    let mut cos = dot(&self.eyev, &self.normalv);

    if self.n1 > self.n2 {
      let n_ratio = self.n1 / self.n2;
      let sin2_t = n_ratio * n_ratio * (1. - cos * cos);
      if sin2_t > 1.0 {
        return 1.0;
      }

      cos = f32::sqrt(1.0 - sin2_t);
    }

    let r0 = (self.n1 - self.n2) / (self.n1 + self.n2);
    let r0 = r0 * r0;
    return r0 + (1. - r0) * f32::powi(1. - cos, 5);
  }
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
    let normalv = if inside { -normalv } else { normalv };
    let reflectv = UnitVector::new_unchecked(reflect(&ray.direction, &normalv));

    // Transparency
    let material = self.object.get_material();
    let (n1, n2) = if material.transparency.is_some() {
      // TODO: nested shapes. It only works in
      // vacuum-material interfaces.
      let n1 = 1.;
      let n2 = material.refractive_index;
      if inside {
        (n2, n1)
      } else {
        (n1, n2)
      }
    } else {
      (1., 1.)
    };

    Hit {
      intersection: &self,
      point,
      eyev,
      inside,
      normalv,
      reflectv,
      n1,
      n2,
    }
  }
}

// TODO: Remove
pub type Intersections<'a> = Vec<Intersection<'a>>;

// Returns the closest, not negative intersection
pub fn hit<'a>(xs: &'a Intersections) -> Option<&'a Intersection<'a>> {
  xs.iter()
    .filter(|&x| x.t > 0.)
    .min_by(|&x, &y| x.t.partial_cmp(&y.t).unwrap())
}

pub trait Shape: core::fmt::Debug + Sync + Send {
  fn get_base(&self) -> &BaseShape;
  fn get_base_mut(&mut self) -> &mut BaseShape;
  fn local_intersects(&self, local_ray: &Ray) -> Option<Intersection>;
  fn local_normal_at(&self, p: &Point) -> Vector;

  fn intersects(&self, ray: &Ray) -> Option<Intersection> {
    let local_ray = ray.transform(self.get_transform_inverse());
    self.local_intersects(&local_ray)
  }

  fn normal_at(&self, p: &Point) -> UnitVector {
    let t_inv = &self.get_transform_inverse();
    let local_point = *t_inv * p;
    let local_normal = self.local_normal_at(&local_point);
    let mut world_normal = (*t_inv).matrix().transpose() * local_normal.to_homogeneous();
    world_normal[3] = 0.;
    UnitVector::new_normalize(Vector::from_homogeneous(world_normal).unwrap())
  }

  fn get_material(&self) -> &Material {
    &self.get_base().material
  }

  fn set_material(&mut self, material: Material) {
    self.get_base_mut().material = material
  }

  fn set_transform(&mut self, trans: Transform) {
    self.get_base_mut().transform_inverse = trans.inverse();
  }

  fn get_transform(&self) -> Transform {
    self.get_base().transform_inverse.inverse()
  }

  fn get_transform_inverse(&self) -> &Transform {
    &self.get_base().transform_inverse
  }
}
