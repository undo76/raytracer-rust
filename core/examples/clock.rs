extern crate rustracer_core;

use rustracer_core::*;

fn clock() -> Vec<Point> {
    let mut points = Vec::with_capacity(12);
    let p = point(200., 0., 0.);
    let angle: f32 = std::f32::consts::FRAC_PI_6;

    for i in 0..12 {
        points.push(rotation_y(angle * i as f32) * p)
    }
    points
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
    c.save("./output/clock.png")
}
