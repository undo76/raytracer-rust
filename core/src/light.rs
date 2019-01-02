use crate::*;
use rand::*;

#[derive(Debug, Clone)]
pub enum Attenuation {
    None,
    Linear,
    Squared,
}

pub struct LightHit {
    pub lightv: UnitVector,
    pub distance: f32,
    pub point: Point,
    pub intensity: ColorRgbFloat,
}

pub enum Light {
    Point(PointLight),
    Directional(DirectionalLight),
    Area(AreaLight),
}

impl Light {
    pub fn visibility(&self, world: &World, hit_point: Point) -> f32 {
        match self {
            Light::Point(point_light) => point_light.visibility(world, hit_point),
            Light::Directional(directional_light) => directional_light.visibility(world, hit_point),
            Light::Area(area_light) => area_light.visibility(world, hit_point),
        }
    }

    // TODO: Remove the Box heap allocation
    pub fn hits(&self, hit_point: Point) -> Box<dyn Iterator<Item = LightHit>> {
        match self {
            Light::Point(point_light) => Box::new(std::iter::once(point_light.hit(hit_point))),
            Light::Directional(directional_light) => {
                Box::new(std::iter::once(directional_light.hit(hit_point)))
            }
            Light::Area(area_light) => Box::new(area_light.hits(hit_point)),
        }
    }

    pub fn lighting<F>(&self, object_hit: &Hit, is_not_shadowed: F) -> ColorRgbFloat
    where
        F: Fn(&LightHit) -> bool,
    {
        object_hit.intersection.object.get_material().lighting(
            object_hit.intersection.object,
            &self,
            &object_hit.point,
            &object_hit.eyev,
            &object_hit.normalv,
            is_not_shadowed,
        )
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

    fn visibility(&self, world: &World, hit_point: Point) -> f32 {
        if world.is_shadowed(&self.hit(hit_point)) {
            0.0
        } else {
            1.0
        }
    }

    pub fn hit(&self, hit_point: Point) -> LightHit {
        LightHit {
            lightv: self.direction,
            distance: std::f32::INFINITY,
            intensity: self.intensity,
            point: hit_point,
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

    fn visibility(&self, world: &World, hit_point: Point) -> f32 {
        let light_hit = &self.hit(hit_point);
        if world.is_shadowed(&light_hit) {
            0.0
        } else {
            calculate_attenuation(&self.attenuation, light_hit.distance)
        }
    }

    fn hit(&self, hit_point: Point) -> LightHit {
        let light_vector = self.position - hit_point;
        let distance = magnitude(&light_vector);
        LightHit {
            lightv: unit_vector_from_vector(light_vector / distance),
            distance,
            intensity: self.intensity,
            point: hit_point,
        }
    }
}

#[derive(Debug)]
pub struct AreaLight {
    pub position: Point,
    pub intensity: ColorRgbFloat,
    pub attenuation: Attenuation,
    u_vec: Vector,
    v_vec: Vector,
    u_steps: u8,
    v_steps: u8,
    jitter: u8,
}

impl AreaLight {
    pub fn new(
        position: Point,
        intensity: ColorRgbFloat,
        uv: (Vector, Vector),
        steps: (u8, u8),
        jitter: u8,
    ) -> AreaLight {
        AreaLight {
            u_vec: uv.0,
            v_vec: uv.1,
            u_steps: steps.0,
            v_steps: steps.1,
            jitter,
            position,
            intensity,
            attenuation: Attenuation::None,
        }
    }

    fn visibility(&self, world: &World, hit_point: Point) -> f32 {
        self.hits(hit_point)
            .map(|light_hit| {
                if world.is_shadowed(&light_hit) {
                    0.0
                } else {
                    calculate_attenuation(&self.attenuation, light_hit.distance)
                        / (self.u_steps as f32 * self.v_steps as f32 * self.jitter as f32)
                }
            })
            .sum()
    }

    fn hits(&self, hit_point: Point) -> impl Iterator<Item = LightHit> {
        let ligh_corner_vector = self.position - hit_point;
        let u_vec = self.u_vec / (self.u_steps as f32);
        let v_vec = self.v_vec / (self.u_steps as f32);

        let u_steps = self.u_steps;
        let v_steps = self.v_steps;
        let jitter = self.jitter;
        let intensity = self.intensity;
        let frac = 1. / (u_steps as f32 * v_steps as f32 * jitter as f32);
        (0..u_steps).flat_map(move |u| {
            (0..v_steps).flat_map(move |v| {
                (0..jitter).map(move |_| {
                    let ru = rand::thread_rng().gen_range(0., 1.0);
                    let rv = rand::thread_rng().gen_range(0., 1.0);
                    let light_vector =
                        ligh_corner_vector + u_vec * (u as f32 + ru) + v_vec * (v as f32 + rv);
                    let distance = magnitude(&light_vector);

                    LightHit {
                        lightv: unit_vector_from_vector(light_vector / distance),
                        distance,
                        intensity: intensity * frac,
                        point: hit_point,
                    }
                })
            })
        })
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
