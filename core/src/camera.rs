use crate::*;
use std::sync::Arc;
use std::thread;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    h_size: usize,
    v_size: usize,
    transform_inverse: Transform,
    half_width: f32,
    half_height: f32,
    pixel_size: f32,
    max_reflects: u8,
}

impl Camera {
    pub fn new(h_size: usize, v_size: usize, field_of_view: f32) -> Camera {
        let half_width;
        let half_height;
        let half_view = f32::tan(field_of_view / 2.);
        let aspect = (h_size as f32) / (v_size as f32);
        if aspect >= 1. {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = half_width * 2.0 / (h_size as f32);

        Camera {
            h_size,
            v_size,
            transform_inverse: Transform::identity(),
            half_width,
            half_height,
            pixel_size,
            max_reflects: 5,
        }
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform_inverse = transform.inverse();
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x_offset = (x as f32 + 0.5) * self.pixel_size;
        let y_offset = (y as f32 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let transform_inv = &self.transform_inverse;
        let pixel = transform_inv * point(world_x, world_y, -1.);
        let origin = transform_inv * point(0., 0., 0.);
        let direction = normalize(&(pixel - origin));
        Ray::new(origin, direction.into_inner())
    }

    pub fn render(self, world: World) -> Canvas {
        let canvas = Canvas::new(self.h_size, self.v_size);
        let canvas = Arc::new(canvas);
        let world = Arc::new(world);
        let camera = Arc::new(self);

        let mut handles = vec![];
        let n_threads = num_cpus::get();
        for i in 0..n_threads {
            let shared_canvas = Arc::clone(&canvas);
            let world = Arc::clone(&world);
            let camera = Arc::clone(&camera);
            let handle = thread::spawn(move || {
                for y in (i..=camera.v_size - n_threads + i).step_by(n_threads) {
                    for x in 0..camera.h_size {
                        let ray = camera.ray_for_pixel(x, y);
                        let color = world.color_at(&ray, camera.max_reflects).into();
                        shared_canvas.set(x, y, color);
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        Arc::try_unwrap(canvas).unwrap()
    }
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
        assert_relative_eq!(t, scaling(-1., 1., -1.));
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = point(0., 0., 8.);
        let to = point(0., 0., 1.);
        let up = vector(0., 1., 0.);
        let t = view_transform(from, to, up);
        assert_relative_eq!(t, translation(0., 0., -8.));
    }

    #[test]
    fn construct_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = std::f32::consts::FRAC_PI_2;
        let camera = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(camera.v_size, vsize);
        assert_eq!(camera.h_size, hsize);
        assert_relative_eq!(camera.transform_inverse, Transform::identity());
    }

    #[test]
    fn pixel_size_horizontal() {
        let c = Camera::new(200, 125, std::f32::consts::FRAC_PI_2);
        assert_relative_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn pixel_size_vertical() {
        let c = Camera::new(125, 200, std::f32::consts::FRAC_PI_2);
        assert_relative_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn construct_ray_center() {
        let c = Camera::new(201, 101, std::f32::consts::FRAC_PI_2);
        let r: Ray = c.ray_for_pixel(100, 50);
        assert_relative_eq!(r.origin, point(0., 0., 0.));
        assert_relative_eq!(r.direction, vector(0., 0., -1.));
    }

    #[test]
    fn construct_ray_corner() {
        let c = Camera::new(201, 101, std::f32::consts::FRAC_PI_2);
        let r: Ray = c.ray_for_pixel(0, 0);
        assert_relative_eq!(r.origin, point(0., 0., 0.));
        assert_relative_eq!(r.direction, vector(0.6651864, 0.33259323, -0.66851234));
    }

    #[test]
    fn construct_ray_transformed() {
        let mut c = Camera::new(201, 101, std::f32::consts::FRAC_PI_2);
        c.set_transform(rotation_y(std::f32::consts::FRAC_PI_4) * translation(0., -2., 5.));
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
        let mut camera = Camera::new(11, 11, std::f32::consts::FRAC_PI_2);
        let from = point(0., 0., -5.);
        let to = point(0., 0., 0.);
        let up = vector(0., 1., 0.);
        camera.set_transform(view_transform(from, to, up));
        let canvas = camera.render(world);
        assert_eq!(canvas.get(5, 5), color(0.38066, 0.47583, 0.2855).into());
    }
}
