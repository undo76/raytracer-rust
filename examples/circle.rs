extern crate raytracer_rust;

use raytracer_rust::canvas::*;
use raytracer_rust::color::*;
use raytracer_rust::geom::*;
use raytracer_rust::ray::*;
use raytracer_rust::shape::*;
use raytracer_rust::transform::*;

use nalgebra as na;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let origin = point(0., 0., -5.);
  let color1 = color(1., 0., 1.);
  let color2 = color(0., 0., 1.);
  let wall_size = 7.;
  let pixels = 200;
  let mut c = canvas(pixels, pixels);
  let pixel_size = wall_size / (pixels as f32);

  let mut s = sphere();
  s.set_transform(na::convert(scaling(1., 0.5, 1.)));
  s.set_color(color1);

  let mut s2 = sphere();
  s2.set_transform(na::convert(
    translation(-0.5, 0., 0.) * scaling(0.8, 0.8, 0.8),
  ));
  s2.set_color(color2);

  for y in 0..pixels {
    for x in 0..pixels {
      let pos = point(
        -wall_size / 2. + (x as f32) * pixel_size,
        wall_size / 2. - (y as f32) * pixel_size,
        10.,
      );
      let r = ray(origin, normalize(&(pos - origin)));
      let mut intersections = s.intersects(&r);
      intersections.extend(s2.intersects(&r));

      if let Some(Intersection { object, .. }) = hit(&intersections) {
        c.set(x, y, object.get_color().into());
      }
    }
  }

  let mut file = File::create("circle.ppm").expect("Couldn't create file");
  file
    .write_all(c.to_ppm_string().as_bytes())
    .expect("Couldn't write canvas");
}
