use crate::*;
use nom::*;
use nom::types::CompleteStr;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct VertexCommand(Point);

#[derive(Debug, PartialEq)]
struct FaceCommand(Face);

#[derive(Debug, PartialEq)]
struct GroupCommand(GroupName);

#[derive(Debug, PartialEq)]
pub struct FaceVertex {
    pub idx: usize,
}

type GroupName = String;
pub type Face = Vec<FaceVertex>;

#[derive(Debug, PartialEq)]
enum Command {
    Vertex(VertexCommand),
    Face(FaceCommand),
    Group(GroupName),
    Other(String),
}

pub struct Object {
    pub vertices: Vec<Point>,
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

named!(parse_u32<CompleteStr, usize>,
    map_res!(recognize!(nom::digit), from_u32)
);

fn from_u32(input: CompleteStr) -> Result<usize, std::num::ParseIntError> {
  usize::from_str(&input)
}

named!(face<CompleteStr, Command>, 
    do_parse!(
        tag!("f") >> 
        space >>
        vertex_idxs: separated_nonempty_list!(space, parse_u32) >>
        (Command::Face(FaceCommand(vertex_idxs.iter().map(|&idx| FaceVertex { idx }).collect())))
    )
);

named!(parse_line<CompleteStr, Command>,
    alt!(
        vertex | group | face | other
    )
);


pub fn parse(input: &str) -> Object {
    let mut obj = Object {
            vertices: vec![],
            faces: vec![]
        };
    input
        .lines()
        .filter_map(|line| parse_line(CompleteStr(line)).ok())
        .for_each(|(_i, c)| {
            match c {
                Command::Vertex(VertexCommand(p)) => obj.vertices.push(p),
                Command::Face(FaceCommand(f)) => obj.faces.push(f),
                _ => ()
                // Command::Group(name) => obj.vertices.push(p)
                // Command::Vertex(VertexCommand(p)) => obj.vertices.push(p)
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
            FaceVertex{ idx: 1 },
            FaceVertex{ idx: 2 },
            FaceVertex{ idx: 3 },
            FaceVertex{ idx: 4 },
            FaceVertex{ idx: 55 }])))))
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
