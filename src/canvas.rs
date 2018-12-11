use crate::*;

#[derive(Debug)]
pub struct Canvas {
  pub width: usize,
  pub height: usize,
  pub frame_buffer: Vec<u8>,
}

pub fn canvas(width: usize, height: usize) -> Canvas {
  Canvas::new(width, height)
}

impl Canvas {
  pub fn new(width: usize, height: usize) -> Canvas {
    let mut frame_buffer = Vec::with_capacity(width * height * 3);
    frame_buffer.resize(width * height * 3, u8::default());
    Canvas {
      width,
      height,
      frame_buffer,
    }
  }

  fn idx(&self, x: usize, y: usize) -> usize {
    debug_assert!(x < self.width);
    debug_assert!(y < self.height);
    3 * (x + y * self.width)
  }

  pub fn set(&mut self, x: usize, y: usize, c: ColorRgbByte) {
    let start = self.idx(x, y);
    self.frame_buffer[start] = c.r;
    self.frame_buffer[start + 1] = c.g;
    self.frame_buffer[start + 2] = c.b;
  }

   pub fn set_idx(&mut self, idx: usize, c: ColorRgbByte) {
    self.frame_buffer[idx] = c.r;
    self.frame_buffer[idx + 1] = c.g;
    self.frame_buffer[idx + 2] = c.b;
  }

  pub fn get(&self, x: usize, y: usize) -> ColorRgbByte {
    let start = self.idx(x, y);
    ColorRgbByte {
      r: self.frame_buffer[start],
      g: self.frame_buffer[start + 1],
      b: self.frame_buffer[start +2],
    }
  }

  pub fn to_string(&self) -> String {
    self
      .frame_buffer
      .chunks(10)
      .map(|chunk| {
        chunk
          .iter()
          .map(|byte| byte.to_string())
          .collect::<Vec<String>>()
          .join(" ")
      })
      .collect::<Vec<String>>()
      .join("\n")
  }

  pub fn to_ppm_string(&self) -> String {
    let header = format!("P3\n{} {}\n255\n", self.width, self.height);
    header + &self.to_string() + "\n"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_canvas() {
    let mut can = Canvas::new(5, 3);
    assert!(can.frame_buffer.iter().all(|&c| c == u8::default()));
    can.set(0, 0, color(0.5, 0., 1.).into());
    let buffer = can.to_ppm_string();
    println!("{}", buffer);
    assert_eq!(
      buffer,
      "P3
5 3
255
128 0 255 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0
0 0 0 0 0
"
    );
  }
}
