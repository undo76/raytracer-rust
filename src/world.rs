use crate::*;
use nalgebra as na;

pub struct World {
  pub shapes: Vec<Box<dyn Shape>>,
  pub light: PointLight,
}

impl World {
  pub fn intersects(&self, ray: &Ray) -> Intersections {
    let mut v = (*self).shapes.iter()
      .flat_map(|s| s.intersects(ray))
      .collect::<Vec<_>>();
    v.sort_unstable_by(|x, y| x.t.partial_cmp(&y.t).unwrap());
    v
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
    let s2 = Sphere::new(
      na::convert(scaling(0.5, 0.5, 0.5)), 
      Material::default()
    );

    World {
      shapes: vec![Box::new(s1), Box::new(s2)],
      light: point_light(point(-10., 10., -10.), WHITE),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn intersect_world_with_ray() {
    let world = World::default();
    let ray = Ray::new(
      point(0., 0., -5.),
      vector(0., 0., 1.)
    );
    let xs = world.intersects(&ray);
    assert_eq!(xs.len(), 4);
    assert_relative_eq!(xs[0].t, 4.);
    assert_relative_eq!(xs[1].t, 4.5);
    assert_relative_eq!(xs[2].t, 5.5);
    assert_relative_eq!(xs[3].t, 6.);
  }
}
