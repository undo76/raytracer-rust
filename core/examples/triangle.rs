extern crate rustracer_core;

use rustracer_core::*;
use std::f32::consts::FRAC_PI_3;

fn main() {
    let floor_material = Material {
        color: Mapping::checkers(&[WHITE * 0.7, WHITE * 0.8], scaling(0.2, 0.2, 0.2)),
        specular: Mapping::from(0.7),
        reflective: Some(Mapping::from(0.05)),
        attenuation: Attenuation::Squared,
        ..Material::default()
    };

    let walls = Box::new(Cube::new(scaling(10., 10., 10.), Material::default()));
    let floor = Box::new(Plane::new(Transform::identity(), floor_material.clone()));

    let mut group = Group::new(Transform::identity(), Material::default());
    Triangle::add_to_group(
        &mut group,
        &[
            (point(1., 1., 0.), None),
            (point(1., 0., 0.), None),
            (point(0., 0., 0.), None),
            (point(0., 1., 0.), None),
            (point(0., 0., -2.), None),
        ],
    );

    let group = Box::new(group);

    let light = Light::Point(PointLight::new(point(-8., 8., -8.), color(0.9, 0.8, 0.7)));
    let world = World::new(vec![floor, walls, group], vec![light]);

    let mut camera = Camera::new(1000, 800, FRAC_PI_3);
    camera.set_transform(view_transform(
        point(1., 2.5, -5.),
        point(0., 1., 0.),
        vector(0., 1., 0.),
    ));

    let canvas = camera.render(world);
    canvas.save("./output/triangle.png");
}
