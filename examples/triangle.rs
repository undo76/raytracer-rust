extern crate raytracer_rust;

use raytracer_rust::*;
use std::sync::Arc;

use nalgebra as na;
use std::fs::File;
use std::io::prelude::*;

const F_PI_4: f32 = std::f32::consts::FRAC_PI_4;
const F_PI_3: f32 = std::f32::consts::FRAC_PI_3;

fn main() {
    let mut floor_material = Material::default();

    floor_material.color = Mapping::checkers(
        &vec![WHITE * 0.7, WHITE * 0.8],
        na::convert(rotation_y(F_PI_4) * scaling(0.5, 0.5, 0.5)),
    );
    floor_material.specular = Mapping::from(0.7);
    floor_material.reflective = Some(Mapping::from(0.2));
    floor_material.attenuation = Attenuation::Squared;

    let floor = Arc::new(Plane::new(Transform::identity(), floor_material.clone()));

    let triangle = Arc::new(Triangle::new(
        Material::default(),
        [point(0., 0., 0.), point(1., 0., 0.), point(0., 1., 0.)],
    ));

    let light = PointLight::new(point(-10., 10., -10.), color(0.9, 0.8, 0.7));
    let world = World::new(vec![floor, triangle], vec![light]);

    let mut camera = Camera::new(1000, 800, F_PI_3);
    camera.set_transform(view_transform(
        point(0., 1.5, -5.),
        point(0., 1., 0.),
        vector(0., 1., 0.),
    ));

    let canvas = camera.render(world);

    let mut file = File::create("triangle.ppm").expect("Couldn't create file");
    file.write_all(canvas.to_ppm_string().as_bytes())
        .expect("Couldn't write canvas");
}
