use crate::*;
use bvh::aabb::Bounded;
use bvh::aabb::AABB;
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;

pub fn no_bounds() -> Bounds {
    (
        point(
            core::f32::INFINITY,
            core::f32::INFINITY,
            core::f32::INFINITY,
        ),
        point(
            core::f32::NEG_INFINITY,
            core::f32::NEG_INFINITY,
            core::f32::NEG_INFINITY,
        ),
    )
}

pub type Bounds = (Point, Point);

pub fn transform_bounds(bounds: &Bounds, transform: &Transform) -> Bounds {
    let transformed = [
        transform * point(bounds.0.x, bounds.0.y, bounds.0.z),
        transform * point(bounds.0.x, bounds.0.y, bounds.1.z),
        transform * point(bounds.0.x, bounds.1.y, bounds.0.z),
        transform * point(bounds.0.x, bounds.1.y, bounds.1.z),
        transform * point(bounds.1.x, bounds.0.y, bounds.0.z),
        transform * point(bounds.1.x, bounds.0.y, bounds.1.z),
        transform * point(bounds.1.x, bounds.1.y, bounds.0.z),
        transform * point(bounds.1.x, bounds.1.y, bounds.1.z),
    ];

    transformed.iter().fold(no_bounds(), |(p1, p2), p| {
        (
            point(p1.x.min(p.x), p1.y.min(p.y), p1.z.min(p.z)),
            point(p2.x.max(p.x), p2.y.max(p.y), p2.z.max(p.z)),
        )
    })
}

pub fn bounds_reducer((p1_acc, p2_acc): Bounds, (p1, p2): Bounds) -> Bounds {
    (
        point(p1_acc.x.min(p1.x), p1_acc.y.min(p1.y), p1_acc.z.min(p1.z)),
        point(p2_acc.x.max(p2.x), p2_acc.y.max(p2.y), p2_acc.z.max(p2.z)),
    )
}

pub fn bounds_intersects<'a>(shape: &'a Shape, ray: &Ray) -> Option<Intersection<'a>> {
    let bounds = shape.get_bounds();
    let (xtmin, xtmax) = check_axis((bounds.0.x, bounds.1.x), ray.origin.x, ray.direction.x);
    let (ytmin, ytmax) = check_axis((bounds.0.y, bounds.1.y), ray.origin.y, ray.direction.y);
    let (ztmin, ztmax) = check_axis((bounds.0.z, bounds.1.z), ray.origin.z, ray.direction.z);

    let tmin = xtmin.max(ytmin).max(ztmin);
    let tmax = xtmax.min(ytmax).min(ztmax);

    if tmin > tmax {
        None
    } else if tmin > EPS {
        Some(Intersection::new(tmin, shape))
    } else if tmax > EPS {
        Some(Intersection::new(tmax, shape))
    } else {
        None
    }
}

fn check_axis(limits: (f32, f32), origin: f32, direction: f32) -> (f32, f32) {
    let tmin;
    let tmax;
    let tmin_numerator = limits.0 - origin;
    let tmax_numerator = limits.1 - origin;

    if direction.abs() > std::f32::EPSILON {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        tmin = tmin_numerator * std::f32::INFINITY;
        tmax = tmax_numerator * std::f32::INFINITY;
    }
    if tmin > tmax {
        return (tmax, tmin);
    }
    (tmin, tmax)
}

#[derive(Debug)]
pub struct BoundedShape {
    pub shape: Box<dyn Shape + Send>,
    bounds: (Point, Point),
    node_index: usize,
}

impl BoundedShape {
    pub fn new(shape: Box<dyn Shape + Send>) -> BoundedShape {
        let bounds = transform_bounds(&shape.get_bounds(), &shape.get_transform());

        BoundedShape {
            shape,
            bounds,
            node_index: 0,
        }
    }

    pub fn get_shape(&self) -> &(dyn Shape + Send) {
        self.shape.as_ref()
    }

    pub fn get_shape_mut(&mut self) -> &mut (dyn Shape + Send) {
        self.shape.as_mut()
    }
}

impl Bounded for BoundedShape {
    fn aabb(&self) -> AABB {
        let (min, max) = self.bounds;
        AABB::with_bounds(min, max)
    }
}

impl BHShape for BoundedShape {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

pub fn bvh_intersects<'a>(
    bvh: &BVH,
    bounded_shapes: &'a [BoundedShape],
    ray: &Ray,
) -> impl Iterator<Item = &'a BoundedShape> {
    let bvh_ray = bvh::ray::Ray::new(ray.origin, ray.direction);

    let mut indices = Vec::new();
    bvh::bvh::BVHNode::traverse_recursive(&bvh.nodes, 0, &bvh_ray, &mut indices);
    indices.into_iter().map(move |index| &bounded_shapes[index])
}
