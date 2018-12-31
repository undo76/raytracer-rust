use crate::*;

// Checks hit in radius <= 1 .
#[inline]
fn check_cap(ray: &Ray, t: f32) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;
    x * x + z * z <= 1.
}

#[derive(Debug)]
pub struct Cylinder {
    base: BaseShape,
    closed: bool,
}

impl Cylinder {
    pub fn new(transform: Transform, material: Material, closed: bool) -> Cylinder {
        Cylinder {
            base: BaseShape::new(transform, material),
            closed,
        }
    }

    fn intersect_side(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;
        if a.abs() < EPS {
            None
        } else {
            let b = 2. * ray.origin.x * ray.direction.x + 2. * ray.origin.z * ray.direction.z;
            let c = ray.origin.x * ray.origin.x + ray.origin.z * ray.origin.z - 1.;
            let disc = b * b - 4. * a * c;
            if disc < 0. {
                None
            } else {
                let sqrt_disc = disc.sqrt();
                let mut t1 = (-b - sqrt_disc) / (2. * a);
                let mut t2 = (-b + sqrt_disc) / (2. * a);

                if t2 < t1 {
                    std::mem::swap(&mut t1, &mut t2);
                }

                let y1 = ray.origin.y + t1 * ray.direction.y;
                let y2 = ray.origin.y + t2 * ray.direction.y;

                if t1 > EPS && -1. < y1 && y1 < 1. {
                    Some(Intersection::new(t1, self))
                } else if t2 > EPS && -1. < y2 && y2 < 1. {
                    Some(Intersection::new(t2, self))
                } else {
                    None
                }
            }
        }
    }

    fn intersect_caps(&self, ray: &Ray) -> Option<Intersection> {
        if !self.closed || ray.direction.y.abs() <= EPS {
            return None;
        }

        let mut t1 = (-1. - ray.origin.y) / ray.direction.y;
        let mut t2 = (1. - ray.origin.y) / ray.direction.y;
        if t2 < t1 {
            std::mem::swap(&mut t1, &mut t2);
        }

        if t1 > EPS && check_cap(ray, t1) {
            Some(Intersection::new(t1, self))
        } else if t2 > EPS && check_cap(ray, t2) {
            Some(Intersection::new(t2, self))
        } else {
            None
        }
    }
}

impl Default for Cylinder {
    fn default() -> Cylinder {
        Cylinder::new(Transform::identity(), Material::default(), true)
    }
}

impl Shape for Cylinder {
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
        let dist = local_point.x * local_point.x + local_point.z * local_point.z;

        if dist < 1. {
            if local_point.y >= 1. - EPS {
                return unit_vector(0., 1., 0.);
            } else if local_point.y <= -1. + EPS {
                return unit_vector(0., -1., 0.);
            }
        }
        unit_vector(local_point.x, 0., local_point.z)
    }

    fn local_intersects(&self, ray: &Ray) -> Option<Intersection> {
        let t_cap = self.intersect_caps(ray);
        let t_side = self.intersect_side(ray);

        match (&t_cap, &t_side) {
            (Some(Intersection { t: t1, .. }), Some(Intersection { t: t2, .. })) if t1 < t2 => {
                t_cap
            }
            (Some(_), Some(_)) => t_side,
            (Some(_), None) => t_cap,
            (None, Some(_)) => t_side,
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra as na;

    #[test]
    fn ray_intersects_cylinder_default() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let s = Cylinder::default();
        assert!(s.intersects(&r).is_some());
    }

    #[test]
    fn ray_intersects_cylinder_at_two_points() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let s = Cylinder::default();
        let xs = s.intersects(&r).unwrap();
        assert_relative_eq!(xs.t, 4.);
    }

    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray::new(point(0., 2., -5.), vector(0., 0., 1.));
        let s = Cylinder::default();
        let xs = s.intersects(&r);
        assert!(xs.is_none());
    }

    #[test]
    fn ray_originates_inside_a_sphere() {
        let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let s = Cylinder::default();
        let xs = s.intersects(&r).unwrap();
        assert_relative_eq!(xs.t, 1.);
    }

    #[test]
    fn intesections() {
        let s1 = Cylinder::default();
        let s2 = Cylinder::default();
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
        let s1 = Cylinder::default();
        let s2 = Cylinder::default();
        let xs = vec![
            Intersection::new(-1., &s1),
            Intersection::new(-2., &s2),
            Intersection::new(-3., &s2),
        ];
        let h = hit(&xs);
        assert!(h.is_none());
    }

    #[test]
    fn cylinder_default_transform() {
        let s = Cylinder::default();
        assert_eq!(s.get_transform(), Transform::identity())
    }

    #[test]
    fn cylinder_transform() {
        let mut s = Cylinder::default();
        s.set_transform(na::convert(translation(2., 3., 4.)));
        assert_eq!(s.get_transform(), na::convert(translation(2., 3., 4.)))
    }

    #[test]
    fn intersect_a_scaled_cylinder_with_a_ray() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = Cylinder::default();
        s.set_transform(na::convert(scaling(2., 2., 2.)));
        let xs = s.intersects(&r).unwrap();
        assert_relative_eq!(xs.t, 3.);
    }

    #[test]
    fn intersect_a_translated_cylinder_with_a_ray() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = Cylinder::default();
        s.set_transform(na::convert(translation(5., 0., 0.)));
        let xs = s.intersects(&r);
        assert!(xs.is_none());
    }

    #[test]
    fn normal_cylinder_axis() {
        let s = Cylinder::default();
        let n = s.normal_at(&point(1., 0., 0.), &Intersection::new(1., &s));
        assert_relative_eq!(n.into_inner(), vector(1., 0., 0.));
        let n = s.normal_at(&point(0., 0., 1.), &Intersection::new(1., &s));
        assert_relative_eq!(n.into_inner(), vector(0., 0., 1.));
    }
}
