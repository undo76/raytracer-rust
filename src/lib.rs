#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod geom;
pub mod color;
pub mod canvas;
pub mod transform;
pub mod ray;
pub mod shape;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_relative_eq!(2. + 2., 4.);
    }
}
