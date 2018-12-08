use crate::*;

use nalgebra as na;

#[derive(Debug, Clone)]
pub struct Material {
  pub pattern: Pattern<ColorRgbFloat>,
  pub ambient: f32,
  pub diffuse: f32,
  pub specular: f32,
  pub shininess: f32,
}

impl Material {
  pub fn lighting(
    &self,
    object: &dyn Shape,
    light: &PointLight,
    position: &Point,
    eyev: &na::Unit<Vector>,
    normalv: &na::Unit<Vector>,
    in_shadow: bool,
  ) -> ColorRgbFloat {
    let color = self.pattern.map_at_object(object, position);
    let effective_color = color * light.intensity;
    let lightv = normalize(&(light.position - position));
    let light_dot_normal = normalv.dot(&lightv);
    let reflectv = reflect(&-lightv, normalv);
    let reflect_dot_eye = f32::powf(dot(&reflectv, eyev), self.shininess);

    let mut total = effective_color * self.ambient;

    if !in_shadow && light_dot_normal > 0. {
      total = total + effective_color * self.diffuse * light_dot_normal;
      if reflect_dot_eye > 0. {
        total = total + light.intensity * self.specular * reflect_dot_eye;
      }
    }
    return total;
  }
}

impl Default for Material {
  fn default() -> Self {
    Material {
      pattern: Pattern::Uniform(UniformPattern { value: WHITE }),
      ambient: 0.1,
      diffuse: 0.9,
      specular: 0.9,
      shininess: 200.,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lighting_eye_between_light_and_surface() {
    let position = point(0., 0., 0.);
    let eyev = na::Unit::new_normalize(vector(0., 0., -1.));
    let normalv = na::Unit::new_normalize(vector(0., 0., -1.));
    let light = PointLight::new(point(0., 0., -10.), WHITE);
    let m = Material::default();
    let sphere = Sphere::default();
    let result = m.lighting(&sphere, &light, &position, &eyev, &normalv, false);
    assert_relative_eq!(result, color(1.9, 1.9, 1.9));
  }

  #[test]
  fn lighting_eye_between_light_offset_45deg() {
    let position = point(0., 0., 0.);
    let eyev = na::Unit::new_normalize(vector(0., 0., -1.));
    let normalv = na::Unit::new_normalize(vector(0., 0., -1.));
    let light = PointLight::new(point(0., 10., -10.), WHITE);
    let m = Material::default();
    let sphere = Sphere::default();
    let result = m.lighting(&sphere, &light, &position, &eyev, &normalv, false);
    assert_relative_eq!(result, color(0.7363961, 0.7363961, 0.7363961));
  }

  #[test]
  fn lighting_eye_in_reflection_vector() {
    let position = point(0., 0., 0.);
    let eyev = na::Unit::new_normalize(vector(0., -f32::sqrt(2.) / 2., -f32::sqrt(2.) / 2.));
    let normalv = na::Unit::new_normalize(vector(0., 0., -1.));
    let light = PointLight::new(point(0., 10., -10.), WHITE);
    let m = Material::default();
    let sphere = Sphere::default();
    let result = m.lighting(&sphere, &light, &position, &eyev, &normalv, false);
    assert_relative_eq!(result, color(1.636396, 1.636396, 1.636396));
  }

  #[test]
  fn lighting_light_behind() {
    let position = point(0., 0., 0.);
    let eyev = na::Unit::new_normalize(vector(0., 0., -1.));
    let normalv = na::Unit::new_normalize(vector(0., 0., -1.));
    let light = PointLight::new(point(0., 0., 10.), WHITE);
    let m = Material::default();
    let sphere = Sphere::default();
    let result = m.lighting(&sphere, &light, &position, &eyev, &normalv, false);
    assert_relative_eq!(result, color(0.1, 0.1, 0.1));
  }

  #[test]
  fn light_with_surface_in_shadow() {
    let position = point(0., 0., 0.);
    let eyev = na::Unit::new_normalize(vector(0., 0., -1.));
    let normalv = na::Unit::new_normalize(vector(0., 0., -1.));
    let light = PointLight::new(point(0., 0., -10.), color(1., 1., 1.));
    let in_shadow = true;
    let m = Material::default();
    let sphere = Sphere::default();
    let result = m.lighting(&sphere, &light, &position, &eyev, &normalv, in_shadow);
    assert_relative_eq!(result, color(0.1, 0.1, 0.1));
  }
}
