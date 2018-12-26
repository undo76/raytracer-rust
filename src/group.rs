use crate::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct Group {
    base: BaseShape,
    shapes: Vec<Arc<dyn Shape>>,
    bounds: Bounds,
}

impl Group {
    pub fn new(transform: Transform, material: Material) -> Group {
        Group {
            base: BaseShape::new(transform, material),
            shapes: vec![],
            bounds: no_bounds(),
        }
    }

    pub fn add_shape(&mut self, mut shape: Arc<dyn Shape>) {
        Arc::get_mut(&mut shape).unwrap().set_parent(self);
        let transform = &shape.get_transform();
        self.bounds = bounds_reducer(
            self.bounds,
            transform_bounds(&shape.get_bounds(), transform),
        );
        self.shapes.push(shape);
    }
}

impl Default for Group {
    fn default() -> Group {
        Group::new(Transform::identity(), Material::default())
    }
}

impl Shape for Group {
    fn get_bounds(&self) -> Bounds {
        self.bounds
    }

    fn get_base(&self) -> &BaseShape {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }

    fn local_normal_at(&self, _local_point: &Point) -> UnitVector {
        // unit_vector(0., 1., 0.)
        panic!("Local normal called for group.")
    }

    fn local_intersects(&self, ray: &Ray) -> Option<Intersection> {
        //return bounds_intersects(self, &ray);
        if bounds_intersects(self, &ray).is_none() {
            return None;
        }

        self.shapes
            .iter()
            .filter_map(|s| s.intersects(ray))
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
        let s = g.shapes;
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn add_shape_to_group() {
        let mut g = Group::default();
        let s = Sphere::default();
        let a_s = Arc::new(s);
        g.add_shape(a_s);
    }
}
