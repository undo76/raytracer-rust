extern crate raytracer_rust;

use raytracer_rust::canvas::*;
use raytracer_rust::color::*;
use raytracer_rust::geom::*;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Projectile {
  position: Point,
  velocity: Vector,
}

#[derive(Debug)]
struct World {
  gravity: Vector,
  wind: Vector,
}

fn tick(w: &World, p: &Projectile) -> Projectile {
  Projectile {
    position: p.position + p.velocity,
    velocity: p.velocity + w.gravity + w.wind,
  }
}

#[test]
fn one_tick() {
  let p = Projectile {
    position: point(0., 0., 0.),
    velocity: vector(10., 10., 0.),
  };
  let w = World {
    gravity: vector(0., 1., 0.),
    wind: vector(0., 0., 0.),
  };
  tick(&w, &p);
}

fn main() {
  let mut p = Projectile {
    position: point(0., 1., 0.),
    velocity: normalize(&vector(1., 1.8, 0.)) * 11.25,
  };
  let w = World {
    gravity: vector(0., -0.1, 0.),
    wind: vector(-0.01, 0., 0.),
  };

  let mut c = canvas(900, 550);
  while p.position.y >= 0. {
    c.set(
      p.position.x as usize,
      550 - p.position.y as usize,
      color(1., 1., 1.).into(),
    );
    p = tick(&w, &p);
  }
  let mut file = File::create("projectile.ppm").expect("Couldn't create file");
  file
    .write_all(c.to_ppm_string().as_bytes())
    .expect("Couldn't write canvas");
}
