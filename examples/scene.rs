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
  let mut wall_material = Material::default();

  floor_material.color = Pattern::Checkered(CheckersPattern {
    values: vec![BLACK, WHITE],
    transform_inverse: na::convert((rotation_y(F_PI_4) * scaling(0.5, 0.5, 0.5)).inverse()),
  });
  floor_material.specular = 0.7;
  floor_material.reflective = Some(0.1);

  wall_material.color = Pattern::Ring(RingPattern {
    values: vec![RED, BLUE, WHITE],
    transform_inverse: na::convert((rotation_y(F_PI_4) * scaling(0.5, 0.5, 0.5)).inverse()),
  });
  wall_material.specular = 0.;

  let floor = Box::new(Plane::new(Transform::identity(), floor_material.clone()));

  #[rustfmt::skip]
  let left_wall = Box::new(Plane::new(
    na::convert(translation(0., 0., 5.)
      * rotation_y(-F_PI_4)
      * rotation_x(-F_PI_2)),
    wall_material.clone(),
  ));

  #[rustfmt::skip]
  let right_wall = Box::new(Plane::new(
    na::convert(translation(0., 0., 5.)
      * rotation_y(F_PI_4)
      * rotation_x(F_PI_2)),
    wall_material.clone(),
  ));

  let mut middle_material = Material::default();
  middle_material.color = Pattern::Uniform(UniformPattern {
    value: color(1., 0.2, 1.),
  });
  middle_material.specular = 1.;
  middle_material.reflective = Some(0.1);
  let middle = Box::new(Sphere::new(
    na::convert(translation(-0.5, 1., 0.5)),
    middle_material,
  ));

  let mut right_material = Material::default();
  right_material.color = Pattern::Gradient(GradientPattern {
    values: (GREEN, BLUE),
    transform_inverse: na::convert((translation(1., 0., 0.) * scaling(2., 2., 2.)).inverse()),
  });
  right_material.specular = 0.;
  right_material.reflective = Some(0.5);
  right_material.diffuse = 0.5;

  let right = Box::new(Sphere::new(
    na::convert(translation(1.2, 0.5, -1.0) * scaling(0.5, 0.5, 0.5)),
    right_material,
  ));

  let mut left_material = Material::default();
  left_material.color = Pattern::Uniform(UniformPattern {
    value: color(1., 0.2, 0.2),
  });
  left_material.specular = 1.;
  left_material.reflective = Some(0.05);

  let left = Box::new(Sphere::new(
    na::convert(translation(-1.5, 0.333, -0.75) * scaling(0.333, 0.333, 0.333)),
    left_material,
  ));

  let light = PointLight::new(point(-10., 10., -10.), color(0.7, 0.7, 0.7));
  let light2 = PointLight::new(point(10., 5., -10.), color(0.3, 0.3, 0.3));

  let world = World::new(
    vec![floor, right_wall, left_wall, middle, left, right],
    // vec![floor, middle, left, right],
    vec![light, light2],
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
