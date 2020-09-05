use crate::*;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Mapping<ColorRgbFloat>,
    pub ambient: Mapping<f32>,
    pub diffuse: Mapping<f32>,
    pub specular: Mapping<f32>,
    pub shininess: Mapping<f32>,
    pub reflective: Option<Mapping<f32>>,
    pub transparency: Option<Mapping<f32>>,
    pub refractive_index: f32,
    pub attenuation: Attenuation,
}

impl Material {
    pub fn get_hit_material<'a>(&self, hit: &'a Hit) -> HitMaterial<'a> {
        let object_point = &hit.object_point;
        HitMaterial {
            hit,
            color: self.color.map_at_object(object_point),
            ambient: self.ambient.map_at_object(object_point),
            diffuse: self.diffuse.map_at_object(object_point),
            specular: self.specular.map_at_object(object_point),
            shininess: self.shininess.map_at_object(object_point),
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Mapping::from(WHITE),
            ambient: Mapping::from(0.1),
            diffuse: Mapping::from(0.9),
            specular: Mapping::from(0.9),
            shininess: Mapping::from(200.),
            reflective: None,
            transparency: None,
            refractive_index: 1.0,
            attenuation: Attenuation::None,
        }
    }
}

pub struct HitMaterial<'a> {
    pub hit: &'a Hit<'a>,
    pub color: ColorRgbFloat,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl<'a> HitMaterial<'a> {
    pub fn shading(
        &self,
        &LightHit {
            lightv, intensity, ..
        }: &LightHit,
    ) -> ColorRgbFloat {
        let Hit { eyev, normalv, .. } = self.hit;
        let light_dot_normal = dot(&lightv, normalv);
        let mut total = BLACK;
        if light_dot_normal > 0. {
            let reflectv = reflect(&-lightv, normalv);
            total = total + self.color * intensity * self.diffuse * light_dot_normal;

            let reflect_dot_eye = dot(&reflectv, eyev);
            if reflect_dot_eye > 0. {
                total = total + intensity * self.specular * reflect_dot_eye.powf(self.shininess);
            }
        }
        return total;
    }
}
