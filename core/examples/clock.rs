extern crate rustracer_core;

use rustracer_core::*;

use nalgebra as na;
use std::fs::File;
use std::io::prelude::*;

fn clock() -> Vec<Point> {
    let mut points = Vec::with_capacity(12);
    let p = point(200., 0., 0.);
    let angle: f32 = na::Real::frac_pi_6();

    for i in 0..12 {
        points.push(rotation_y(angle * i as f32) * p)
    }
    return points;
}

fn main() {
    let points = clock();

    let c = canvas(500, 500);
    for p in points {
        c.set(
            (250. + p.x) as usize,
            (250. + p.z) as usize,
            color(1., 1., 1.).into(),
        );
    }
    let mut file = File::create("clock.ppm").expect("Couldn't create file");
    file.write_all(c.to_ppm_string().as_bytes())
        .expect("Couldn't write canvas");
}
