use crate::*;
use nalgebra as na;

#[derive(Debug)]
pub struct Camera {
  hsize: usize,
  vsize: usize,
  field_of_view: f32,
  transform: Transform,
  half_width: f32,
  half_height: f32,
  pixel_size: f32,
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
    }
  }
}

#[inline]
fn view_transform(from: Point, to: Point, up: Vector) -> Transform {
  let forward = normalize(&(to - from));
  let upn = normalize(&up);
  let left = cross(&forward, &upn);
  let true_up = cross(&left, &forward);
  let translation = translation(-from.x, -from.y, -from.z);
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
}