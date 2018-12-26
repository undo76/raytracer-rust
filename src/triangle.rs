use crate::*;
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;
use std::sync::Arc;

#[derive(Debug)]
pub struct Triangle {
    parent: AtomicPtr<Group>,
    p1: Point,
    e1: Vector,
    e2: Vector,
    normal: UnitVector,
}

impl Triangle {
    pub fn add_to_group(group: &mut Group, points: &[Point]) {
        debug_assert!(points.len() >= 3);
        let p1 = points[0];
        for index in 1..(points.len() - 1) {
            let e1 = points[index] - p1;
            let e2 = points[index + 1] - p1;
            let normal = normalize(&cross(&e1, &e2));
            let t = Triangle {
                parent: AtomicPtr::new(&mut *group),
                p1,
                e1,
                e2,
                normal,
            };
            group.add_shape(Arc::new(t));
        }
    }
}

impl Shape for Triangle {
    fn get_base(&self) -> &BaseShape {
        unimplemented!()
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        unimplemented!()
    }

    fn local_normal_at(&self, _local_point: &Point) -> UnitVector {
        self.normal
    }

    fn local_intersects(&self, ray: &Ray) -> Option<Intersection> {
        let dir_cross_e2 = cross(&ray.direction, &self.e2);
        let det = dot(&self.e1, &dir_cross_e2);
        if f32::abs(det) < core::f32::EPSILON {
            return None;
        }
        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.p1;
        let u = f * dot(&p1_to_origin, &dir_cross_e2);
        if u < 0. || u > 1. {
            return None;
        }
        let origin_cross_e1 = cross(&p1_to_origin, &self.e1);
        let v = f * dot(&ray.direction, &origin_cross_e1);
        if v < 0. || (u + v) > 1. {
            return None;
        }
        let t = f * dot(&self.e2, &origin_cross_e1);
        if t > EPS {
            return Some(Intersection::new(t, self));
        } else {
            return None;
        }
    }

    fn get_material(&self) -> &Material {
        self.get_parent().unwrap().get_material()
    }

    fn set_material(&mut self, _material: Material) {
        unimplemented!()
    }

    fn set_transform(&mut self, _trans: Transform) {
        unimplemented!()
    }

    fn get_transform(&self) -> Transform {
        Transform::identity()
    }

    fn get_transform_inverse(&self) -> Transform {
        Transform::identity()
    }

    fn get_parent(&self) -> Option<&Group> {
        unsafe { self.parent.load(Ordering::Relaxed).as_ref() }
    }

    fn set_parent(&mut self, group: &mut Group) {
        self.parent = AtomicPtr::new(group);
    }
}
