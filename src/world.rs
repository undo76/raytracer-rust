use crate::*;
use bvh::bvh::BVH;
use nalgebra as na;

pub struct World {
    pub bounded_shapes: Vec<BoundedShape>,
    pub lights: Vec<PointLight>,
    bvh: BVH,
}

impl World {
    pub fn new(shapes: Vec<Box<dyn Shape + Send>>, lights: Vec<PointLight>) -> World {
        let mut bounded_shapes = shapes
            .into_iter()
            .map(|mut s| {
                s.shape_added();
                BoundedShape::new(s)
            })
            .collect::<Vec<_>>();

        let bvh = BVH::build(&mut bounded_shapes);
        let world = World {
            bounded_shapes,
            lights,
            bvh,
        };
        world
    }

    fn intersects_bvh(&self, ray: &Ray) -> Vec<&BoundedShape> {
        let bvh_ray = bvh::ray::Ray::new(ray.origin, ray.direction);
        self.bvh.traverse(&bvh_ray, &self.bounded_shapes)
    }

    fn ray_in_shadow(&self, ray: &Ray, light_distance: f32) -> Option<Intersection> {
        self.intersects_bvh(&ray)
            .iter()
            .filter_map(|s| s.get_shape().intersects(ray))
            .find(|x| x.t < light_distance)
    }

    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        self.intersects_bvh(&ray)
            .iter()
            .filter_map(|s| s.get_shape().intersects(ray))
            .min_by(|min, x| f32::partial_cmp(&min.t, &x.t).unwrap())
    }

    fn is_shadowed(&self, light: &PointLight, point: &Point) -> bool {
        let v = light.position - point;
        let distance = magnitude(&v);
        let direction = normalize(&v).unwrap();
        let r = Ray::new(*point + direction * 10. * EPS, direction);
        self.ray_in_shadow(&r, distance).is_some()
    }

    fn shade_hit(&self, hit: &Hit, remaining: u8) -> ColorRgbFloat {
        let surface: ColorRgbFloat = self
            .lights
            .iter()
            .map(|light| {
                let in_shadow = self.is_shadowed(&light, &hit.point);
                hit.intersection.object.get_material().lighting(
                    hit.intersection.object,
                    &light,
                    &hit.point,
                    &hit.eyev,
                    &hit.normalv,
                    in_shadow,
                )
            })
            .sum();

        let reflected = self.reflected_color(hit, remaining);
        let refracted = self.refracted_color(hit, remaining);

        let material = hit.intersection.object.get_material();
        if material.transparency.is_some() && material.reflective.is_some() {
            let reflectance = hit.schlick();
            surface + reflected * reflectance + refracted * (1. - reflectance)
        } else {
            surface + reflected + refracted
        }
    }

    pub fn color_at(&self, ray: &Ray, remaining: u8) -> ColorRgbFloat {
        let hit = self.intersects(&ray);
        match hit {
            Some(h) => self.shade_hit(&h.prepare_hit(ray), remaining),
            None => BLACK,
        }
    }

    fn reflected_color(&self, hit: &Hit, remaining: u8) -> ColorRgbFloat {
        if remaining == 0 {
            return BLACK;
        } else {
            let object = hit.intersection.object;
            match &object.get_material().reflective {
                Some(reflective) => {
                    let reflect_ray = Ray::new(hit.point, hit.reflectv.unwrap());
                    let object_point = object.get_transform_inverse() * hit.point;
                    self.color_at(&reflect_ray, remaining - 1)
                        * reflective.map_at_object(&object_point)
                }
                None => BLACK,
            }
        }
    }

    fn refracted_color(&self, hit: &Hit, remaining: u8) -> ColorRgbFloat {
        if remaining == 0 {
            return BLACK;
        }

        let object = hit.intersection.object;
        let transparency = object.get_material().transparency.as_ref();

        match transparency {
            Some(transparency) => {
                let n_ratio = hit.n1 / hit.n2;
                let cos_i = dot(&hit.eyev, &hit.normalv);
                let sin2_t = n_ratio * n_ratio * (1. - cos_i * cos_i);
                if sin2_t > 1. {
                    // Internal reflection
                    BLACK
                } else {
                    let cos_t = f32::sqrt(1. - sin2_t);
                    let normal = hit.normalv.unwrap();
                    let direction =
                        normal * (n_ratio * cos_i - cos_t) - hit.eyev.unwrap() * n_ratio;
                    let origin = hit.point - (normal * EPS);
                    let refract_ray = Ray::new(origin, direction);

                    let object_point = object.get_transform_inverse() * hit.point;
                    self.color_at(&refract_ray, remaining - 1)
                        * transparency.map_at_object(&object_point)
                }
            }
            None => BLACK,
        }
    }
}

impl Default for World {
    fn default() -> World {
        let m1 = Material {
            color: Mapping::from(color(0.8, 1.0, 0.6)),
            diffuse: Mapping::from(0.7),
            specular: Mapping::from(0.2),
            ..Material::default()
        };

        let s1 = Sphere::new(Transform::identity(), m1);
        let s2 = Sphere::new(na::convert(scaling(0.5, 0.5, 0.5)), Material::default());

        World::new(
            vec![Box::new(s1), Box::new(s2)],
            vec![PointLight::new(point(-10., 10., -10.), WHITE)],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_world_with_ray() {
        let world = World::default();
        let ray = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let xs = world.intersects(&ray).unwrap();
        assert_relative_eq!(xs.t, 4.);
    }

    #[test]
    fn shade_intersection() {
        let world = World::default();
        let ray = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let xs = world.intersects(&ray).unwrap();
        let hit = xs.prepare_hit(&ray);
        let c = world.shade_hit(&hit, 0);
        assert_relative_eq!(c, color(0.38066125, 0.4758265, 0.28549594));
    }

    #[test]
    fn shade_intersection_inside() {
        let mut world = World::default();
        world.lights = vec![PointLight::new(point(0., 0.25, 0.), color(1., 1., 1.))];
        let ray = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let intersection = Intersection::new(0.5, &(*world.bounded_shapes[1].shape));
        let hit = intersection.prepare_hit(&ray);
        let c = world.shade_hit(&hit, 0);
        assert_relative_eq!(c, color(0.9049845, 0.9049845, 0.9049845));
    }

    #[test]
    fn color_at_intersection() {
        let world = World::default();
        let ray = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let c = world.color_at(&ray, 0);
        assert_relative_eq!(c, color(0.38066125, 0.4758265, 0.28549594));
    }

    #[test]
    fn color_at_behind() {
        let mut world = World::default();
        let mut material = Material::default();
        material.ambient = Mapping::from(1.);
        material.diffuse = Mapping::from(0.);
        material.specular = Mapping::from(0.);
        world.bounded_shapes[1].shape.set_material(material);
        let ray = Ray::new(point(0., 0., -0.75), vector(0., 0., 1.));
        let c = world.color_at(&ray, 0);
        assert_relative_eq!(
            c,
            world.bounded_shapes[1]
                .shape
                .get_material()
                .color
                .map_at_object(&point(0., 0., 0.))
        );
    }
}
