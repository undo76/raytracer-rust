use crate::*;

#[derive(Debug)]
pub struct Triangle {
    base: BaseShape,
    points: [Point; 3],
    e1: Vector,
    e2: Vector,
    normal: UnitVector,
}

impl Triangle {
    pub fn new(material: Material, points: [Point; 3]) -> Triangle {
        let e1 = points[1] - points[0];
        let e2 = points[2] - points[0];
        let normal = normalize(&cross(&(e1), &(e2)));
        Triangle {
            base: BaseShape::new(Transform::identity(), material),
            points,
            e1,
            e2,
            normal,
        }
    }
}

impl Shape for Triangle {
    fn get_base(&self) -> &BaseShape {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
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
        let p1_to_origin = ray.origin - self.points[0];
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
        return Some(Intersection::new(t, self));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_on_a_triangle() {
        let t = Triangle::new(
            Material::default(),
            [point(0., 1., 0.), point(-1., 0., 0.), point(1., 0., 0.)],
        );
        let n1 = t.local_normal_at(&point(0., 0.5, 0.));
        let n2 = t.local_normal_at(&point(-0.5, 0.75, 0.));
        let n3 = t.local_normal_at(&point(0.5, 0.25, 0.));
        assert_relative_eq!(n1, t.normal);
        assert_relative_eq!(n2, t.normal);
        assert_relative_eq!(n3, t.normal);
    }
}
