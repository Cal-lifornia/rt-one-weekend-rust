use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Obj {
    pub verts: Box<Vec<Point3>>,
    pub normals: Box<Vec<Point3>>,
    pub faces: Vec<Face>,
}

#[derive(Debug)]
pub struct Face {
    pub verts: [usize; 3],
    pub text_coords: [usize; 3],
    pub normals: [usize; 3],
}

impl Obj {
    pub fn from<P>(filename: P) -> Option<Obj>
    where
        P: AsRef<Path>,
    {
        if let Ok(lines) = read_lines(filename) {
            // Obj files indices start at 1, so create a dummy entry to avoid instantiating later
            let mut points: Vec<Point3> = vec![];
            let mut normals: Vec<Point3> = vec![];

            let mut faces: Vec<Face> = vec![];

            for line in lines.map_while(anyhow::Result::ok) {
                match line {
                    s if s.starts_with("v ") => points.push(string_to_point(s[2..].to_string())),
                    s if s.starts_with("f ") => faces.push(string_to_face(s[2..].to_string())),
                    s if s.starts_with("vn ") => normals.push(string_to_point(s[3..].to_string())),

                    _ => {}
                }
            }
            Some(Obj {
                verts: Box::new(points),
                normals: Box::new(normals),
                faces,
            })
        } else {
            None
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn string_to_point(line: String) -> Point3 {
    let splits: Vec<f64> = line
        .split_ascii_whitespace()
        .take(3)
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    Point3::new(splits[0], splits[1], splits[2])
}

fn string_to_face(line: String) -> Face {
    // let splits: Vec<usize> = line
    //     .split_ascii_whitespace()
    //     .take(3)
    //     .map_while(|x| {
    //         let i = x.split("/").next().unwrap();
    //         i.parse::<usize>().ok()
    //     })
    //     .collect();

    let mut verts: [usize; 3] = Default::default();
    let mut text_coords: [usize; 3] = Default::default();
    let mut normals: [usize; 3] = Default::default();

    line.split_ascii_whitespace()
        .enumerate()
        .for_each(|(i, val)| {
            let splits: Vec<&str> = val.split("/").collect();
            verts[i] = splits[0].parse().unwrap_or_default();
            text_coords[i] = splits[1].parse().unwrap_or_default();
            normals[i] = splits[2].parse().unwrap_or_default();
        });

    Face {
        verts,
        text_coords,
        normals,
    }
}
