use crate::*;

pub fn read_obj_file(group: &mut Group, file: &str) {
    let bytes = std::fs::read(file).unwrap_or_else(|_| panic!("'File {}' not found", file));
    read_obj_from_bytes(group, &bytes);
}

pub fn read_obj_from_bytes(group: &mut Group, bytes: &[u8]) {
    let obj_str = String::from_utf8(bytes.to_vec())
        .unwrap_or_else(|_| panic!("Failed to convert byte array to string"));
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
