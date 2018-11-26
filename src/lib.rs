#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod canvas;
pub mod color;
pub mod geom;
pub mod light;
pub mod material;
pub mod ray;
pub mod shape;
pub mod transform;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_relative_eq!(2. + 2., 4.);
  }
}
