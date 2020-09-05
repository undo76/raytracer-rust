extern crate rustracer_core;

use core::f32::consts::*;
use std::fs::File;
use std::io::Write;

use rustracer_core::color;
use rustracer_core::*;

fn main() {
    let floor_material = Material {
        color: Mapping::checkers(&[WHITE * 0.7, WHITE * 0.8], scaling(0.2, 0.2, 0.2)),
        specular: Mapping::from(0.7),
        reflective: Some(Mapping::from(0.05)),
        attenuation: Attenuation::Squared,
        ..Material::default()
    };

    let floor = Box::new(Plane::new(Transform::identity(), floor_material.clone()));

    let sky_material = Material {
        color: color(0.4, 0.4, 0.7).into(),
        ambient: 0.7.into(),
        attenuation: Attenuation::None,
        ..Default::default()
    };
    let sky = Box::new(Plane::new(translation(0., 100., 0.), sky_material.clone()));

    let mut group = Group::new(Transform::identity(), Material::default());
    read_obj_file(&mut group, "./examples/models/icosahedron.obj");

    group.set_transform(translation(0., 1., 0.));

    group.set_material(Material {
        color: Mapping::from(color(1.0, 1.0, 0.5)),
        ambient: Mapping::from(0.0),
        diffuse: Mapping::from(0.01),
        specular: Mapping::from(1.0),
        shininess: Mapping::from(10000.),
        reflective: Some(0.8.into()),
        transparency: None,
        refractive_index: 1.5,
        attenuation: Attenuation::Squared,
    });

    let group = Box::new(group);

    let light = Light::Point(PointLight::new(point(-8., 8., -5.), color(0.9, 0.8, 0.7)));
    let light2 = Light::Point(PointLight::new(point(8., 8., -5.), color(0.3, 0.3, 0.3)));

    let world = World::new(vec![sky, floor, group], vec![light, light2]);

    let mut camera = Camera::new(1000, 1000, FRAC_PI_6);
    camera.set_transform(view_transform(
        point(4., 2., -5.),
        point(0., 1., 0.),
        vector(0., 1., 0.),
    ));

    let canvas = camera.render(world);

    let mut file = File::create("icosahedron.ppm").expect("Couldn't create file");
    file.write_all(canvas.to_ppm_string().as_bytes())
        .expect("Couldn't write canvas");
}
