use crate::types::*;
use rustracer_core as rc;

pub fn parse_yaml<T>(yaml_str: &str) -> Result<T, serde_yaml::Error>
where
    for<'de> T: serde::de::Deserialize<'de>,
{
    let value = serde_yaml::from_str(&yaml_str)?;
    let merged = yaml_merge_keys::merge_keys_serde(value).expect("Error while merging YAML");
    serde_yaml::from_value(merged)
}

pub fn build_scene(scene: &Scene) -> (rc::World, rc::Camera) {
    let Scene {
        shapes,
        lights,
        camera,
        ..
    } = scene;

    let rc_shapes: Vec<Box<dyn rc::Shape + Send>> =
        shapes.iter().map(|shape| build_shape(shape)).collect();

    let rc_lights: Vec<rc::PointLight> = build_lights(lights);
    let rc_camera: rc::Camera = build_camera(camera);
    (rc::World::new(rc_shapes, rc_lights), rc_camera)
}

fn build_shape(shape: &Shape) -> Box<dyn rc::Shape + Send> {
    use crate::Shape::*;
    match shape {
        Plane {
            base: BaseShape {
                transform,
                material,
            },
        } => Box::new(rc::Plane::new(
            build_transforms(transform),
            build_material(material),
        )),
        Cylinder {
            closed,
            base: BaseShape {
                transform,
                material,
            },
        } => Box::new(rc::Cylinder::new(
            build_transforms(transform),
            build_material(material),
            *closed,
        )),
        Cube {
            base: BaseShape {
                transform,
                material,
            },
        } => Box::new(rc::Cube::new(
            build_transforms(transform),
            build_material(material),
        )),
        Sphere {
            base: BaseShape {
                transform,
                material,
            },
        } => Box::new(rc::Sphere::new(
            build_transforms(transform),
            build_material(material),
        )),
        Group {
            shapes,
            base: BaseShape {
                transform,
                material,
            },
        } => {
            let mut group = rc::Group::new(build_transforms(transform), build_material(material));
            for s in shapes {
                group.add_shape(build_shape(s))
            }
            Box::new(group)
        }
    }
}

fn build_transforms(transforms: &Transforms) -> rc::Transform {
    use crate::Transforms::*;
    match transforms {
        SingleTransform(t) => build_transform(t),
        ChainedTransform(ts) => ts
            .iter()
            .map(|ts| build_transforms(ts))
            .fold(rc::Transform::identity(), |acc, t| t * acc),
    }
}

fn build_transform(transform: &Transform) -> rc::Transform {
    use crate::Transform::*;
    match *transform {
        Identity => rc::Transform::identity(),
        Translation(x, y, z) => rc::translation(x, y, z),
        Scaling(x, y, z) => rc::scaling(x, y, z),
        RotationX(a) => rc::rotation_x(build_angle(a)),
        RotationY(a) => rc::rotation_y(build_angle(a)),
        RotationZ(a) => rc::rotation_z(build_angle(a)),
    }
}

fn build_material(material: &Material) -> rc::Material {
    rc::Material {
        color: build_mapping(&material.color),
        ambient: build_mapping(&material.ambient),
        diffuse: build_mapping(&material.diffuse),
        specular: build_mapping(&material.specular),
        shininess: build_mapping(&material.shininess),
        reflective: material.reflective.as_ref().map(|v| build_mapping(v)),
        transparency: material.transparency.as_ref().map(|v| build_mapping(v)),
        refractive_index: material.refractive_index,
        attenuation: rc::Attenuation::None,
    }
}

fn build_mapping<F, T>(mapping: &Mapping<F>) -> rc::Mapping<T>
where
    F: Copy,
    T: Copy
        + core::ops::Sub<Output = T>
        + core::ops::Add<Output = T>
        + core::ops::Mul<f32, Output = T>
        + std::convert::From<F>,
{
    use crate::Mapping::*;
    use crate::PatternMapping::*;
    match mapping {
        Uniform(value) => rc::Mapping::uniform((*value).into()),
        Pattern(Stripes { values, transform }) => {
            rc::Mapping::stripes(&map_vector(&values), build_transforms(&transform))
        }
        Pattern(Gradient { values, transform }) => {
            let v = map_vector(&values);
            let tuple = (v[0], v[1]);
            rc::Mapping::gradient(tuple, build_transforms(&transform))
        }
        Pattern(Checkers { values, transform }) => {
            rc::Mapping::checkers(&map_vector(&values), build_transforms(&transform))
        }
        Pattern(Rings { values, transform }) => {
            rc::Mapping::rings(&map_vector(&values), build_transforms(&transform))
        }
    }
}

fn map_vector<F: Copy, T: From<F>>(f: &[F]) -> Vec<T> {
    f.iter().map(|&f| f.into()).collect()
}

impl From<Rgb> for rc::ColorRgbFloat {
    fn from(rgb: Rgb) -> Self {
        build_rgb(&rgb)
    }
}

fn build_camera(camera: &Camera) -> rc::Camera {
    let Camera {
        size: (h, w),
        field_of_view,
        from,
        to,
        up,
    } = camera;
    let transform = rc::view_transform(build_point(&from), build_point(&to), build_vector(&up));
    let mut camera = rc::Camera::new(*h, *w, build_angle(*field_of_view));
    camera.set_transform(transform);
    camera
}

fn build_point(p: &Point) -> rc::Point {
    let Point(x, y, z) = *p;
    rc::point(x, y, z)
}

fn build_rgb(rgb: &Rgb) -> rc::ColorRgbFloat {
    let Rgb(r, g, b) = *rgb;
    rc::color(r, g, b)
}

fn build_vector(v: &Vector) -> rc::Vector {
    let Vector(x, y, z) = *v;
    rc::vector(x, y, z)
}

fn build_lights(lights: &[Light]) -> Vec<rc::PointLight> {
    lights.iter().map(|l| build_light(l)).collect()
}

fn build_light(light: &Light) -> rc::PointLight {
    use crate::Light::*;
    match light {
        PointLight {
            position,
            intensity,
        } => rc::PointLight::new(build_point(position), build_rgb(intensity)),
    }
}

fn build_angle(angle: Angle) -> f32 {
    use crate::Angle::*;
    use std::f32::consts::*;
    match angle {
        Pi => PI,
        FPi2 => FRAC_PI_2,
        FPi3 => FRAC_PI_3,
        FPi4 => FRAC_PI_4,
        FPi6 => FRAC_PI_6,
        FPi8 => FRAC_PI_8,
        Rad(rad) => rad,
        Deg(deg) => PI * deg / 180.,
    }
}
