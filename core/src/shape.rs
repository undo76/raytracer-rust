use crate::*;
use core::fmt::Debug;
use core::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;

#[derive(Debug)]
pub struct BaseShape {
    transform_inverse: Transform,
    material: Material,
    parent: AtomicPtr<Group>,
}

impl BaseShape {
    pub fn new(transform: Transform, material: Material) -> BaseShape {
        BaseShape {
            transform_inverse: transform.inverse(),
            material,
            parent: AtomicPtr::new(core::ptr::null_mut()),
        }
    }
}

pub trait Shape: Debug + Sync + Send {
    fn shape_added(&mut self) {
        ()
    }
    fn get_bounds(&self) -> Bounds;
    fn get_base(&self) -> &BaseShape;
    fn get_base_mut(&mut self) -> &mut BaseShape;
    fn local_intersects(&self, local_ray: &Ray) -> Option<Intersection>;
    fn local_normal_at(&self, point: &Point, intersection: &Intersection) -> UnitVector;

    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let local_ray = ray.transform(&self.get_transform_inverse());
        self.local_intersects(&local_ray)
    }

    fn normal_at(&self, point: &Point, intersection: &Intersection) -> UnitVector {
        let local_point = self.world_to_object(point);
        let local_normal = self.local_normal_at(&local_point, intersection);
        self.normal_to_world(&local_normal)
    }

    fn world_to_object(&self, point: &Point) -> Point {
        let parent = self.get_parent();
        let point = match parent {
            Some(parent) => parent.world_to_object(point),
            None => *point,
        };

        let t_inv = self.get_transform_inverse();
        t_inv * point
    }

    fn normal_to_world(&self, local_normal: &Vector) -> UnitVector {
        let t_inv = self.get_transform_inverse();
        let mut world_normal = t_inv.matrix().transpose() * local_normal.to_homogeneous();
        world_normal[3] = 0.;
        let normal = UnitVector::new_normalize(Vector::from_homogeneous(world_normal).unwrap());

        let parent = self.get_parent();
        match parent {
            Some(parent) => parent.normal_to_world(&normal.into_inner()),
            None => normal,
        }
    }

    fn get_material(&self) -> &Material {
        &self.get_base().material
    }

    fn set_material(&mut self, material: Material) {
        self.get_base_mut().material = material
    }

    fn set_transform(&mut self, trans: Transform) {
        self.get_base_mut().transform_inverse = trans.inverse();
    }

    fn get_transform(&self) -> Transform {
        self.get_base().transform_inverse.inverse()
    }

    fn get_transform_inverse(&self) -> Transform {
        self.get_base().transform_inverse
    }

    fn get_parent(&self) -> Option<&Group> {
        unsafe { self.get_base().parent.load(Ordering::Relaxed).as_ref() }
    }

    fn set_parent(&mut self, group: &mut Group) {
        self.get_base_mut().parent = AtomicPtr::new(group);
    }
}
