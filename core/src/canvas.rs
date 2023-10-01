use std::sync::{Mutex, MutexGuard};

use image::ImageBuffer;

use crate::*;

const N_CHANNELS: usize = 3;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub frame_buffer: Mutex<Vec<u8>>,
}

pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas::new(width, height)
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut frame_buffer = Vec::with_capacity(width * height * N_CHANNELS);
        frame_buffer.resize(width * height * N_CHANNELS, u8::default());
        Canvas {
            width,
            height,
            frame_buffer: Mutex::new(frame_buffer),
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        N_CHANNELS * (x + y * self.width)
    }

    pub fn set(&self, x: usize, y: usize, c: ColorRgbByte) {
        let start = self.idx(x, y);
        self.set_idx(start, c);
    }

    pub fn set_idx(&self, idx: usize, c: ColorRgbByte) {
        let mut fb = self.get_frame_buffer();
        fb[idx] = c.r;
        fb[idx + 1] = c.g;
        fb[idx + 2] = c.b;
    }

    pub fn get(&self, x: usize, y: usize) -> ColorRgbByte {
        let fb = self.get_frame_buffer();
        let start = self.idx(x, y);
        ColorRgbByte {
            r: fb[start],
            g: fb[start + 1],
            b: fb[start + 2],
        }
    }

    fn buffer_as_ppm_string(&self) -> String {
        let fb = self.get_frame_buffer();
        fb.chunks(10).map(|chunk| {
            chunk.iter().map(|byte| byte.to_string()).collect::<Vec<String>>().join(" ")
        }).collect::<Vec<String>>().join("\n")
    }

    pub fn to_ppm_string(&self) -> String {
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        header + &self.buffer_as_ppm_string() + "\n"
    }

    fn get_frame_buffer(&self) -> MutexGuard<Vec<u8>> {
        self.frame_buffer.lock().unwrap()
    }

    pub fn save(&self, filename: &str) {
        let image: ImageBuffer<image::Rgb<u8>, Vec<u8>> = self.into();
        image.save(filename).unwrap();
    }
}

impl From<&Canvas> for ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    fn from(canvas: &Canvas) -> Self {
        let mut img = ImageBuffer::new(canvas.width as u32, canvas.height as u32);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let c = canvas.get(x as usize, y as usize);
            *pixel = image::Rgb([c.r, c.g, c.b]);
        }
        img
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let can = Canvas::new(5, 3);
        assert!(can.get_frame_buffer().iter().all(|&c| c == u8::default()));
        can.set(0, 0, color(0.5, 0., 1.).into());
        let buffer = can.to_ppm_string();
        println!("{}", buffer);
        assert_eq!(
            buffer,
            "P3
5 3
255
186 0 255 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0
0 0 0 0 0
"
        );
    }

    #[test]
    fn write_image() {
        let canvas = Canvas::new(5, 3);
        canvas.set(0, 0, color(0.5, 0., 0.).into());
        canvas.set(2, 1, color(0., 0.5, 0.).into());
        assert_eq!(canvas.get(0, 0), color(0.5, 0., 0.).into());
        canvas.save("test.png");
        std::fs::remove_file("test.png").unwrap();
    }
}
