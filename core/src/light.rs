use crate::*;
use rand::*;

#[derive(Debug, Clone)]
pub enum Attenuation {
    None,
    Linear,
    Squared,
}

impl Attenuation {
    pub fn calculate(&self, distance: f32) -> f32 {
        match self {
            Attenuation::None => 1.,
            Attenuation::Linear => 10. / distance,
            Attenuation::Squared => 100. / (distance * distance),
        }
    }
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
    pub fn lighting(&self, object_hit: &Hit, world: &World) -> ColorRgbFloat {
        let material = object_hit.intersection.object.get_material();
        let hm = material.get_hit_material(object_hit);

        match self {
            Light::Point(point_light) => point_light.lighting(&hm, world),
            Light::Directional(directional_light) => directional_light.lighting(&hm, world),
            Light::Area(area_light) => area_light.lighting(&hm, world),
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

    pub fn lighting(&self, hm: &HitMaterial, world: &World) -> ColorRgbFloat {
        let light_vector = self.direction;

        let mut sum = hm.color * hm.ambient;
        let light_hit = LightHit {
            lightv: light_vector,
            distance: std::f32::INFINITY,
            intensity: self.intensity,
            point: hm.hit.point,
        };

        if !world.is_shadowed(&light_hit) {
            sum = sum + hm.shading(&light_hit);
        }
        sum
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

    pub fn lighting(&self, hm: &HitMaterial, world: &World) -> ColorRgbFloat {
        let light_vector = self.position - hm.hit.point;
        let distance = magnitude(&light_vector);

        let mut sum = hm.color * hm.ambient;
        let light_hit = LightHit {
            lightv: unit_vector_from_vector(light_vector / distance),
            distance,
            intensity: self.intensity,
            point: hm.hit.point,
        };

        if !world.is_shadowed(&light_hit) {
            sum = sum + hm.shading(&light_hit);
        }
        sum
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

    pub fn lighting(&self, hm: &HitMaterial, world: &World) -> ColorRgbFloat {
        let steps = (self.u_steps, self.v_steps);
        let u_vec = self.u_vec / (self.u_steps as f32);
        let v_vec = self.v_vec / (self.u_steps as f32);
        let vecs = (u_vec, v_vec);

        let acc = BLACK;
        self.lighting_rec(acc, hm, world, steps, vecs, 4, 4) + hm.color * hm.ambient
    }

    fn lighting_rec(
        &self,
        mut acc: ColorRgbFloat,
        hm: &HitMaterial,
        world: &World,
        steps: (u8, u8),
        vecs: (Vector, Vector),
        depth: u8,
        max_depth: u8,
    ) -> ColorRgbFloat {
        if depth == 0 {
            return acc;
        }

        let (u_steps, v_steps) = steps;
        let (u_vec, v_vec) = vecs;

        let hit_point = hm.hit.point;
        let ligh_corner_vector = self.position - hit_point;

        let jitter = self.jitter;
        let frac = 1. / (max_depth as f32 * u_steps as f32 * v_steps as f32 * jitter as f32);
        let mut rng = rand::thread_rng();

        let mut shadowed = 0;
        let mut not_shadowed = 0;
        let mut val = BLACK;
        for u in 0..u_steps {
            for v in 0..v_steps {
                for _ in 0..jitter {
                    let ru = rng.gen_range(0., 1.0);
                    let rv = rng.gen_range(0., 1.0);

                    let light_vector =
                        ligh_corner_vector + u_vec * (u as f32 + ru) + v_vec * (v as f32 + rv);
                    let distance = magnitude(&light_vector);

                    let light_hit = LightHit {
                        lightv: unit_vector_from_vector(light_vector / distance),
                        distance,
                        intensity: self.intensity * frac,
                        point: hit_point,
                    };

                    if world.is_shadowed(&light_hit) {
                        shadowed += 1;
                    } else {
                        not_shadowed += 1;
                        val = val + hm.shading(&light_hit);
                    }
                }
            }
        }
        acc = acc + val;
        if shadowed == 0 || not_shadowed == 0 {
            return acc * ((max_depth as f32) / (1. + max_depth as f32 - depth as f32));
        } else {
            self.lighting_rec(acc, hm, world, steps, vecs, depth - 1, max_depth)
        }
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
