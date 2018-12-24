#[cfg(test)]
#[macro_use]
extern crate approx;

pub use crate::camera::*;
pub use crate::canvas::*;
pub use crate::color::*;
pub use crate::cube::*;
pub use crate::cylinder::*;
pub use crate::geom::*;
pub use crate::group::*;
pub use crate::light::*;
pub use crate::mapping::*;
pub use crate::material::*;
pub use crate::plane::*;
pub use crate::ray::*;
pub use crate::shape::*;
pub use crate::sphere::*;
pub use crate::transform::*;
pub use crate::triangle::*;
pub use crate::world::*;

mod camera;
mod canvas;
mod color;
mod cube;
mod cylinder;
mod geom;
mod group;
mod light;
mod mapping;
mod material;
mod plane;
mod ray;
mod shape;
mod sphere;
mod transform;
mod triangle;
mod world;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_relative_eq!(2. + 2., 4.);
    }
}
