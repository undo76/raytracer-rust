#[macro_use]
extern crate approx;

mod geom;
mod color;
mod canvas;
mod transform;
mod ray;
mod clock;
// mod projectile;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
