#[cfg(test)]
#[macro_use]
extern crate approx;

pub use crate::camera::*;
pub use crate::canvas::*;
pub use crate::color::*;
pub use crate::cube::*;
pub use crate::geom::*;
pub use crate::light::*;
pub use crate::material::*;
pub use crate::mapping::*;
pub use crate::plane::*;
pub use crate::ray::*;
pub use crate::shape::*;
pub use crate::sphere::*;
pub use crate::transform::*;
pub use crate::world::*;

mod camera;
mod canvas;
mod color;
mod cube;
mod geom;
mod light;
mod material;
mod mapping;
mod plane;
mod ray;
mod shape;
mod sphere;
mod transform;
mod world;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_relative_eq!(2. + 2., 4.);
  }
}
