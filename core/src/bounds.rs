use bvh::{Point3, Vector3};
use bvh::aabb::AABB;
use bvh::aabb::Bounded;
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use bvh::bvh::BVHNode;

use crate::*;

pub fn no_bounds() -> Bounds {
    (
        point(f32::INFINITY, f32::INFINITY, f32::INFINITY),
        point(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
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

pub fn bounds_intersects<'a>(shape: &'a dyn Shape, ray: &Ray) -> Option<Intersection<'a>> {
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

    if direction.abs() > f32::EPSILON {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        tmin = tmin_numerator * f32::INFINITY;
        tmax = tmax_numerator * f32::INFINITY;
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
        AABB::with_bounds(
            Point3::from_array(min.into()),
            Point3::from_array(max.into()),
        )
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

#[inline(always)]
pub fn bvh_intersects<'a>(
    bvh: &'a BVH,
    bounded_shapes: &'a [BoundedShape],
    ray: &Ray,
) -> impl Iterator<Item=&'a BoundedShape> {
    let origin: Point3 = Point3::from_array(ray.origin.into());
    let direction: Vector3 = Vector3::from_array(ray.direction.into());
    let bvh_ray = bvh::ray::Ray::new(origin, direction);
    bvh_iterator(bounded_shapes, &bvh.nodes, bvh_ray)
    // .map(move |index| &bounded_shapes[index])
}

#[inline(always)]
fn bvh_iterator<'a>(
    bounded_shapes: &'a [BoundedShape],
    nodes: &'a [BVHNode],
    ray: bvh::ray::Ray,
) -> BvhIterator<'a> {
    BvhIterator {
        bounded_shapes,
        nodes,
        ray,
        node_index: 0,
        traversal: NodeTraversal::FromParent,
    }
}

#[derive(Debug, Copy, Clone)]
enum NodeTraversal {
    FromParent,
    FromBottom(usize),
}

struct BvhIterator<'a> {
    bounded_shapes: &'a [BoundedShape],
    nodes: &'a [BVHNode],
    ray: bvh::ray::Ray,
    node_index: usize,
    traversal: NodeTraversal,
}

impl<'a> Iterator for BvhIterator<'a> {
    type Item = &'a BoundedShape;

    fn next(&mut self) -> Option<&'a BoundedShape> {
        let mut traversal = self.traversal;
        let mut node_index = self.node_index;

        loop {
            let node = &self.nodes[node_index];

            match node {
                BVHNode::Node {
                    ref child_l_aabb,
                    child_l_index,
                    ref child_r_aabb,
                    child_r_index,
                    parent_index,
                    ..
                } => match traversal {
                    NodeTraversal::FromParent => {
                        node_index = {
                            if self.ray.intersects_aabb(child_l_aabb) {
                                *child_l_index
                            } else if self.ray.intersects_aabb(child_r_aabb) {
                                *child_r_index
                            } else {
                                traversal = NodeTraversal::FromBottom(node_index);
                                *parent_index
                            }
                        }
                    }
                    NodeTraversal::FromBottom(child_index) => {
                        if *child_l_index == child_index && self.ray.intersects_aabb(child_r_aabb) {
                            traversal = NodeTraversal::FromParent;
                            node_index = *child_r_index;
                        } else if child_index == 0 {
                            return None;
                        } else {
                            traversal = NodeTraversal::FromBottom(node_index);
                            node_index = *parent_index
                        }
                    }
                },
                BVHNode::Leaf {
                    shape_index,
                    parent_index,
                    ..
                } => {
                    self.traversal = NodeTraversal::FromBottom(node_index);
                    self.node_index = *parent_index;
                    return Some(&self.bounded_shapes[*shape_index]);
                }
            }
        }
    }
}
