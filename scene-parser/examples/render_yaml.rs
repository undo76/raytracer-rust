extern crate rustracer_parser;
extern crate serde_yaml;

use rustracer_parser::*;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("Usage:  render_yaml <scene.yaml> <output.ppm>");
        std::process::exit(1);
    }
    let input_file = args.get(1).expect("Input file name required");
    let output_file = args.get(2).expect("Output file name required");
    let file_contents =
        std::fs::read(input_file).unwrap_or_else(|_| panic!("Couldn't open {}", &input_file));
    let yaml_str = String::from_utf8(file_contents).expect("Couldn't read contents");

    match parse_yaml::<Scene>(&yaml_str) {
        Ok(scene) => {
            let (world, camera) = build_scene(&scene);
            let canvas = camera.render(world);
            std::fs::write(output_file, canvas.to_ppm_string()).unwrap();
        }
        Err(err) => {
            eprintln!("{}", &err);
            std::process::exit(2);
        }
    };
}
