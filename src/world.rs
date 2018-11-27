use crate::*;
use nalgebra as na;

pub struct World {
  pub objects: Vec<Box<dyn Shape>>,
  pub light: PointLight,
}

pub fn default_world() -> World {
  let s1 = Sphere::default();
  //let m1 = s1.get_material();
  //m1.color = color(0.8, 1.0, 0.6);

  let mut s2 = Sphere::default();
  s2.set_transform(na::convert(scaling(0.5, 0.5, 0.5)));

  World { 
    objects: vec![Box::new(s1), Box::new(s2)],
    light: point_light(point(-10., 10., -10.), WHITE)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_world() {
    let _w = default_world();
  }
}
