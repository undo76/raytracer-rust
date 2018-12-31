use crate::*;
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;

#[derive(Debug)]
pub enum NormalType {
    Uniform(UnitVector),
    Smooth(UnitVector, UnitVector, UnitVector),
}

#[derive(Debug)]
pub struct Triangle {
    parent: AtomicPtr<Group>,
    p1: Point,
    e1: Vector,
    e2: Vector,
    normal: NormalType,
}

impl Triangle {
    pub fn add_to_group(group: &mut Group, points: &[(Point, Option<UnitVector>)]) {
        debug_assert!(points.len() >= 3);
        let p1 = points[0].0;
        let n1 = points[0].1;
        for index in 1..(points.len() - 1) {
            let e1 = points[index].0 - p1;
            let e2 = points[index + 1].0 - p1;
            let n2 = points[index].1;
            let n3 = points[index + 1].1;
            let normal = match (n1, n2, n3) {
                (Some(n1), Some(n2), Some(n3)) => NormalType::Smooth(n1, n2, n3),
                _ => NormalType::Uniform(normalize(&cross(&e1, &e2))),
            };
            let t = Triangle {
                parent: AtomicPtr::new(&mut *group),
                p1,
                e1,
                e2,
                normal,
            };
            group.add_shape(Box::new(t));
        }
    }
}

impl Shape for Triangle {
    fn get_bounds(&self) -> Bounds {
        let p1 = self.p1;
        let p2 = p1 + self.e1;
        let p3 = p1 + self.e2;

        let min_x = p1.x.min(p2.x).min(p3.x);
        let min_y = p1.y.min(p2.y).min(p3.y);
        let min_z = p1.z.min(p2.z).min(p3.z);
        let max_x = p1.x.max(p2.x).max(p3.x);
        let max_y = p1.y.max(p2.y).max(p3.y);
        let max_z = p1.z.max(p2.z).max(p3.z);
        (point(min_x, min_y, min_z), point(max_x, max_y, max_z))
    }

    fn get_base(&self) -> &BaseShape {
        unimplemented!()
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        unimplemented!()
    }

    fn local_normal_at(&self, _local_point: &Point, hit: &Intersection) -> UnitVector {
        match self.normal {
            NormalType::Uniform(n) => n,
            NormalType::Smooth(n1, n2, n3) => {
                let (u, v) = hit.uv.unwrap();
                normalize(
                    &(n2.into_inner() * u + n3.into_inner() * v + n1.into_inner() * (1. - u - v)),
                )
            }
        }
    }

    fn local_intersects(&self, _ray: &Ray) -> Option<Intersection> {
        unimplemented!("Triangles don't need local_intersects")
    }

    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
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
            Some(Intersection::new_with_uv(t, self, (u, v)))
        } else {
            None
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
