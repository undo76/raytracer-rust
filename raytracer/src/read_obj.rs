use crate::*;
use std::fs::File;
use std::io::Read;

pub fn read_obj_file(group: &mut Group, file: &str) {
    let mut obj_file = File::open(file).expect("teapot.obj no file");
    let mut obj_str = String::new();
    let _res = obj_file.read_to_string(&mut obj_str).unwrap();
    let obj = parse(&obj_str);

    for f in &obj.faces {
        Triangle::add_to_group(
            group,
            &f.iter()
                .map(|v| {
                    (
                        obj.vertices[v.idx - 1],
                        v.normal_idx.map(|n_idx| obj.normals[n_idx - 1]),
                    )
                })
                .collect::<Vec<_>>(),
        );
    }
}
