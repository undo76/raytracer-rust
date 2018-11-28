use crate::*;
use nalgebra as na;

pub struct World {
  pub shapes: Vec<Box<dyn Shape>>,
  pub lights: Vec<PointLight>,
}

impl World {
  pub fn intersects(&self, ray: &Ray) -> Intersections {
    let mut v = (*self)
      .shapes
      .iter()
      .filter_map(|s| s.intersects(ray))
      .flatten()
      .collect::<Vec<_>>();
    v.sort_unstable_by(|x, y| x.t.partial_cmp(&y.t).unwrap());
    v
  }

  pub fn shade_hit(&self, hit: &Hit) -> ColorRgbFloat {
    self
      .lights
      .iter()
      .map(|light| {
        hit
          .intersection
          .object
          .get_material()
          .lighting(&light, &hit.point, &hit.eyev, &hit.normalv)
      })
      .sum()
  }
}

impl Default for World {
  fn default() -> World {
    let m1 = Material {
      color: color(0.8, 1.0, 0.6),
      diffuse: 0.7,
      specular: 0.2,
      ..Material::default()
    };

    let s1 = Sphere::new(Transform::identity(), m1);
    let s2 = Sphere::new(na::convert(scaling(0.5, 0.5, 0.5)), Material::default());

    World {
      shapes: vec![Box::new(s1), Box::new(s2)],
      lights: vec![PointLight::new(point(-10., 10., -10.), WHITE)],
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn intersect_world_with_ray() {
    let world = World::default();
    let ray = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let xs = world.intersects(&ray);
    assert_eq!(xs.len(), 4);
    assert_relative_eq!(xs[0].t, 4.);
    assert_relative_eq!(xs[1].t, 4.5);
    assert_relative_eq!(xs[2].t, 5.5);
    assert_relative_eq!(xs[3].t, 6.);
  }

  #[test]
  fn shade_intersection() {
    let world = World::default();
    let ray = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let xs = world.intersects(&ray);
    let hit = xs[0].prepare_hit(&ray);
    let c = world.shade_hit(&hit);
    assert_relative_eq!(c, color(0.38066125, 0.4758265, 0.28549594))
  }

  #[test]
  fn shade_intersection_inside() {
    let mut world = World::default();
    world.lights = vec![PointLight::new(point(0., 0.25, 0.), color(1., 1., 1.))];
    let ray = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
    let intersection = Intersection::new(0.5, &(*world.shapes[1]));
    let hit = intersection.prepare_hit(&ray);
    let c = world.shade_hit(&hit);
    assert_relative_eq!(c, color(0.9049845, 0.9049845, 0.9049845))
  }
}
