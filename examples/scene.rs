extern crate raytracer_rust;

use raytracer_rust::*;

use nalgebra as na;
use std::fs::File;
use std::io::prelude::*;

const F_PI_4: f32 = std::f32::consts::FRAC_PI_4;
const F_PI_3: f32 = std::f32::consts::FRAC_PI_3;
const F_PI_2: f32 = std::f32::consts::FRAC_PI_2;

fn main() {
  let mut floor_material = Material::default();
  floor_material.color = color(1., 0.9, 0.9);
  floor_material.specular = 0.;

  let floor = Box::new(Sphere::new(
    na::convert(scaling(10., 0.01, 10.)),
    floor_material,
  ));

  #[rustfmt::skip]
  let left_wall = Box::new(Sphere::new(
    na::convert(translation(0., 0., 5.)
      * rotation_y(-F_PI_4)
      * rotation_x(-F_PI_2)
      * scaling(10., 0.01, 10.)),
    floor_material,
  ));

  #[rustfmt::skip]
  let right_wall = Box::new(Sphere::new(
    na::convert(translation(0., 0., 5.)
      * rotation_y(F_PI_4)
      * rotation_x(F_PI_2)
      * scaling(10., 0.01, 10.)),
    floor_material,
  ));

  let mut middle_material = Material::default();
  middle_material.color = color(1., 0.2, 1.);
  middle_material.specular = 1.;
  let middle = Box::new(Sphere::new(
    na::convert(translation(-0.5, 1., 0.5)),
    middle_material,
  ));

  let mut right_material = Material::default();
  right_material.color = color(0.2, 0.2, 1.);
  right_material.specular = 0.1;
  let right = Box::new(Sphere::new(
    na::convert(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5)),
    right_material,
  ));

  let mut left_material = Material::default();
  left_material.color = color(1., 0.2, 0.2);
  left_material.specular = 1.;
  let left = Box::new(Sphere::new(
    na::convert(translation(-1.5, 0.333, -0.75) * scaling(0.333, 0.333, 0.333)),
    left_material,
  ));

  let light = PointLight::new(point(-10., 10., -10.), color(1., 1., 1.));
  let world = World::new(
    vec![floor, right_wall, left_wall, middle, left, right],
    vec![light],
  );
  let mut camera = Camera::new(1000, 800, F_PI_3);
  camera.set_transform(view_transform(
    point(0., 1.5, -5.),
    point(0., 1., 0.),
    vector(0., 1., 0.),
  ));

  let canvas = camera.render(&world);

  let mut file = File::create("scene.ppm").expect("Couldn't create file");
  file
    .write_all(canvas.to_ppm_string().as_bytes())
    .expect("Couldn't write canvas");
}
