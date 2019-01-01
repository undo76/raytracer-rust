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
    pub fn lighting(
        &self,
        object: &dyn Shape,
        light: &Light,
        position: &Point,
        eyev: &UnitVector,
        normalv: &UnitVector,
        light_visibility: f32,
    ) -> ColorRgbFloat {
        let object_point = object.get_transform_inverse() * position;
        let color = self.color.map_at_object(&object_point);
        let ambient = self.ambient.map_at_object(&object_point);
        let diffuse = self.diffuse.map_at_object(&object_point);
        let specular = self.specular.map_at_object(&object_point);
        let shininess = self.shininess.map_at_object(&object_point);

        let sum = if light_visibility > 0.0 {
            light
                .hits(position.clone())
                .map(
                    |LightHit {
                         lightv, intensity, ..
                     }| {
                        let light_dot_normal = dot(&lightv, &normalv);

                        let intensity = intensity * light_visibility;

                        let mut total = BLACK;
                        if light_dot_normal > 0. {
                            let reflectv = reflect(&-lightv, normalv);
                            total = total + color * intensity * diffuse * light_dot_normal;

                            let reflect_dot_eye = dot(&reflectv, eyev);
                            if reflect_dot_eye > 0. {
                                total =
                                    total + intensity * specular * reflect_dot_eye.powf(shininess);
                            }
                        }
                        return total;
                    },
                )
                .fold(BLACK, |sum, c| sum + c)
        } else {
            BLACK
        };
        sum + color * ambient
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lighting_eye_between_light_and_surface() {
        let position = point(0., 0., 0.);
        let eyev = unit_vector_from_vector(vector(0., 0., -1.));
        let normalv = unit_vector_from_vector(vector(0., 0., -1.));
        let light = Light::Point(PointLight::new(point(0., 0., -10.), WHITE));
        let m = Material::default();
        let sphere = Sphere::default();
        let result = m.lighting(&sphere, &light, &position, &eyev, &normalv, 1.0);
        assert_relative_eq!(result, color(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_eye_between_light_offset_45deg() {
        let position = point(0., 0., 0.);
        let eyev = unit_vector_from_vector(vector(0., 0., -1.));
        let normalv = unit_vector_from_vector(vector(0., 0., -1.));
        let light = Light::Point(PointLight::new(point(0., 10., -10.), WHITE));
        let m = Material::default();
        let sphere = Sphere::default();
        let result = m.lighting(&sphere, &light, &position, &eyev, &normalv, 1.0);
        assert_relative_eq!(result, color(0.7363961, 0.7363961, 0.7363961));
    }

    #[test]
    fn lighting_eye_in_reflection_vector() {
        let position = point(0., 0., 0.);
        let eyev = unit_vector_from_vector(vector(0., -f32::sqrt(2.) / 2., -f32::sqrt(2.) / 2.));
        let normalv = unit_vector_from_vector(vector(0., 0., -1.));
        let light = Light::Point(PointLight::new(point(0., 10., -10.), WHITE));
        let m = Material::default();
        let sphere = Sphere::default();
        let result = m.lighting(&sphere, &light, &position, &eyev, &normalv, 1.0);
        assert_relative_eq!(result, color(1.6363853, 1.6363853, 1.6363853));
    }

    #[test]
    fn lighting_light_behind() {
        let position = point(0., 0., 0.);
        let eyev = unit_vector_from_vector(vector(0., 0., -1.));
        let normalv = unit_vector_from_vector(vector(0., 0., -1.));
        let light = Light::Point(PointLight::new(point(0., 0., 10.), WHITE));
        let m = Material::default();
        let sphere = Sphere::default();
        let result = m.lighting(&sphere, &light, &position, &eyev, &normalv, 1.0);
        assert_relative_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn light_with_surface_in_shadow() {
        let position = point(0., 0., 0.);
        let eyev = unit_vector_from_vector(vector(0., 0., -1.));
        let normalv = unit_vector_from_vector(vector(0., 0., -1.));
        let light = Light::Point(PointLight::new(point(0., 0., -10.), color(1., 1., 1.)));
        let m = Material::default();
        let sphere = Sphere::default();
        let result = m.lighting(&sphere, &light, &position, &eyev, &normalv, 0.0);
        assert_relative_eq!(result, color(0.1, 0.1, 0.1));
    }
}
