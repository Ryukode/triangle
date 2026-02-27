use crate::transform::Transform;
use crate::mesh::Mesh;
use crate::buffers;
use std::{fmt, fs};
use crate::buffers::{IndexBuffer, Vertex, VertexBuffer};
use crate::camera::Camera;
use crate::color::Color;
use crate::shader::{AnyShader, BaseShader, PhongShader};
use crate::vector::Vector3;

pub struct Model {
    mesh: Mesh,
    pub transform: Transform,
    shader: AnyShader,
    //texture
    //AABB
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Model {{")?;
        writeln!(f, "{}", self.mesh)?;
        writeln!(f, "}}")
    }
}

impl Model {
    pub fn draw(&self, cam: &Camera) {
        // bind shader + textures
        self.shader.activate();
        // bind buffers

        // draw call
    }
    pub fn load_obj(&mut self, filepath: &str) -> &mut Model{
        let data: String = fs::read_to_string(filepath).unwrap();
        let mut vb = VertexBuffer::new();
        let mut ib = IndexBuffer::new();
        let mut count: u32 = 0;

        let mut positions: Vec<[f32;3]> = Vec::new();
        let mut normals: Vec<[f32;3]> = Vec::new();
        let mut uvs: Vec<[f32;2]> = Vec::new();

        for line in data.lines() {
            if line.is_empty() {continue;}
            let (pattern, values) = line.split_once(' ').unwrap();
            println!("Pattern: {}, Values: {}", pattern, values);
            match pattern {
                "v" => {
                    let values: Vec<&str> = values.split(' ').collect();

                    let pos= [Self::parse_f32(values[0]), Self::parse_f32(values[1]), Self::parse_f32(values[2])];

                    positions.push(pos);
                }
                "vn" => {
                    let values: Vec<&str> = values.split(' ').collect();

                    let normal= [Self::parse_f32(values[0]), Self::parse_f32(values[1]), Self::parse_f32(values[2])];

                    normals.push(normal);
                }
                "vt" => {
                    let values: Vec<&str> = values.split(' ').collect();

                    let uv = [Self::parse_f32(values[0]), Self::parse_f32(values[1])];

                    uvs.push(uv);
                }
                "f" => {
                    let indices = values.split(' ');
                    if indices.clone().count() != 3 {
                        eprintln!("{}", "Faces are not triangulated, render will probably suck")
                    }
                    for index in indices {
                        let index: Vec<&str> = index.split('/').collect();
                        let red = [1., 0., 0., 1.];
                        let vertex = Vertex::new(
                            positions[(Self::parse_u32(index[0]) - 1) as usize],
                            normals[(Self::parse_u32(index[2]) - 1) as usize],
                            uvs[(Self::parse_u32(index[1]) - 1) as usize],
                            red);

                        vb.add_vertex(vertex);
                        //ib.add_index(Self::parse_u32(index[0]) - 1);
                        ib.add_index(count);
                        count+=1
                    }
                }
                _ => {}
            }
        }

        self.mesh = Mesh::new(vb, ib);

        self
    }

    fn parse_f32(value: &str) -> f32 {
        value.parse::<f32>().unwrap()
    }
    fn parse_u32(value: &str) -> u32 {
        value.parse::<u32>().unwrap()
    }

    pub fn get_indices(&self) -> Vec<u32> {
        self.mesh.ib.get_indices()
    }

    pub fn get_vertices(&self) -> Vec<f32> {
        self.mesh.vb.get_vertices()
    }

    pub fn update(&mut self) {

    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            mesh: Mesh::default(),
            transform: Transform::default(),
            shader: AnyShader::PhongShader(PhongShader::default())
        }
    }
}
