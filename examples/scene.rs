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

  floor_material.color = Mapping::checkers(
    &vec![BLACK, WHITE * 0.8],
    na::convert(rotation_y(F_PI_4) * scaling(0.5, 0.5, 0.5)),
  );
  floor_material.specular = Mapping::from(0.7);
  floor_material.reflective = Some(Mapping::from(0.2));

  wall_material.color = Mapping::rings(
    &vec![RED * 0.7, BLUE * 0.5, WHITE * 0.5],
    na::convert(rotation_y(F_PI_4) * scaling(0.5, 0.5, 0.5)),
  );
  wall_material.reflective = Some(Mapping::rings(
    &vec![0.1, 0.1, 0.4],
    na::convert(rotation_y(F_PI_4) * scaling(0.5, 0.5, 0.5)),
  ));

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
  middle_material.color = Mapping::stripes(
    &vec![PURPLE * 0.7, PURPLE * 0.5],
    na::convert(rotation_z(F_PI_2) * scaling(0.2, 0.2, 0.2)),
  );
  middle_material.specular = Mapping::from(1.);
  middle_material.reflective = Some(Mapping::stripes(
    &vec![0.03, 0.1],
    na::convert(rotation_z(F_PI_2) * scaling(0.2, 0.2, 0.2)),
  ));
  let middle = Box::new(Sphere::new(
    na::convert(translation(-0.5, 1., 0.5)),
    middle_material,
  ));

  let mut right_material = Material::default();
  right_material.color = Mapping::gradient(
    (GREEN, BLUE),
    na::convert(translation(1., 0., 0.) * scaling(2., 2., 2.)),
  );
  right_material.specular = Mapping::from(0.);
  right_material.diffuse = Mapping::from(0.5);

  right_material.reflective = Some(Mapping::stripes(
    &vec![0., 0.4],
    na::convert(scaling(0.2, 0.2, 0.2)),
  ));
  right_material.diffuse = Mapping::stripes(&vec![0.4, 0.2], na::convert(scaling(0.2, 0.2, 0.2)));
  // wall_material.diffuse = Mapping::stripes(&vec![0.7, 0.1], Transform::identity());
  // wall_material.ambient = Mapping::from(0.);

  let right = Box::new(Sphere::new(
    na::convert(translation(1.2, 0.5, -1.0) * scaling(0.5, 0.5, 0.5)),
    right_material,
  ));

  let mut left_material = Material::default();
  left_material.color = Mapping::from(color(1., 0.2, 0.2));
  left_material.specular = Mapping::from(1.);
  left_material.reflective = Some(Mapping::from(0.05));

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

  let canvas = camera.render(world);

  let mut file = File::create("scene.ppm").expect("Couldn't create file");
  file
    .write_all(canvas.to_ppm_string().as_bytes())
    .expect("Couldn't write canvas");
}
