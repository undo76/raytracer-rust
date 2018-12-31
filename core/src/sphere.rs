use crate::*;
use nalgebra as na;

#[derive(Debug)]
pub struct Sphere {
    base: BaseShape,
}

impl Sphere {
    pub fn new(transform: Transform, material: Material) -> Sphere {
        Sphere {
            base: BaseShape::new(transform, material),
        }
    }
}

impl Default for Sphere {
    fn default() -> Sphere {
        Sphere::new(Transform::identity(), Material::default())
    }
}

impl Shape for Sphere {
    fn get_bounds(&self) -> Bounds {
        (point(-1., -1., -1.), point(1., 1., 1.))
    }

    fn get_base(&self) -> &BaseShape {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }

    fn local_normal_at(&self, local_point: &Point, _intersection: &Intersection) -> UnitVector {
        na::Unit::new_unchecked(local_point - point(0., 0., 0.))
    }

    fn local_intersects(&self, ray: &Ray) -> Option<Intersection> {
        let sphere_to_ray = ray.origin - point(0., 0., 0.);
        let a = dot(&ray.direction, &ray.direction);
        let b = 2. * dot(&ray.direction, &sphere_to_ray);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.;
        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            return None;
        } else {
            let sqrt_disc = f32::sqrt(discriminant);
            let mut t1 = (-b - sqrt_disc) / (2. * a);
            let mut t2 = (-b + sqrt_disc) / (2. * a);
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            if t1 > EPS {
                Some(Intersection::new(t1, self))
            } else if t2 > EPS {
                Some(Intersection::new(t2, self))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra as na;

    #[test]
    fn ray_intersects_sphere_default() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let s = Sphere::default();
        assert!(s.intersects(&r).is_some());
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = s.intersects(&r).unwrap();
        assert_relative_eq!(xs.t, 4.);
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let r = Ray::new(point(0., 1., -5.), vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = s.intersects(&r).unwrap();
        assert_relative_eq!(xs.t, 5.);
    }

    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray::new(point(0., 2., -5.), vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = s.intersects(&r);
        assert!(xs.is_none());
    }

    #[test]
    fn ray_originates_inside_a_sphere() {
        let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = s.intersects(&r).unwrap();
        assert_relative_eq!(xs.t, 1.);
    }

    #[test]
    fn intesections() {
        let s1 = Sphere::default();
        let s2 = Sphere::default();
        let xs = vec![
            Intersection::new(-1., &s1),
            Intersection::new(1., &s2),
            Intersection::new(2., &s2),
        ];
        let h = hit(&xs);
        assert!(std::ptr::eq(h.unwrap(), &xs[1]));
    }

    #[test]
    fn intesections_none() {
        let s1 = Sphere::default();
        let s2 = Sphere::default();
        let xs = vec![
            Intersection::new(-1., &s1),
            Intersection::new(-2., &s2),
            Intersection::new(-3., &s2),
        ];
        let h = hit(&xs);
        assert!(h.is_none());
    }

    #[test]
    fn sphere_default_transform() {
        let s = Sphere::default();
        assert_eq!(s.get_transform(), Transform::identity())
    }

    #[test]
    fn sphere_transform() {
        let mut s = Sphere::default();
        s.set_transform(na::convert(translation(2., 3., 4.)));
        assert_eq!(s.get_transform(), na::convert(translation(2., 3., 4.)))
    }

    #[test]
    fn intersect_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = Sphere::default();
        s.set_transform(na::convert(scaling(2., 2., 2.)));
        let xs = s.intersects(&r).unwrap();
        assert_relative_eq!(xs.t, 3.);
    }

    #[test]
    fn intersect_a_translated_sphere_with_a_ray() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = Sphere::default();
        s.set_transform(na::convert(translation(5., 0., 0.)));
        let xs = s.intersects(&r);
        assert!(xs.is_none());
    }

    #[test]
    fn normal_sphere_axis() {
        let s = Sphere::default();
        let n = s.normal_at(&point(1., 0., 0.), &Intersection::new(1., &s));
        assert_relative_eq!(n.into_inner(), vector(1., 0., 0.));
        let n = s.normal_at(&point(0., 1., 0.), &Intersection::new(1., &s));
        assert_relative_eq!(n.into_inner(), vector(0., 1., 0.));
        let n = s.normal_at(&point(0., 0., 1.), &Intersection::new(1., &s));
        assert_relative_eq!(n.into_inner(), vector(0., 0., 1.));
    }

    #[test]
    fn normal_sphere_translated() {
        let mut s = Sphere::default();
        s.set_transform(na::convert(translation(0., 1., 0.)));
        let n = s.normal_at(
            &point(0., 1.70710677, -0.70710677),
            &Intersection::new(1., &s),
        );
        assert_relative_eq!(n.into_inner(), vector(0., 0.70710677, -0.70710677));
    }

    #[test]
    fn normal_sphere_scaled() {
        let mut s = Sphere::default();
        s.set_transform(na::convert(scaling(1., 0.5, 1.)));
        let n = s.normal_at(
            &point(0., 0.70710677, -0.70710677),
            &Intersection::new(1., &s),
        );
        assert_relative_eq!(n.into_inner(), vector(0., 0.97014254, -0.24253564));
    }

    #[test]
    fn assign_material() {
        let mut s = Sphere::default();
        let mut m = Material::default();
        m.ambient = Mapping::from(1.);
        s.set_material(m);
        // assert_eq!(s.get_material().ambient, 1.);
    }

    #[test]
    fn precompute_state_of_intersection() {
        let ray = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let shape = Sphere::default();
        let intersection = Intersection::new(4., &shape);
        let hit = intersection.prepare_hit(&ray);
        assert_relative_eq!(hit.point, point(0., 0., -1.));
        assert_relative_eq!(hit.eyev.into_inner(), vector(0., 0., -1.));
        assert_relative_eq!(hit.normalv.into_inner(), vector(0., 0., -1.));
    }

    #[test]
    fn precompute_state_of_intersection_inside() {
        let ray = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let shape = Sphere::default();
        let intersection = Intersection::new(1., &shape);
        let hit = intersection.prepare_hit(&ray);
        assert_relative_eq!(hit.point, point(0., 0., 1.));
        assert_relative_eq!(hit.eyev.into_inner(), vector(0., 0., -1.));
        assert_relative_eq!(hit.normalv.into_inner(), vector(0., 0., -1.));
        assert_eq!(hit.inside, true);
    }
}
