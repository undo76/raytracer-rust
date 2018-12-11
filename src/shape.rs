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
    Hit {
      intersection: &self,
      point,
      eyev,
      inside,
      normalv,
      reflectv,
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

pub trait Shape: core::fmt::Debug + Sync + Send {
  fn get_base(&self) -> &BaseShape;
  fn get_base_mut(&mut self) -> &mut BaseShape;
  fn local_intersects(&self, local_ray: &Ray) -> Option<Intersections>;
  fn local_normal_at(&self, p: &Point) -> Vector;

  fn intersects(&self, ray: &Ray) -> Option<Intersections> {
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
