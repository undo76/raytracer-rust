extern crate raytracer_rust;

use raytracer_rust::*;

use rand::distributions::Uniform;
use rand::*;
use std::f32::consts::*;
use std::fs::File;
use std::io::prelude::Write;

const MAX_SPHERES: usize = 2000;
const RADIUS: f32 = 12.0;

#[derive(Debug)]
struct SphereData {
    center: Point,
    radius: f32,
}

fn glass(color: ColorRgbFloat) -> Material {
    Material {
        color: color.into(),
        diffuse: 0.1.into(),
        ambient: 0.0.into(),
        specular: 0.5.into(),
        shininess: 100.0.into(),
        reflective: Some(0.9.into()),
        transparency: Some(0.9.into()),
        refractive_index: 1.5.into(),
        ..Material::default()
    }
}

fn metal(color: ColorRgbFloat) -> Material {
    Material {
        color: color.into(),
        diffuse: 0.6.into(),
        ambient: 0.1.into(),
        specular: 0.4.into(),
        shininess: 7.0.into(),
        reflective: Some(0.1.into()),
        ..Material::default()
    }
}

fn random_color() -> ColorRgbFloat {
    let mut rng = thread_rng();
    let c: Vec<f32> = rng.sample_iter(&Uniform::from(0.5..1.0)).take(3).collect();
    color(c[0], c[1], c[2])
}

fn random_material() -> Material {
    let c = random_color();
    let mut rng = thread_rng();
    match rng.next_u32() % 10 {
        0 => glass(c),
        _ => metal(c),
    }
}

fn spheres() -> Vec<Box<dyn Shape + Send>> {
    let mut rng = thread_rng();

    let mut spheres = vec![SphereData {
        center: point(0.0, RADIUS, 0.0),
        radius: 1.0,
    }];

    let mut attempts = 0;
    while spheres.len() < MAX_SPHERES {
        let (r_min, r_max): (f32, f32) = match attempts {
            0...999 => (0.5, 1.5),
            1_000...2_999 => (0.5 * 0.5, 1.5 * 0.5),
            3_000...4_999 => (0.5 * 0.25, 1.5 * 0.25),
            5_000...10_000 => (0.5 * 0.125, 1.5 * 0.125),
            _ => break,
        };

        let theta = rng.gen_range(0.0, PI);
        let phi = rng.gen_range(0.0, PI * 2.);
        let r = r_min + (rng.gen_range(0.0, 1.0) * (r_max - r_min));

        let x = RADIUS * theta.sin() * phi.cos();
        let y = RADIUS * theta.cos();
        let z = RADIUS * theta.sin() * phi.sin();

        let sphere = SphereData {
            center: point(x, y, z),
            radius: r,
        };

        let mut ok = true;
        for s in &spheres {
            let dp = sphere.center - s.center;
            let r2 = sphere.radius + s.radius;

            ok = dot(&dp, &dp) > r2 * r2;
            if !ok {
                break;
            }
        }

        if ok {
            println!("{:?}", &sphere);
            spheres.push(sphere);
            println!("{} {}", spheres.len(), attempts);
            attempts = 0;
        } else {
            attempts += 1;
        }
    }

    spheres
        .iter()
        .map(
            |&SphereData {
                 center: c,
                 radius: r,
             }| {
                let boxed: Box<dyn Shape + Send> = Box::new(Sphere::new(
                    translation(c.x, c.y, c.z) * scaling(r, r, r),
                    random_material(),
                ));

                boxed
            },
        )
        .collect()
}

fn lights() -> Vec<PointLight> {
    vec![
        PointLight::new(point(-100., 100., -100.), color(1.0, 1.0, 1.0)),
        PointLight::new(point(150., 30., -50.), color(0.2, 0.2, 0.2)),
        PointLight::new(point(0.0, 0.0, 0.0), color(0.5, 0.5, 0.5)),
    ]
}

fn main() {
    let world = World::new(spheres(), lights());
    let mut camera = Camera::new(3000, 3000, FRAC_PI_6);
    camera.set_transform(view_transform(
        point(50., 15., -50.),
        point(0.0, 0.0, 0.0),
        vector(0., 1., 0.),
    ));
    let canvas = camera.render(world);

    let mut file = File::create("spheres.ppm").expect("Couldn't create file");
    file.write_all(canvas.to_ppm_string().as_bytes())
        .expect("Couldn't write canvas");
}
