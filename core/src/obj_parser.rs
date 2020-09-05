use std::str::FromStr;

use nom::{character, IResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, space1};
use nom::combinator::{all_consuming, map_res, opt};
use nom::multi::separated_list1;
use nom::number::complete::float;
use nom::sequence::{preceded, tuple};

/// Parser for a small subset of the Wavefront .OBJ format
///
/// http://www.martinreddy.net/gfx/3d/OBJ.spec

use crate::*;

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

fn vertex(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("v")(input)?;
    let (input, _) = space1(input)?;
    let (input, (x, y, z)) = tuple((float, preceded(space1, float), preceded(space1, float)))(input)?;
    Ok((input, Command::Vertex(VertexCommand(point(x, y, z)))))
}

fn vertex_normal(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("vn")(input)?;
    let (input, _) = space1(input)?;
    let (input, (x, y, z)) = tuple((float, preceded(space1, float), preceded(space1, float)))(input)?;
    Ok((input, Command::VertexNormal(VertexNormalCommand(normalize(&vector(x, y, z))))))
}

fn other(input: &str) -> IResult<&str, Command> {
    let (input, s) = character::complete::not_line_ending(input)?;
    Ok((input, Command::Other(s.to_string())))
}

fn group(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("g")(input)?;
    let (input, _) = space1(input)?;
    let (input, group_name) = alpha1(input)?;
    Ok((input, Command::Group(group_name.to_string())))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, from_usize)(input)
}

fn from_usize(input: &str) -> Result<usize, std::num::ParseIntError> {
    usize::from_str(input)
}

fn parse_face_vertex_1(input: &str) -> IResult<&str, FaceVertex> {
    let (input, idx) = parse_usize(input)?;
    Ok((input, FaceVertex { idx, texture_idx: None, normal_idx: None }))
}

fn parse_face_vertex_2(input: &str) -> IResult<&str, FaceVertex> {
    let (input, (idx, texture_idx, normal_idx)) = tuple((
        parse_usize,
        preceded(character::complete::char('/'), opt(parse_usize)),
        preceded(character::complete::char('/'), opt(parse_usize)),
    ))(input)?;
    Ok((input, FaceVertex { idx, texture_idx, normal_idx }))
}

fn parse_face_vertex(input: &str) -> IResult<&str, FaceVertex> {
    alt((parse_face_vertex_2, parse_face_vertex_1))(input)
}

fn face(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("f")(input)?;
    let (input, _) = space1(input)?;
    let (input, face_vertices) = separated_list1(space1, parse_face_vertex)(input)?;
    Ok((input, Command::Face(FaceCommand(face_vertices))))
}


/// Parse a single line of an OBJ file
fn parse_line(input: &str) -> IResult<&str, Command> {
    let (_, cmd) = all_consuming(alt((
        vertex,
        vertex_normal,
        group,
        face,
        other
    )))(input)?;
    Ok((input, cmd))
}

// Main parse function
pub fn parse(input: &str) -> Object {
    let mut obj = Object {
        vertices: vec![],
        normals: vec![],
        faces: vec![],
    };
    input
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .for_each(|c| {
            let (_text, cmd) = c;
            match cmd {
                Command::Vertex(VertexCommand(p)) => obj.vertices.push(p),
                Command::VertexNormal(VertexNormalCommand(n)) => obj.normals.push(n),
                Command::Face(FaceCommand(f)) => obj.faces.push(f),
                Command::Other(_texts) => (),
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
        let v = vertex("v 1.0 2.0 3.0");
        assert_eq!(v, Ok(("", Command::Vertex(VertexCommand(point(1.0, 2.0, 3.0))))));
    }

    #[test]
    fn parse_vertex_normal() {
        let v = vertex_normal("vn 1.0 2.0 3.0");
        assert_eq!(v, Ok(("", Command::VertexNormal(VertexNormalCommand(normalize(&vector(1.0, 2.0, 3.0)))))));
    }

    #[test]
    fn parse_group() {
        let v = group("g GroupName");
        assert_eq!(v, Ok(("", Command::Group("GroupName".to_string()))));
    }

    #[test]
    fn parse_other() {
        let v = other("this is a string");
        assert_eq!(v, Ok(("", Command::Other("this is a string".to_string()))));
    }

    #[test]
    fn parse_face() {
        let v = face("f 1 2 3 4 55");
        assert_eq!(v, Ok(("", Command::Face(FaceCommand(vec![
            FaceVertex { idx: 1, texture_idx: None, normal_idx: None },
            FaceVertex { idx: 2, texture_idx: None, normal_idx: None },
            FaceVertex { idx: 3, texture_idx: None, normal_idx: None },
            FaceVertex { idx: 4, texture_idx: None, normal_idx: None },
            FaceVertex { idx: 55, texture_idx: None, normal_idx: None }])))));
    }
}
