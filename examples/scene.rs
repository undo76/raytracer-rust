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
    let mut wall_material = Material::default();

    floor_material.color = Mapping::checkers(
        &vec![WHITE * 0.6, WHITE * 0.8],
        na::convert(rotation_y(F_PI_4) * scaling(0.2, 0.2, 0.2)),
    );
    floor_material.specular = Mapping::from(0.6);
    floor_material.reflective = Some(Mapping::from(0.1));

    wall_material.color = Mapping::rings(
        &vec![RED * 0.7, BLUE * 0.5, WHITE * 0.5],
        na::convert(rotation_y(F_PI_4) * scaling(0.5, 0.5, 0.5)),
    );
    wall_material.reflective = Some(Mapping::rings(
        &vec![0.1, 0.01, 0.4],
        na::convert(rotation_y(F_PI_4) * scaling(0.5, 0.5, 0.5)),
    ));

    let floor = Arc::new(Plane::new(Transform::identity(), floor_material.clone()));

    #[rustfmt::skip]
    let left_wall = Arc::new(Plane::new(
        na::convert(translation(0., 0., 5.)
        * rotation_y(-F_PI_4)
        * rotation_x(-F_PI_2)),
        wall_material.clone(),
    ));

    #[rustfmt::skip]
    let right_wall = Arc::new(Plane::new(
        na::convert(translation(0., 0., 4.)
        * rotation_y(F_PI_4)
        * rotation_x(F_PI_2)),
        wall_material.clone(),
    ));

    let mut middle_material = Material::default();
    middle_material.color = Mapping::stripes(
        &vec![PURPLE * 0.7, PURPLE * 0.5],
        na::convert(rotation_z(F_PI_2) * scaling(0.2, 0.2, 0.2)),
    );
    middle_material.specular = Mapping::stripes(
        &vec![0.1, 1.],
        na::convert(rotation_z(F_PI_2) * scaling(0.2, 0.2, 0.2)),
    );
    middle_material.reflective = Some(Mapping::stripes(
        &vec![0.03, 0.1],
        na::convert(rotation_z(F_PI_2) * scaling(0.2, 0.2, 0.2)),
    ));
    let middle = Arc::new(Sphere::new(
        na::convert(translation(-0.5, 1., 0.5) * rotation_z(0.2) * rotation_x(0.2)),
        middle_material,
    ));

    let mut right_material = Material::default();
    right_material.color = Mapping::from(RED * 0.5);
    right_material.specular = Mapping::from(1.);
    right_material.diffuse = Mapping::from(0.5);

    right_material.reflective = Some(Mapping::from(0.3));
    right_material.diffuse = (0.8).into();

    let right = Arc::new(Cylinder::new(
        na::convert(translation(1.2, 0.2, -1.0) * scaling(0.2, 0.2, 0.2)),
        right_material.clone(),
        true,
    ));

    let mut left_material = Material::default();
    left_material.color = Mapping::from(color(1., 0.2, 0.2));
    left_material.ambient = Mapping::from(0.0);
    left_material.diffuse = Mapping::from(0.0);
    left_material.specular = Mapping::from(1.);
    left_material.reflective = Some(Mapping::from(0.7));
    left_material.transparency = Some(Mapping::from(0.9));
    left_material.refractive_index = 1.5;

    let left = Arc::new(Sphere::new(
        na::convert(translation(-1.5, 0.333, -0.75) * scaling(0.333, 0.333, 0.333)),
        left_material.clone(),
    ));

    let mut cube_material = Material::default();
    cube_material.color = Mapping::checkers(
        &vec![BLUE * 0.7, RED * 0.6],
        na::convert(scaling(0.5, 0.5, 0.5)),
    );
    cube_material.diffuse = 0.8.into();
    cube_material.transparency = Some(Mapping::checkers(
        &vec![0.01, 0.5],
        na::convert(scaling(0.5, 0.5, 0.5)),
    ));
    cube_material.reflective = Some((0.5).into());
    let cube = Arc::new(Cube::new(
        na::convert(translation(-0.2, 0.3001, -1.) * scaling(0.3, 0.3, 0.3) * rotation_y(-0.5)),
        cube_material.clone(),
    ));
    cube_material.refractive_index = 1.;

    let mut triangles = Group::new(
        na::convert(translation(2., 0., 0.) * rotation_y(F_PI_2) * scaling(0.5, 0.5, 0.5)),
        Material::default(),
    );
    Triangle::add_to_group(
        &mut triangles,
        &[
            point(0., 0., 0.),
            point(1., 0., 0.),
            point(1., 1., 0.),
            point(0., 1., 0.),
            point(0., 0., -2.),
        ],
    );
    let triangles = Arc::new(triangles);

    let light = PointLight::new(point(-10., 10., -10.), color(0.9, 0.8, 0.7));
    let light2 = PointLight::new(point(5., 5., -10.), color(0.3, 0.5, 0.5));

    let world = World::new(
        vec![
            floor, right_wall, left_wall, middle, left, right, cube, triangles,
        ],
        // vec![floor, middle, left, right],
        vec![light, light2],
    );

    let mut camera = Camera::new(2000, 1600, F_PI_3);
    camera.set_transform(view_transform(
        point(0., 1.5, -5.),
        point(0., 1., 0.),
        vector(0., 1., 0.),
    ));

    let canvas = camera.render(world);

    let mut file = File::create("scene.ppm").expect("Couldn't create file");
    file.write_all(canvas.to_ppm_string().as_bytes())
        .expect("Couldn't write canvas");
}
