extern crate rustracer_parser;
extern crate serde_yaml;

use rustracer_parser::*;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!(
            "
Usage:  parse_yaml <file_name>    
            "
        );
        std::process::exit(1);
    }
    let file_name = args.get(1).expect("File name required");
    let file_contents =
        std::fs::read(file_name).unwrap_or_else(|_| panic!("Couldn't open {}", &file_name));
    let yaml_str = String::from_utf8(file_contents).expect("Couldn't read contents");

    match parse_yaml::<Scene>(&yaml_str) {
        Ok(scene) => println!("{:#?}", scene),
        Err(err) => {
            eprintln!("{}", &err);
            std::process::exit(2);
        }
    };
}
