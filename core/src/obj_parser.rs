/// Parser for a small subset of the Wavefront .OBJ format
/// 
/// http://www.martinreddy.net/gfx/3d/OBJ.spec 

use crate::*;
use nom::*;
use nom::types::CompleteStr;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct VertexCommand(Point);

#[derive(Debug, PartialEq)]
struct VertexNormalCommand(UnitVector);

#[derive(Debug, PartialEq)]
struct FaceCommand(Face);

#[derive(Debug, PartialEq)]
struct GroupCommand(GroupName);

#[derive(Debug, PartialEq)]
pub struct FaceVertex {
    pub idx: usize,
    pub texture_idx: Option<usize>,
    pub normal_idx: Option<usize>,
}

type GroupName = String;
pub type Face = Vec<FaceVertex>;

#[derive(Debug, PartialEq)]
enum Command {
    Vertex(VertexCommand),
    VertexNormal(VertexNormalCommand),
    Face(FaceCommand),
    Group(GroupName),
    Other(String),
}

pub struct Object {
    pub vertices: Vec<Point>,
    pub normals: Vec<UnitVector>,
    pub faces: Vec<Face>,
}

named!(vertex<CompleteStr, Command>, 
    do_parse!(
        tag!("v") >> 
        space >>
        x: float >>
        space >>
        y: float >>
        space >>
        z: float >>
        (Command::Vertex(VertexCommand(point(x, y, z))))
    )
);

named!(vertex_normal<CompleteStr, Command>, 
    do_parse!(
        tag!("vn") >> 
        space >>
        x: float >>
        space >>
        y: float >>
        space >>
        z: float >>
        (Command::VertexNormal(VertexNormalCommand(normalize(&vector(x, y, z)))))
    )
);

named!(other<CompleteStr, Command>, 
    map!(rest, |s| Command::Other(s.to_string()))
);

named!(group<CompleteStr, Command>, 
    do_parse!(
        tag!("g") >> 
        space >>
        groupName: alpha >>
        (Command::Group(groupName.to_string()))
    )
);

named!(parse_usize<CompleteStr, usize>,
    map_res!(recognize!(nom::digit), from_usize)
);

fn from_usize(input: CompleteStr) -> Result<usize, std::num::ParseIntError> {
  usize::from_str(&input)
}

named!(parse_face_vertex_1<CompleteStr, FaceVertex>,
    do_parse!(
        idx: parse_usize >>
        (FaceVertex { 
            idx,
            texture_idx: None,
            normal_idx: None
        })
    )
);

named!(parse_face_vertex_2<CompleteStr, FaceVertex>,
    do_parse!(
        idx: parse_usize >>
        tag!("/") >>
        texture_idx: opt!(parse_usize) >>
        tag!("/") >>
        normal_idx: opt!(parse_usize) >>
        (FaceVertex { 
            idx,
            texture_idx: texture_idx,
            normal_idx: normal_idx
        })
    )
);

named!(parse_face_vertex<CompleteStr, FaceVertex>,
    alt!( parse_face_vertex_2 | parse_face_vertex_1 )
);

named!(face<CompleteStr, Command>, 
    do_parse!(
        tag!("f") >> 
        space >>
        face_vertices: separated_nonempty_list!(space, parse_face_vertex) >>
        (Command::Face(FaceCommand(face_vertices)))
    )
);

named!(parse_line<CompleteStr, Command>,
    alt!(
        vertex | vertex_normal | group | face | other
    )
);


pub fn parse(input: &str) -> Object {
    let mut obj = Object {
        vertices: vec![],
        normals: vec![],
        faces: vec![]
    };
    input
        .lines()
        .filter_map(|line| parse_line(CompleteStr(line)).ok())
        .for_each(|(_i, c)| {
            match c {
                Command::Vertex(VertexCommand(p)) => obj.vertices.push(p),
                Command::VertexNormal(VertexNormalCommand(n)) => obj.normals.push(n),
                Command::Face(FaceCommand(f)) => obj.faces.push(f),
                _ => ()
            }
        });
    obj
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_vertex() {
        let v = vertex(CompleteStr("v 1.0 2.0 3.0"));
        assert_eq!(v, Ok((CompleteStr(""), Command::Vertex(VertexCommand(point(1.0, 2.0, 3.0))))))
    }

    #[test]
    fn parse_vertex_normal() {
        let v = vertex_normal(CompleteStr("vn 1.0 2.0 3.0"));
        assert_eq!(v, Ok((CompleteStr(""), Command::VertexNormal(VertexNormalCommand(normalize(&vector(1.0, 2.0, 3.0)))))))
    }

    #[test]
    fn parse_group() {
        let v = group(CompleteStr("g GroupName"));
        assert_eq!(v, Ok((CompleteStr(""), Command::Group("GroupName".to_string()))))
    }

    #[test]
    fn parse_other() {
        let v = other(CompleteStr("this is a string"));
        assert_eq!(v, Ok((CompleteStr(""), Command::Other("this is a string".to_string()))))
    }

    #[test]
    fn parse_face() {
        let v = face(CompleteStr("f 1 2 3 4 55"));
        assert_eq!(v, Ok((CompleteStr(""), Command::Face(FaceCommand(vec![
            FaceVertex{ idx: 1, texture_idx: None, normal_idx: None },
            FaceVertex{ idx: 2, texture_idx: None, normal_idx: None },
            FaceVertex{ idx: 3, texture_idx: None, normal_idx: None },
            FaceVertex{ idx: 4, texture_idx: None, normal_idx: None },
            FaceVertex{ idx: 55, texture_idx: None, normal_idx: None }])))))
    }

//     #[test]
//     fn parse_obj() {
//         let input = r#"
// v 1 0 0
// v 1 1 0

// g FirstGroup
// f 1 2 3
// g SecondGroup
// f 1 3 4
//         "#;
//         println!("{:?}", parse(&input));
//     }
}
