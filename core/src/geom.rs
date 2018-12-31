use nalgebra as na;

pub const EPS: f32 = 1.0e-4;

pub type Point = na::Point3<f32>;
pub type Vector = na::Vector3<f32>;
pub type UnitVector = na::Unit<Vector>;

#[inline]
pub fn point(x: f32, y: f32, z: f32) -> Point {
    Point::new(x, y, z)
}

#[inline]
pub fn vector(x: f32, y: f32, z: f32) -> Vector {
    Vector::new(x, y, z)
}

#[inline]
pub fn unit_vector(x: f32, y: f32, z: f32) -> UnitVector {
    na::Unit::new_unchecked(vector(x, y, z))
}

#[inline]
pub fn magnitude(v: &Vector) -> f32 {
    na::Matrix::norm(v)
}

#[inline]
pub fn normalize(v: &Vector) -> UnitVector {
    na::Unit::new_normalize(*v)
}

#[inline]
pub fn dot(v1: &Vector, v2: &Vector) -> f32 {
    na::Matrix::dot(v1, v2)
}

#[inline]
pub fn cross(v1: &Vector, v2: &Vector) -> Vector {
    v1.cross(&v2)
}

#[inline]
pub fn reflect(v: &Vector, n: &na::Unit<Vector>) -> Vector {
    v - n.into_inner() * 2. * dot(v, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substracting_two_points() {
        let p1 = point(3., 2., 1.);
        let p2 = point(5., 6., 7.);
        assert_relative_eq!(p1 - p2, vector(-2., -4., -6.));
    }

    #[test]
    fn substracting_vector_from_point() {
        let p = point(3., 2., 1.);
        let v = vector(5., 6., 7.);
        assert_relative_eq!(p - v, point(-2., -4., -6.));
    }

    #[test]
    fn substracting_two_vectors() {
        let v1 = point(3., 2., 1.);
        let v2 = point(5., 6., 7.);
        assert_relative_eq!(v1 - v2, vector(-2., -4., -6.));
    }

    #[test]
    fn magnitude_of_100() {
        assert_relative_eq!(magnitude(&vector(1., 0., 0.)), 1.);
    }

    #[test]
    fn magnitude_of_123() {
        assert_relative_eq!(magnitude(&vector(1., 2., 3.)), (14.0_f32).sqrt());
    }

    #[test]
    fn normalizing_vector() {
        assert_relative_eq!(
            normalize(&vector(4., 0., 0.)).into_inner(),
            vector(1., 0., 0.)
        );
    }

    #[test]
    fn dot_product() {
        let v1 = vector(1., 2., 3.);
        let v2 = vector(2., 3., 4.);
        assert_relative_eq!(dot(&v1, &v2), 20.)
    }

    #[test]
    fn cross_product() {
        let v1 = vector(1., 2., 3.);
        let v2 = vector(2., 3., 4.);
        assert_relative_eq!(cross(&v1, &v2), vector(-1., 2., -1.));
        assert_relative_eq!(cross(&v2, &v1), vector(1., -2., 1.));
    }

    #[test]
    fn reflection() {
        let v = vector(1., -1., 0.);
        let n = vector(0., 1., 0.);
        let r = reflect(&v, &na::Unit::new_unchecked(n));
        assert_relative_eq!(r, vector(1., 1., 0.));
    }

    #[test]
    fn reflection_slated() {
        let v = vector(0., -1., 0.);
        let n = vector(f32::sqrt(2.) / 2., f32::sqrt(2.) / 2., 0.);
        let r = reflect(&v, &na::Unit::new_unchecked(n));
        assert_relative_eq!(r, vector(1., 0., 0.));
    }
}
