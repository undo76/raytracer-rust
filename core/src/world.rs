use bvh::bvh::BVH;

use crate::*;

pub struct World {
    pub bounded_shapes: Vec<BoundedShape>,
    pub lights: Vec<Light>,
    bvh: BVH,
}

impl World {
    pub fn new(shapes: Vec<Box<dyn Shape + Send>>, lights: Vec<Light>) -> World {
        let mut bounded_shapes = shapes
            .into_iter()
            .map(|mut s| {
                s.shape_added();
                BoundedShape::new(s)
            })
            .collect::<Vec<_>>();

        let bvh = BVH::build(&mut bounded_shapes);
        World {
            bounded_shapes,
            lights,
            bvh,
        }
    }

    fn ray_in_shadow<'a>(
        &'a self,
        ray: &Ray,
        light_distance: f32,
        cached_shape: Option<&'a (dyn Shape)>,
    ) -> Option<Intersection> {
        let it = cached_shape.into_iter();
        let it2 = bvh_intersects(&self.bvh, &self.bounded_shapes, ray)
            .map(|s| s.get_shape() as &dyn Shape);
        it.chain(it2)
            .filter_map(|s| s.intersects(ray))
            .find(|x| x.t < light_distance)
    }

    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        bvh_intersects(&self.bvh, &self.bounded_shapes, ray)
            .filter_map(|s| s.get_shape().intersects(ray))
            .min_by(|min, x| f32::partial_cmp(&min.t, &x.t).unwrap())
    }

    pub fn is_shadowed<'a>(
        &'a self,
        light_hit: &LightHit,
        cached_shape: Option<&'a (dyn Shape)>,
    ) -> Option<&(dyn Shape)> {
        let LightHit {
            lightv,
            distance,
            point,
            ..
        } = light_hit;
        let r = Ray::new(
            *point + lightv.into_inner() * 100. * EPS,
            lightv.into_inner(),
        );
        self.ray_in_shadow(&r, *distance, cached_shape)
            .map(|i| i.object)
    }

    fn shade_hit(&self, object_hit: &Hit, remaining: u8) -> ColorRgbFloat {
        let surface: ColorRgbFloat = self
            .lights
            .iter()
            .map(|light| light.lighting(object_hit, self))
            .sum();

        let reflected = self.reflected_color(object_hit, remaining);
        let refracted = self.refracted_color(object_hit, remaining);

        let material = object_hit.intersection.object.get_material();
        if material.transparency.is_some() && material.reflective.is_some() {
            let reflectance = object_hit.schlick();
            surface + reflected * reflectance + refracted * (1. - reflectance)
        } else {
            surface + reflected + refracted
        }
    }

    pub fn color_at(&self, ray: &Ray, remaining: u8) -> ColorRgbFloat {
        let hit = self.intersects(ray);
        match hit {
            Some(h) => self.shade_hit(&h.prepare_hit(ray), remaining),
            None => BLACK,
        }
    }

    fn reflected_color(&self, hit: &Hit, remaining: u8) -> ColorRgbFloat {
        if remaining == 0 {
            BLACK
        } else {
            let object = hit.intersection.object;
            match &object.get_material().reflective {
                Some(reflective) => {
                    let reflectv = hit.reflectv.into_inner();
                    let reflect_ray = Ray::new(hit.point + reflectv * EPS * 100., reflectv);
                    self.color_at(&reflect_ray, remaining - 1)
                        * reflective.map_at_object(&hit.object_point)
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
                    let normal = hit.normalv.into_inner();
                    let direction =
                        normal * (n_ratio * cos_i - cos_t) - hit.eyev.into_inner() * n_ratio;
                    let origin = hit.point - (normal * EPS);
                    let refract_ray = Ray::new(origin + direction * EPS * 100., direction);

                    self.color_at(&refract_ray, remaining - 1)
                        * transparency.map_at_object(&hit.object_point)
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
        let s2 = Sphere::new(scaling(0.5, 0.5, 0.5), Material::default());

        World::new(
            vec![Box::new(s1), Box::new(s2)],
            vec![Light::Point(PointLight::new(point(-10., 10., -10.), WHITE))],
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
        let world = World {
            lights: vec![Light::Point(PointLight::new(
                point(0., 0.25, 0.),
                color(1., 1., 1.),
            ))],
            ..World::default()
        };
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
        let material = Material {
            ambient: Mapping::from(1.),
            diffuse: Mapping::from(0.),
            specular: Mapping::from(0.),
            ..Material::default()
        };
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
