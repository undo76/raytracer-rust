#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct Point(pub f32, pub f32, pub f32);

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct Vector(pub f32, pub f32, pub f32);

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct Rgb(pub f32, pub f32, pub f32);

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum Angle {
    Pi,
    FPi2,
    FPi3,
    FPi4,
    FPi6,
    FPi8,
    Deg(f32),
    Rad(f32),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Transforms {
    ChainedTransform(Vec<Transforms>),
    SingleTransform(Transform),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Mapping<T> {
    Uniform(T),
    Pattern(PatternMapping<T>),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum PatternMapping<T> {
    Stripes {
        values: Vec<T>,
        #[serde(default)]
        transform: Transforms,
    },
    Gradient {
        values: Vec<T>,
        #[serde(default)]
        transform: Transforms,
    },
    Checkers {
        values: Vec<T>,
        #[serde(default)]
        transform: Transforms,
    },
    Rings {
        values: Vec<T>,
        #[serde(default)]
        transform: Transforms,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, deny_unknown_fields)]
pub struct Material {
    pub color: Mapping<Rgb>,
    pub ambient: Mapping<f32>,
    pub diffuse: Mapping<f32>,
    pub specular: Mapping<f32>,
    pub shininess: Mapping<f32>,
    pub reflective: Option<Mapping<f32>>,
    pub transparency: Option<Mapping<f32>>,
    pub refractive_index: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Transform {
    Identity,
    Scaling(f32, f32, f32),
    Translation(f32, f32, f32),
    RotationX(Angle),
    RotationY(Angle),
    RotationZ(Angle),
}

// Shapes

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub enum Shape {
    Plane {
        #[serde(flatten, default)]
        base: BaseShape,
    },
    Cylinder {
        #[serde(default)]
        closed: bool,
        #[serde(flatten, default)]
        base: BaseShape,
    },
    Sphere {
        #[serde(flatten, default)]
        base: BaseShape,
    },
    Cube {
        #[serde(flatten, default)]
        base: BaseShape,
    },
    Group {
        shapes: Vec<Shape>,
        #[serde(flatten, default)]
        base: BaseShape,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(default, deny_unknown_fields)]
pub struct BaseShape {
    pub material: Material,
    pub transform: Transforms,
}

// Camera & Lights

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, deny_unknown_fields)]
pub struct Camera {
    pub size: (usize, usize),
    pub field_of_view: Angle,
    pub from: Point,
    pub to: Point,
    pub up: Vector,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub enum Light {
    PointLight {
        position: Point,
        #[serde(default)]
        intensity: Rgb,
    },

    AreaLight {
        position: Point,
        #[serde(default)]
        intensity: Rgb,
        uv: (Vector, Vector),
        steps: (u8, u8),
        jitter: u8,
    },
    DirectionalLight {
        direction: Vector,
        #[serde(default)]
        intensity: Rgb,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    #[serde(default)]
    pub fragments: Vec<Fragment>,
    pub shapes: Vec<Shape>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Fragment {
    MaterialFragment { material: Box<Material> },
    TransformFragment { transform: Box<Transforms> },
    ColorFragment { color: Rgb },
}

// Defaults

impl Default for Rgb {
    fn default() -> Rgb {
        Rgb(1., 1., 1.)
    }
}

impl Default for Material {
    fn default() -> Material {
        Material {
            color: Mapping::Uniform(Rgb::default()),
            ambient: Mapping::Uniform(0.1),
            diffuse: Mapping::Uniform(0.6),
            specular: Mapping::Uniform(0.1),
            shininess: Mapping::Uniform(7.0),
            reflective: None,
            transparency: None,
            refractive_index: 1.0,
        }
    }
}

impl Default for Transforms {
    fn default() -> Transforms {
        Transforms::SingleTransform(Transform::Identity)
    }
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            size: (1600, 1200),
            field_of_view: Angle::FPi4,
            from: Point(1., 1., 0.),
            to: Point(0., 0., 0.),
            up: Vector(0., 1., 0.),
        }
    }
}

// ===============
// TESTS
// ===============
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let json = r#"[1, 2, 3]"#;
        let res: Point = serde_json::from_str(json).unwrap();
        assert_eq!(res, Point(1., 2., 3.));
    }

    #[test]
    fn test_transform() {
        let json = r#"{ "Scaling": [1, 2, 3] }"#;
        let res: Transform = serde_json::from_str(json).unwrap();
        assert_eq!(res, Transform::Scaling(1., 2., 3.));
    }

    #[test]
    fn test_default_transform() {
        let json = r#"
            { 
                "Sphere": {
                    "material": {
                        "color": [1, 1, 0],
                        "ambient": { "Stripes": { "values": [0, 1] } }
                    }
                }
            }
        "#;
        let res: Shape = serde_json::from_str(json).unwrap();
        // println!("{:#}", serde_yaml::to_string(&res).unwrap());
        assert_eq!(
            res,
            Shape::Sphere {
                base: BaseShape {
                    material: Material {
                        color: Mapping::Uniform(Rgb(1., 1., 0.0)),
                        ambient: Mapping::Pattern(PatternMapping::Stripes {
                            values: vec![0.0, 1.0],
                            transform: Transforms::default(),
                        }),
                        ..Material::default()
                    },
                    transform: Transforms::default(),
                }
            }
        );
    }

    #[test]
    fn test_sphere() {
        let yaml = r#"
---
Sphere:
    material:
        color: [1.0, 1.0, 0.0]
        specular: 
            Stripes:
                values: [0, 1]
    transform:
        - Scaling: [1, 2, 3]
"#;
        let res: Shape = serde_yaml::from_str(yaml).unwrap();
        println!("{:#}", serde_yaml::to_string(&res).unwrap());
        assert_eq!(
            res,
            Shape::Sphere {
                base: BaseShape {
                    material: Material {
                        color: Mapping::Uniform(Rgb(1., 1., 0.0)),
                        specular: Mapping::Pattern(PatternMapping::Stripes {
                            values: vec![0.0, 1.0],
                            transform: Transforms::default(),
                        }),
                        ..Material::default()
                    },
                    transform: Transforms::ChainedTransform(vec![Transforms::SingleTransform(
                        Transform::Scaling(1., 2., 3.)
                    )]),
                }
            }
        );
    }

    #[test]
    fn test_simple_camera() {
        let yaml = r#"
---
from: [10, 10, 10]
"#;
        let res: Camera = serde_yaml::from_str(yaml).unwrap();
        println!("{:#}", serde_yaml::to_string(&res).unwrap());
        assert_eq!(
            res,
            Camera {
                size: (1600, 1200),
                field_of_view: Angle::FPi4,
                from: Point(10.0, 10.0, 10.0),
                to: Point(0.0, 0.0, 0.0),
                up: Vector(0.0, 1.0, 0.0),
            }
        );
    }
}
