use crate::*;
use bvh::bvh::BVH;
use core::fmt::Debug;

pub struct Group {
    base: BaseShape,
    bounded_shapes: Vec<BoundedShape>,
    bounds: Bounds,
    bvh: Option<BVH>,
}

impl Debug for Group {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Group {{ base: {:?}, bounded_shapes: {:?}, bounds: {:?} }}",
            self.base, self.bounded_shapes, self.bounds
        )
    }
}

impl Group {
    pub fn new(transform: Transform, material: Material) -> Group {
        Group {
            base: BaseShape::new(transform, material),
            bounded_shapes: vec![],
            bounds: no_bounds(),
            bvh: None,
        }
    }

    pub fn add_shape(&mut self, mut shape: Box<dyn Shape + Send>) {
        shape.set_parent(self);
        let transform = &shape.get_transform();
        self.bounds = bounds_reducer(
            self.bounds,
            transform_bounds(&shape.get_bounds(), transform),
        );

        let bounded_shape = BoundedShape::new(shape);
        self.bounded_shapes.push(bounded_shape);
    }
}

impl Default for Group {
    fn default() -> Group {
        Group::new(Transform::identity(), Material::default())
    }
}

impl Shape for Group {
    fn shape_added(&mut self) {
        for bs in &mut self.bounded_shapes {
            bs.get_shape_mut().shape_added();
        }
        self.bvh = Some(BVH::build(&mut self.bounded_shapes));
    }

    fn get_bounds(&self) -> Bounds {
        self.bounds
    }

    fn get_base(&self) -> &BaseShape {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }

    fn local_normal_at(&self, _local_point: &Point, _intersection: &Intersection) -> UnitVector {
        panic!("Local normal called for group.")
    }

    fn local_intersects(&self, ray: &Ray) -> Option<Intersection> {
        bvh_intersects(self.bvh.as_ref().unwrap(), &self.bounded_shapes, ray)
            .filter_map(|s| s.get_shape().intersects(ray))
            .min_by(|min, x| f32::partial_cmp(&min.t, &x.t).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_group() {
        let g = Group::default();
        let t = g.get_transform_inverse();
        assert_eq!(t, Transform::identity());
        let s = g.bounded_shapes;
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn add_shape_to_group() {
        let mut g = Group::default();
        let s = Sphere::default();
        let a_s = Box::new(s);
        g.add_shape(a_s);
    }
}
