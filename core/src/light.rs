use crate::*;

#[derive(Debug, Clone)]
pub enum Attenuation {
    None,
    Linear,
    Squared,
}

pub struct LightHit {
    pub lightv: UnitVector,
    pub distance: f32,
    pub intensity: ColorRgbFloat,
}

pub enum Light {
    Point(PointLight),
    Directional(DirectionalLight),
}

impl Light {
    #[inline(always)]
    pub fn hit(&self, hit_point: &Point) -> LightHit {
        match self {
            Light::Point(light) => {
                let light_vector = light.position - hit_point;
                let distance = magnitude(&light_vector);
                let attenuation = calculate_attenuation(&light.attenuation, distance);
                LightHit {
                    lightv: unit_vector_from_vector(&light_vector / distance),
                    intensity: light.intensity * attenuation,
                    distance: distance,
                }
            }
            Light::Directional(light) => LightHit {
                lightv: light.direction,
                intensity: light.intensity,
                distance: std::f32::INFINITY,
            },
        }
    }
}

#[derive(Debug)]
pub struct DirectionalLight {
    pub direction: UnitVector,
    pub intensity: ColorRgbFloat,
}

impl DirectionalLight {
    pub fn new(direction: UnitVector, intensity: ColorRgbFloat) -> DirectionalLight {
        DirectionalLight {
            direction,
            intensity,
        }
    }
}

#[derive(Debug)]
pub struct PointLight {
    pub position: Point,
    pub intensity: ColorRgbFloat,
    pub attenuation: Attenuation,
}

impl PointLight {
    pub fn new(position: Point, intensity: ColorRgbFloat) -> PointLight {
        PointLight {
            position,
            intensity,
            attenuation: Attenuation::None,
        }
    }
}

fn calculate_attenuation(attenuation: &Attenuation, distance: f32) -> f32 {
    match attenuation {
        Attenuation::None => 1.,
        Attenuation::Linear => 10. / distance,
        Attenuation::Squared => 100. / (distance * distance),
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn point_light_creation() {
        let light = PointLight::new(point(0., 0., 0.), WHITE);
        assert_eq!(light.position, point(0., 0., 0.,));
        assert_eq!(light.intesity, WHITE);
    }
}
