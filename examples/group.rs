extern crate raytracer_rust;

use raytracer_rust::*;
use std::sync::Arc;

use nalgebra as na;
use std::fs::File;
use std::io::prelude::*;

const F_PI_4: f32 = std::f32::consts::FRAC_PI_4;
const F_PI_3: f32 = std::f32::consts::FRAC_PI_3;
const F_PI_2: f32 = std::f32::consts::FRAC_PI_2;

fn main() {
    let mut floor_material = Material::default();

    floor_material.color = Mapping::checkers(
        &vec![BLACK, WHITE * 0.8],
        na::convert(rotation_y(F_PI_4) * scaling(0.5, 0.5, 0.5)),
    );
    floor_material.specular = Mapping::from(0.7);
    floor_material.reflective = Some(Mapping::from(0.2));

    let floor = Arc::new(Plane::new(Transform::identity(), floor_material.clone()));

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

    let sphere = Arc::new(Sphere::new(
        na::convert(translation(-0.5, 1., 0.5)),
        middle_material.clone(),
    ));

    let sphere2 = Arc::new(Sphere::new(
        na::convert(translation(0.5, 2., 2.5)),
        Material::default(),
    ));

    let mut group = Group::new(na::convert(translation(-0.5, 1., 0.5)), Material::default());
    group.add_shape(sphere);
    group.add_shape(sphere2);

    let group = Arc::new(group);

    let light = PointLight::new(point(-10., 10., -10.), color(0.9, 0.8, 0.7));

    let world = World::new(vec![floor, group], vec![light]);

    let mut camera = Camera::new(1000, 800, F_PI_3);
    camera.set_transform(view_transform(
        point(0., 1.5, -5.),
        point(0., 1., 0.),
        vector(0., 1., 0.),
    ));

    let canvas = camera.render(world);

    let mut file = File::create("group.ppm").expect("Couldn't create file");
    file.write_all(canvas.to_ppm_string().as_bytes())
        .expect("Couldn't write canvas");
}
