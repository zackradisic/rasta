use std::{
    io::{BufRead, Read},
    path::Path,
    str::FromStr,
};

use crate::{
    math::{Vec2, Vec3},
    object::Triangle,
    rasterize::Color,
};

/// Face can be of the following forms:
///
/// Only vertex indices:
/// f v1 v2 v3
///
/// Vertex indices and texture coordinate indices:
/// f v1/vt1 v2/vt2 v3/vt3
///
/// Vertex indices and texture coordinate indices and vertex normal indices:
/// f v1/vt1/vn1 v2/vt2/vn2 v3/vt3/vn3
#[derive(Debug, Clone)]
struct Face(Vec3<(usize, Option<usize>, Option<usize>)>);

impl Face {
    fn parse<'a, I: Iterator<Item = &'a str>>(words: I) -> Self {
        let mut face_vertices = vec![];
        for w in words {
            let vi: usize;
            let mut vti = None;
            let mut vni = None;
            match w.chars().filter(|c| *c == '/').count() {
                0 => {
                    vi = w.parse::<usize>().unwrap();
                }
                1 => {
                    let mut components = w.split("/").map(|w| w.parse::<usize>().unwrap());
                    vi = components.next().unwrap();
                    vti = Some(components.next().unwrap());
                }
                2 => {
                    let mut components = w.split("/").map(|w| w.parse::<usize>());
                    vi = components.next().unwrap().unwrap();
                    // For some reason faces like this are allowed: f 420//69
                    vti = components.next().unwrap().map(Some).unwrap_or(None);
                    vni = components.next().unwrap().map(Some).unwrap_or(None);
                }
                _ => panic!("Invalid word: {}", w),
            }
            face_vertices.push((vi, vti, vni))
        }

        Face(Vec3(
            face_vertices[0].into(),
            face_vertices[1].into(),
            face_vertices[2].into(),
        ))
    }
}

// TODO: Materials, normal, textures, etc.
#[derive(Debug, Clone)]
pub struct WavefrontObj {
    vertices: Vec<Vec3<f32>>,
    vertex_texture_indices: Vec<Vec2<f32>>,
    vertex_normal_indices: Vec<Vec3<f32>>,
    faces: Vec<Face>,
}

impl WavefrontObj {
    pub fn from_file<P: AsRef<Path>>(p: P, scale_coords: f32) -> Self {
        Self::from_reader(std::fs::File::open(p).unwrap(), scale_coords)
    }

    pub fn from_reader<R: Read>(r: R, scale_coords: f32) -> Self {
        let mut vertices = vec![];
        let mut vertex_texture_indices = vec![];
        let mut vertex_normal_indices = vec![];
        let mut faces = vec![];
        for l in std::io::BufReader::new(r).lines() {
            let line = l.unwrap();
            let mut words = line.split(" ").filter(|s| s.len() != 0);

            match words.nth(0) {
                Some("#") => continue,
                Some("v") => {
                    vertices.push(Self::parse_vec3(words) * scale_coords);
                    // println!("Last {:?}", vertices.last().as_ref().unwrap())
                }
                Some("vt") => {
                    vertex_texture_indices.push(Self::parse_vec2(words));
                }
                Some("vn") => {
                    vertex_normal_indices.push(Self::parse_vec3(words));
                }
                Some("f") => {
                    faces.push(Face::parse(words));
                }
                Some(_) => continue,
                None => continue,
            }
        }

        Self {
            vertices,
            vertex_texture_indices,
            vertex_normal_indices,
            faces,
        }
    }

    fn parse_vec3<'a, T, I>(words: I) -> Vec3<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
        I: Iterator<Item = &'a str>,
    {
        let mut parsed = words.map(|word| word.parse::<T>().unwrap());
        Vec3(
            parsed.next().unwrap(),
            parsed.next().unwrap(),
            parsed.next().unwrap(),
        )
    }

    fn parse_vec2<'a, T, I>(words: I) -> Vec2<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
        I: Iterator<Item = &'a str>,
    {
        let mut parsed = words.map(|word| word.parse::<T>().unwrap());
        Vec2(parsed.next().unwrap(), parsed.next().unwrap())
    }
}

impl WavefrontObj {
    pub fn make_triangles(
        &self,
        color: Option<Color>,
        textured: bool,
        normals: bool,
        outlines: bool,
    ) -> Vec<Triangle> {
        let mut triangles = vec![];

        for Face(indices) in &self.faces {
            triangles.push(Triangle {
                p0: self.vertices[indices.0 .0 - 1].clone(),
                p1: self.vertices[if !outlines {
                    indices.1 .0 - 1
                } else {
                    indices.0 .0 - 1
                }]
                .clone(),
                p2: self.vertices[indices.2 .0 - 1].clone(),
                color: color.unwrap_or_default(),
                normals: if normals {
                    Some(Box::new([
                        self.vertex_normal_indices[indices.0 .2.unwrap() - 1].clone(),
                        self.vertex_normal_indices[indices.1 .2.unwrap() - 1].clone(),
                        self.vertex_normal_indices[indices.2 .2.unwrap() - 1].clone(),
                    ]))
                } else {
                    None
                },
                uvs: if textured {
                    Some(Box::new([
                        self.vertex_texture_indices[indices.1 .1.unwrap() - 1].clone(),
                        self.vertex_texture_indices[indices.2 .1.unwrap() - 1].clone(),
                        self.vertex_texture_indices[indices.0 .1.unwrap() - 1].clone(),
                    ]))
                } else {
                    None
                },
            })
        }

        triangles
    }
}
