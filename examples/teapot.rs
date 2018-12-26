extern crate raytracer_rust;

use raytracer_rust::*;
use std::sync::Arc;

use nalgebra as na;
use std::fs::File;
use std::io::prelude::*;

const F_PI_2: f32 = std::f32::consts::FRAC_PI_2;

fn main() {
    let mut floor_material = Material::default();

    floor_material.color = Mapping::checkers(
        &vec![WHITE * 0.7, WHITE * 0.8],
        na::convert(scaling(0.2, 0.2, 0.2)),
    );
    floor_material.specular = Mapping::from(0.7);
    floor_material.reflective = Some(Mapping::from(0.05));
    floor_material.attenuation = Attenuation::Squared;

    let walls = Arc::new(Cube::new(
        na::convert(scaling(10., 10., 10.)),
        Material::default(),
    ));

    let floor = Arc::new(Plane::new(Transform::identity(), floor_material.clone()));

    let mut obj_file = File::open("./examples/models/teapot.obj").expect("teapot.obj no file");
    let mut obj_str = String::new();
    let _res = obj_file.read_to_string(&mut obj_str).unwrap();
    let obj = parse(&obj_str);

    let mut group = Group::new(Transform::identity(), Material::default());
    for f in &obj.faces {
        Triangle::add_to_group(
            &mut group,
            &f.iter()
                .map(|v| obj.vertices[v.idx - 1])
                .collect::<Vec<_>>(),
        );
    }

    Triangle::add_to_group(
        &mut group,
        &[point(1., 1., 1.), point(1., 2., 1.), point(2., 1., 1.)],
    );

    let group = Arc::new(group);

    let light = PointLight::new(point(-8., 8., -8.), color(0.9, 0.8, 0.7));
    let world = World::new(vec![floor, walls, group], vec![light]);

    let mut camera = Camera::new(600, 300, F_PI_2);
    camera.set_transform(view_transform(
        point(1., 2.5, -5.),
        point(0., 1., 0.),
        vector(0., 1., 0.),
    ));

    let canvas = camera.render(world);

    let mut file = File::create("teapot.ppm").expect("Couldn't create file");
    file.write_all(canvas.to_ppm_string().as_bytes())
        .expect("Couldn't write canvas");
}
