extern crate rustracer_core;

use rustracer_core::*;

const F_PI_4: f32 = std::f32::consts::FRAC_PI_4;
const F_PI_3: f32 = std::f32::consts::FRAC_PI_3;
const F_PI_2: f32 = std::f32::consts::FRAC_PI_2;

fn main() {
    let floor_material = Material {
        color: Mapping::checkers(
            &[WHITE * 0.6, WHITE * 0.8],
            rotation_y(F_PI_4) * scaling(0.2, 0.2, 0.2),
        ),
        specular: Mapping::from(0.6),
        reflective: Some(Mapping::from(0.1)),
        ..Material::default()
    };

    let wall_material = Material {
        color: Mapping::rings(
            &[RED * 0.7, BLUE * 0.5, WHITE * 0.5],
            rotation_y(F_PI_4) * scaling(0.5, 0.5, 0.5),
        ),
        specular: Mapping::rings(
            &[0.1, 0.01, 0.1],
            rotation_y(F_PI_4) * scaling(0.5, 0.5, 0.5),
        ),
        reflective: Some(Mapping::rings(
            &[0.1, 0.01, 0.1],
            rotation_y(F_PI_4) * scaling(0.5, 0.5, 0.5),
        )),
        ..Material::default()
    };

    let floor = Box::new(Plane::new(Transform::identity(), floor_material.clone()));

    let left_wall = Box::new(Plane::new(
        translation(0., 0., 5.) * rotation_y(-F_PI_4) * rotation_x(-F_PI_2),
        wall_material.clone(),
    ));

    let right_wall = Box::new(Plane::new(
        translation(0., 0., 4.) * rotation_y(F_PI_4) * rotation_x(F_PI_2),
        wall_material.clone(),
    ));

    let middle_material = Material {
        color: Mapping::stripes(&[PURPLE * 0.7, PURPLE * 0.5], scaling(0.2, 0.2, 0.2)),
        specular: Mapping::stripes(&[0.1, 1.], scaling(0.2, 0.2, 0.2)),
        reflective: Some(Mapping::stripes(&[0.03, 0.1], scaling(0.2, 0.2, 0.2))),
        ..Material::default()
    };

    let middle = Box::new(Sphere::new(
        translation(-0.5, 1., 0.5) * rotation_z(0.2) * rotation_x(0.2),
        middle_material,
    ));

    let right_material = Material {
        color: Mapping::from(RED * 0.5),
        specular: Mapping::from(1.),
        reflective: Some(0.3.into()),
        diffuse: (0.8).into(),
        ..Material::default()
    };

    let right = Box::new(Cylinder::new(
        translation(1.2, 0.2, -1.0) * scaling(0.2, 0.2, 0.2),
        right_material.clone(),
        true,
    ));

    let left_material = Material {
        color: Mapping::from(color(1., 0.2, 0.2)),
        ambient: Mapping::from(0.0),
        diffuse: Mapping::from(0.0),
        specular: Mapping::from(1.),
        shininess: Mapping::from(100.),
        reflective: Some(0.7.into()),
        transparency: Some(0.9.into()),
        refractive_index: 1.5,
        attenuation: Attenuation::Squared,
    };

    let left = Box::new(Sphere::new(
        translation(-1.5, 0.333, -0.75) * scaling(0.333, 0.333, 0.333),
        left_material.clone(),
    ));

    let cube_material = Material {
        color: Mapping::checkers(&[BLUE * 0.7, RED * 0.6], scaling(0.5, 0.5, 0.5)),
        ambient: Mapping::from(0.0),
        diffuse: Mapping::from(0.8),
        specular: Mapping::from(1.),
        shininess: Mapping::from(10000.),
        reflective: Some(0.5.into()),
        transparency: Some(Mapping::checkers(&[0.01, 0.5], scaling(0.5, 0.5, 0.5))),
        refractive_index: 1.2,
        attenuation: Attenuation::Squared,
    };

    let cube = Box::new(Cube::new(
        translation(-0.2, 0.3001, -1.) * scaling(0.3, 0.3, 0.3) * rotation_y(-0.5),
        cube_material.clone(),
    ));

    let mut group = Group::new(
        translation(1.7, 0.3, 0.2) * scaling(0.007, 0.007, 0.007),
        Material::default(),
    );
    read_obj_from_bytes(&mut group, include_bytes!("./models/teapot.obj"));
    let group = Box::new(group);

    let light = Light::Point(PointLight::new(
        point(-10., 10., -10.),
        color(0.5, 0.5, 0.1),
    ));
    let light2 = Light::Point(PointLight::new(point(5., 5., -10.), color(0.3, 0.3, 0.3)));

    let world = World::new(
        vec![
            floor, right_wall, left_wall, middle, left, right, cube, group,
        ],
        vec![light, light2],
    );

    let mut camera = Camera::new(1000, 800, F_PI_3);
    camera.set_transform(view_transform(
        point(0., 1.5, -5.),
        point(0., 1., 0.),
        vector(0., 1., 0.),
    ));

    let canvas = camera.render(world);
    canvas.save("./output/scene.png");
}
