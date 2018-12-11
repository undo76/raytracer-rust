use crate::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use nalgebra as na;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
  hsize: usize,
  vsize: usize,
  field_of_view: f32,
  transform: Transform,
  half_width: f32,
  half_height: f32,
  pixel_size: f32,
  max_reflects: u8,
}

impl Camera {
  pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera {
    let half_width;
    let half_height;
    let half_view = f32::tan(field_of_view / 2.);
    let aspect = (hsize as f32) / (vsize as f32);
    if aspect >= 1. {
      half_width = half_view;
      half_height = half_view / aspect;
    } else {
      half_width = half_view * aspect;
      half_height = half_view;
    }

    let pixel_size = half_width * 2.0 / (hsize as f32);

    Camera {
      hsize,
      vsize,
      field_of_view,
      transform: Transform::identity(),
      half_width,
      half_height,
      pixel_size,
      max_reflects: 5,
    }
  }

  pub fn set_transform(&mut self, transform: Transform) {
    self.transform = transform;
  }

  pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
    let x_offset = (x as f32 + 0.5) * self.pixel_size;
    let y_offset = (y as f32 + 0.5) * self.pixel_size;

    let world_x = self.half_width - x_offset;
    let world_y = self.half_height - y_offset;

    let transform_inv = inverse(&self.transform);
    let pixel = transform_inv * point(world_x, world_y, -1.);
    let origin = transform_inv * point(0., 0., 0.);
    let direction = normalize(&(pixel - origin));
    Ray::new(origin, direction)
  }

  pub fn render(self, world: World) -> Canvas {
    let canvas = Canvas::new(self.hsize, self.vsize);
    let canvas = Arc::new(Mutex::new(canvas));
    let world = Arc::new(world);
    let camera = Arc::new(self);

    let mut handles = vec![];

    let n_threads = 4;
    for i in 0..n_threads {
      let shared_canvas = Arc::clone(&canvas);
      let world = Arc::clone(&world);
      let camera = Arc::clone(&camera);
      let handle = thread::spawn(move || {
        for y in 0..camera.vsize {
          for x in (i..camera.hsize - n_threads + i + 1).step_by(n_threads) {
            let ray = camera.ray_for_pixel(x, y);
            let color = world.color_at(&ray, camera.max_reflects).into();
            let mut shared_canvas = shared_canvas.lock().unwrap();
            shared_canvas.set(x, y, color);
          }
        }
      });
      handles.push(handle);
    }

    for handle in handles {
      handle.join().unwrap();
    }

    return Arc::try_unwrap(canvas).unwrap().into_inner().unwrap();
  }
}

#[inline]
pub fn view_transform(from: Point, to: Point, up: Vector) -> Transform {
  let forward = normalize(&(to - from));
  let upn = normalize(&up);
  let left = cross(&forward, &upn);
  let true_up = cross(&left, &forward);
  let translation = translation(-from.x, -from.y, -from.z);

  #[rustfmt::skip]
  let orientation = Transform::from_matrix_unchecked(
    na::Matrix4::new(
      left.x, left.y, left.z, 0., 
      true_up.x, true_up.y, true_up.z, 0., 
      -forward.x, -forward.y, -forward.z, 0., 
      0., 0., 0., 1.,
    )
  );
  orientation * translation
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn view_transformation_looking_z() {
    let from = point(0., 0., 0.);
    let to = point(0., 0., 1.);
    let up = vector(0., 1., 0.);
    let t = view_transform(from, to, up);
    assert_relative_eq!(t, na::convert(scaling(-1., 1., -1.)));
  }

  #[test]
  fn view_transformation_moves_the_world() {
    let from = point(0., 0., 8.);
    let to = point(0., 0., 1.);
    let up = vector(0., 1., 0.);
    let t = view_transform(from, to, up);
    assert_relative_eq!(t, na::convert(translation(0., 0., -8.)));
  }

  #[test]
  fn construct_camera() {
    let hsize = 160;
    let vsize = 120;
    let field_of_view = na::Real::frac_pi_2();
    let camera = Camera::new(hsize, vsize, field_of_view);
    assert_eq!(camera.vsize, vsize);
    assert_eq!(camera.hsize, hsize);
    assert_relative_eq!(camera.field_of_view, field_of_view);
    assert_relative_eq!(camera.transform, Transform::identity());
  }

  #[test]
  fn pixel_size_horizontal() {
    let c = Camera::new(200, 125, na::Real::frac_pi_2());
    assert_relative_eq!(c.pixel_size, 0.01);
  }

  #[test]
  fn pixel_size_vertical() {
    let c = Camera::new(125, 200, na::Real::frac_pi_2());
    assert_relative_eq!(c.pixel_size, 0.01);
  }

  #[test]
  fn construct_ray_center() {
    let c = Camera::new(201, 101, na::Real::frac_pi_2());
    let r: Ray = c.ray_for_pixel(100, 50);
    assert_relative_eq!(r.origin, point(0., 0., 0.));
    assert_relative_eq!(r.direction, vector(0., 0., -1.));
  }

  #[test]
  fn construct_ray_corner() {
    let c = Camera::new(201, 101, na::Real::frac_pi_2());
    let r: Ray = c.ray_for_pixel(0, 0);
    assert_relative_eq!(r.origin, point(0., 0., 0.));
    assert_relative_eq!(r.direction, vector(0.6651864, 0.33259323, -0.66851234));
  }

  #[test]
  fn construct_ray_transformed() {
    let mut c = Camera::new(201, 101, na::Real::frac_pi_2());
    c.transform = na::convert(rotation_y(na::Real::frac_pi_4()) * translation(0., -2., 5.));
    let r: Ray = c.ray_for_pixel(100, 50);
    assert_relative_eq!(r.origin, point(0., 2., -5.));
    assert_relative_eq!(
      r.direction,
      vector(f32::sqrt(2.) / 2., 0., -f32::sqrt(2.) / 2.)
    );
  }

  #[test]
  fn render_default_world() {
    let world = World::default();
    let mut camera = Camera::new(11, 11, na::Real::frac_pi_2());
    let from = point(0., 0., -5.);
    let to = point(0., 0., 0.);
    let up = vector(0., 1., 0.);
    camera.transform = view_transform(from, to, up);
    let canvas = camera.render(world);
    assert_eq!(canvas.get(5, 5), color(0.38066, 0.47583, 0.2855).into());
  }
}
