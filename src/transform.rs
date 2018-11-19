use super::geom::*;
use nalgebra as na;

pub use nalgebra::inverse;

#[inline]
pub fn rotation_x(angle: f32) -> na::Rotation3<f32> {
  na::Rotation3::from_axis_angle(&na::Vector3::x_axis(), angle)
}

#[inline]
pub fn rotation_y(angle: f32) -> na::Rotation3<f32> {
  na::Rotation3::from_axis_angle(&na::Vector3::y_axis(), angle)
}

#[inline]
pub fn rotation_z(angle: f32) -> na::Rotation3<f32> {
  na::Rotation3::from_axis_angle(&na::Vector3::z_axis(), angle)
}

#[inline]
pub fn translation(x: f32, y: f32, z: f32) -> na::Translation3<f32> {
  na::Translation3::new(x, y, z)
}

#[inline]
pub fn scaling(x: f32, y: f32, z: f32) -> na::Affine3<f32> {
  let m = na::Matrix4::new_nonuniform_scaling(&vector(x, y, z));
  na::Affine3::from_matrix_unchecked(m)
}

#[inline]
pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> na::Affine3<f32> {
  na::Affine3::from_matrix_unchecked(na::Matrix4::new(
    1., xy, xz, 0., yx, 1., yz, 0., zx, zy, 1., 0., 0., 0., 0., 0.,
  ))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn multiplying_translation() {
    let transform = translation(5., -3., 2.);
    let p = point(-3., 4., 5.);
    assert_relative_eq!(transform * p, point(2., 1., 7.));
  }

  #[test]
  fn multiplying_inverse_translation() {
    let transform = translation(5., -3., 2.);
    let inv_transform = inverse(&transform);
    let p = point(-3., 4., 5.);
    assert_relative_eq!(inv_transform * p, point(-8., 7., 3.));
  }

  #[test]
  fn translation_does_not_affect_vectors() {
    let transform = translation(5., -3., 2.);
    let v = vector(-3., 4., 5.);
    assert_relative_eq!(
      transform.to_homogeneous() * v.to_homogeneous(),
      v.to_homogeneous()
    );
  }

  #[test]
  fn scaling_point() {
    let transform = scaling(2., 3., 4.);
    let p = point(-4., 6., 8.);
    assert_relative_eq!(transform * p, point(-8., 18., 32.));
  }

  #[test]
  fn scaling_vector() {
    let transform = scaling(2., 3., 4.);
    let v = vector(-4., 6., 8.);
    assert_relative_eq!(transform * v, vector(-8., 18., 32.));
  }

  #[test]
  fn scaling_inverse_vector() {
    let transform = inverse(&scaling(2., 3., 4.));
    let v = vector(-4., 6., 8.);
    assert_relative_eq!(transform * v, vector(-2., 2., 2.));
  }

  #[test]
  fn rotate_x() {
    let p = point(0., 1., 0.);
    let full_quarter = rotation_x(na::Real::frac_pi_2());
    assert_relative_eq!(full_quarter * p, point(0., 0., 1.));
  }

  #[test]
  fn rotate_y() {
    let p = point(0., 0., 1.);
    let full_quarter = rotation_y(na::Real::frac_pi_2());
    assert_relative_eq!(full_quarter * p, point(1., 0., 0.));
  }

  #[test]
  fn rotate_z() {
    let p = point(0., 1., 0.);
    let full_quarter = rotation_z(na::Real::frac_pi_2());
    assert_relative_eq!(full_quarter * p, point(-1., 0., 0.));
  }

  #[test]
  fn shearing_x_y() {
    let p = point(2., 3., 4.);
    let transform = shearing(1., 0., 0., 0., 0., 0.);
    assert_relative_eq!(transform * p, point(5., 3., 4.));
  }

  #[test]
  fn shearing_x_z() {
    let p = point(2., 3., 4.);
    let transform = shearing(0., 1., 0., 0., 0., 0.);
    assert_relative_eq!(transform * p, point(6., 3., 4.));
  }

  #[test]
  fn shearing_y_x() {
    let p = point(2., 3., 4.);
    let transform = shearing(0., 0., 1., 0., 0., 0.);
    assert_relative_eq!(transform * p, point(2., 5., 4.));
  }

  #[test]
  fn shearing_y_z() {
    let p = point(2., 3., 4.);
    let transform = shearing(0., 0., 0., 1., 0., 0.);
    assert_relative_eq!(transform * p, point(2., 7., 4.));
  }

  #[test]
  fn shearing_z_x() {
    let p = point(2., 3., 4.);
    let transform = shearing(0., 0., 0., 0., 1., 0.);
    assert_relative_eq!(transform * p, point(2., 3., 6.));
  }

  #[test]
  fn shearing_z_y() {
    let p = point(2., 3., 4.);
    let transform = shearing(0., 0., 0., 0., 0., 1.);
    assert_relative_eq!(transform * p, point(2., 3., 7.));
  }

  #[test]
  fn chain_transformations() {
    let p = point(1., 0., 1.);
    let a = rotation_x(na::Real::frac_pi_2());
    let b = scaling(5., 5., 5.);
    let c = translation(10., 5., 7.);
    let t = c * b * a;
    assert_relative_eq!(t * p, point(15., 0., 7.));
  }
}
