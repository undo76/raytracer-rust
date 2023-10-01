extern crate rustracer_core;

use std::fs::File;
use std::io::prelude::*;

use rustracer_core::*;

const F_PI_4: f32 = std::f32::consts::FRAC_PI_4;

fn main() {
    let mut floor_material = Material::default();

    floor_material.color = Mapping::checkers(&[WHITE * 0.7, WHITE * 0.8], scaling(0.2, 0.2, 0.2));
    floor_material.specular = Mapping::from(0.7);
    floor_material.reflective = Some(Mapping::from(0.05));
    floor_material.attenuation = Attenuation::Squared;

    let floor = Box::new(Plane::new(Transform::identity(), floor_material.clone()));

    let mut obj_file = File::open("./examples/models/coca-cola.obj").expect("file not found");
    let mut obj_str = String::new();
    let _res = obj_file.read_to_string(&mut obj_str).unwrap();
    let obj = parse(&obj_str);

    let mut group = Group::new(Transform::identity(), Material::default());
    for f in &obj.faces {
        Triangle::add_to_group(
            &mut group,
            &f.iter()
                .map(|v| {
                    (
                        obj.vertices[v.idx - 1],
                        v.normal_idx.map(|n_idx| obj.normals[n_idx - 1]),
                    )
                })
                .collect::<Vec<_>>(),
        );
    }

    group.set_transform(scaling(0.01, 0.01, 0.01));

    let group = Box::new(group);

    let light = Light::Point(PointLight::new(point(-8., 8., -5.), color(0.9, 0.8, 0.7)));
    let world = World::new(vec![floor, group], vec![light]);

    let mut camera = Camera::new(1600, 1200, F_PI_4);
    camera.set_transform(view_transform(
        point(1., 3., -5.),
        point(0.5, 2., -2.),
        vector(0., 1., 0.),
    ));

    let canvas = camera.render(world);
    canvas.save("coca-cola.png");
}
