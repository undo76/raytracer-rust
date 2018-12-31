extern crate rustracer_parser;
extern crate serde_json;

use rustracer_parser::*;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!(
            "
Usage:  parse_json <file_name>    
            "
        );
        std::process::exit(1);
    }
    let file_name = args.get(1).expect("File name required");
    let scene = std::fs::read(file_name).expect(&format!("Couldn't open {}", &file_name));
    let scene = String::from_utf8(scene).expect("Couldn't read contents");
    match serde_json::from_str::<Scene>(&scene) {
        Ok(scene) => println!("{:#?}", scene),
        Err(err) => {
            eprintln!("{}", &err);
            std::process::exit(2);
        }
    };
}
