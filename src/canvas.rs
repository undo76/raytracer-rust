use super::color::*;

pub struct Canvas {
  width: usize,
  height: usize,
  pixels: Vec<Color>,
}

impl Canvas {
  pub fn set(&mut self, x: usize, y: usize, c: Color) {
    self.pixels[x + y * self.width] = c;
  }

  pub fn get(&self, x: usize, y: usize) -> Color {
    self.pixels[x + y * self.width]
  }
}

pub fn canvas(width: usize, height: usize) -> Canvas {
  let mut pixels = Vec::with_capacity(width * height);
  pixels.resize(width * height, Color::default());

  Canvas {
    width,
    height,
    pixels,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_canvas() {
    let mut can = canvas(10, 20);
    assert!(can.pixels.iter().all(|c| *c == Color::default() ));
    can.set(0, 0, color(1., 1., 1.));
    assert_eq!(can.get(0,0), color(1., 1., 1.));
    let p = &can;
    p.get(0,0);
  }
}
