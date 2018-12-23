use self::Mapping::*;
use crate::*;

#[derive(Debug, Clone)]
pub struct UniformMapping<T> {
    pub value: T,
}

#[derive(Debug, Clone)]
pub struct StripeMapping<T> {
    pub values: Vec<T>,
    pub transform_inverse: Transform,
}

#[derive(Debug, Clone)]
pub struct CheckersMapping<T> {
    pub values: Vec<T>,
    pub transform_inverse: Transform,
}

#[derive(Debug, Clone)]
pub struct GradientMapping<T> {
    pub values: (T, T),
    pub transform_inverse: Transform,
}

#[derive(Debug, Clone)]
pub struct RingMapping<T> {
    pub values: Vec<T>,
    pub transform_inverse: Transform,
}

pub trait MappingMapping<T>
where
    T: Copy,
{
    fn get_transform_inverse(&self) -> &Transform;
    fn map_at(&self, pattern_point: &Point) -> T;
    fn map_at_object(&self, object_point: &Point) -> T {
        let pattern_point = self.get_transform_inverse() * object_point;
        self.map_at(&pattern_point)
    }
}

impl<T> MappingMapping<T> for StripeMapping<T>
where
    T: Copy,
{
    fn get_transform_inverse(&self) -> &Transform {
        &self.transform_inverse
    }
    fn map_at(&self, pattern_point: &Point) -> T {
        let n = self.values.len() as isize;
        let idx = ((pattern_point.x + EPS).floor() as isize % n + n) % n;
        self.values[idx as usize]
    }
}

impl<T> MappingMapping<T> for CheckersMapping<T>
where
    T: Copy,
{
    fn get_transform_inverse(&self) -> &Transform {
        &self.transform_inverse
    }
    fn map_at(&self, pattern_point: &Point) -> T {
        let n = self.values.len() as isize;
        let idx_x = (pattern_point.x + EPS).floor() as isize % n + n;
        let idx_y = (pattern_point.y + EPS).floor() as isize % n + n;
        let idx_z = (pattern_point.z + EPS).floor() as isize % n + n;
        let idx = (idx_x + idx_y + idx_z) % n;
        self.values[idx as usize]
    }
}

impl<T> MappingMapping<T> for GradientMapping<T>
where
    T: Copy
        + core::ops::Sub<Output = T>
        + core::ops::Add<Output = T>
        + core::ops::Mul<f32, Output = T>,
{
    fn get_transform_inverse(&self) -> &Transform {
        &self.transform_inverse
    }
    fn map_at(&self, pattern_point: &Point) -> T {
        let distance = self.values.1 - self.values.0;
        let fraction = pattern_point.x - (pattern_point.x - EPS).floor();
        self.values.0 + distance * fraction
    }
}

impl<T> MappingMapping<T> for RingMapping<T>
where
    T: Copy
        + core::ops::Sub<Output = T>
        + core::ops::Add<Output = T>
        + core::ops::Mul<f32, Output = T>,
{
    fn get_transform_inverse(&self) -> &Transform {
        &self.transform_inverse
    }
    fn map_at(&self, pattern_point: &Point) -> T {
        let (x, z) = (pattern_point.x, pattern_point.z);
        let n = self.values.len() as isize;
        let distance = (x * x + z * z).sqrt().floor();
        let idx = distance as isize % n;
        self.values[idx as usize]
    }
}

#[derive(Debug, Clone)]
pub enum Mapping<T: Copy> {
    Uniform(UniformMapping<T>),
    Striped(StripeMapping<T>),
    Gradient(GradientMapping<T>),
    Ring(RingMapping<T>),
    Checkered(CheckersMapping<T>),
}
impl<T> Mapping<T>
where
    T: Copy
        + core::ops::Sub<Output = T>
        + core::ops::Add<Output = T>
        + core::ops::Mul<f32, Output = T>,
{
    pub fn uniform(value: T) -> Self {
        Uniform(UniformMapping { value })
    }

    pub fn stripes(values: &[T], transform: Transform) -> Self {
        Striped(StripeMapping {
            values: values.to_vec(),
            transform_inverse: transform.inverse(),
        })
    }

    pub fn rings(values: &[T], transform: Transform) -> Self {
        Ring(RingMapping {
            values: values.to_vec(),
            transform_inverse: transform.inverse(),
        })
    }

    pub fn checkers(values: &[T], transform: Transform) -> Self {
        Checkered(CheckersMapping {
            values: values.to_vec(),
            transform_inverse: transform.inverse(),
        })
    }

    pub fn gradient(values: (T, T), transform: Transform) -> Self {
        Gradient(GradientMapping {
            values,
            transform_inverse: transform.inverse(),
        })
    }

    pub fn map_at_object(&self, object_point: &Point) -> T {
        use self::Mapping::*;
        match self {
            Uniform(u) => u.value,
            Striped(s) => s.map_at_object(&object_point),
            Gradient(g) => g.map_at_object(&object_point),
            Ring(r) => r.map_at_object(&object_point),
            Checkered(c) => c.map_at_object(&object_point),
        }
    }
}

impl<T> From<T> for Mapping<T>
where
    T: Copy
        + core::ops::Sub<Output = T>
        + core::ops::Add<Output = T>
        + core::ops::Mul<f32, Output = T>,
{
    fn from(value: T) -> Mapping<T> {
        Mapping::uniform(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = Mapping::stripes(&vec![WHITE, BLACK], Transform::identity());
        assert_eq!(pattern.map_at_object(&point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.map_at_object(&point(0., 0., 2.)), WHITE);
        assert_eq!(pattern.map_at_object(&point(0., 0., 3.)), WHITE);
    }

    #[test]
    fn stripe_pattern_alternates_in_z() {
        let pattern = Mapping::stripes(&vec![WHITE, BLACK], Transform::identity());
        assert_eq!(pattern.map_at_object(&point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.map_at_object(&point(0.9, 0., 0.)), WHITE);
        assert_eq!(pattern.map_at_object(&point(1., 0., 0.)), BLACK);
        assert_eq!(pattern.map_at_object(&point(-0.1, 0., 0.)), BLACK);
        assert_eq!(pattern.map_at_object(&point(-1., 0., 0.)), BLACK);
        assert_eq!(pattern.map_at_object(&point(-1.1, 0., 0.)), WHITE);
    }
}
