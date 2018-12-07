extern crate raytracer_rust;

use raytracer_rust::*;

use nalgebra as na;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let origin = point(0., 0., -5.);
  let wall_size = 7.;
  let pixels = 200;
  let mut c = canvas(pixels, pixels);
  let pixel_size = wall_size / (pixels as f32);

  let light = PointLight::new(point(-10., 10., -10.), WHITE);

  let mut s = Sphere::default();
  s.set_transform(na::convert(scaling(1., 0.5, 1.)));
  let mut m = Material::default();
  m.pattern = Pattern::Uniform(RED);
  s.set_material(m);

  let mut s2 = Sphere::default();
  s2.set_transform(na::convert(
    translation(-0.5, 0., 0.) * scaling(0.8, 0.8, 0.8),
  ));
  let mut m = Material::default();
  m.pattern = Pattern::Uniform(BLUE);
  s2.set_material(m);

  for y in 0..pixels {
    for x in 0..pixels {
      let pos = point(
        -wall_size / 2. + (x as f32) * pixel_size,
        wall_size / 2. - (y as f32) * pixel_size,
        10.,
      );
      let direction = normalize(&(pos - origin));
      let r = Ray::new(origin, direction);
      let mut intersections = s.intersects(&r).unwrap_or_default();
      intersections.extend(s2.intersects(&r).unwrap_or_default());

      if let Some(&Intersection { t, object, .. }) = hit(&intersections) {
        let material = object.get_material();
        let p = r.position(t);
        c.set(
          x,
          y,
          material
            .lighting(
              &light,
              &p,
              &na::Unit::new_unchecked(-direction),
              &object.normal_at(&p),
              false,
            )
            .into(),
        );
      }
    }
  }

  let mut file = File::create("spheres.ppm").expect("Couldn't create file");
  file
    .write_all(c.to_ppm_string().as_bytes())
    .expect("Couldn't write canvas");
}
