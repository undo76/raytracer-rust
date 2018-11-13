use super::geom::*;

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

#[cfg(test)]
mod tests {
  use super::*;

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
}
