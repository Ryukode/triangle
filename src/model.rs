use crate::mesh::Mesh;
use crate::buffers;
use std::{fmt, fs};

pub struct Model {
    mesh: Mesh,
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
    pub fn new() -> Self {
        Self {
            mesh: Mesh::new(),
        }
    }

    pub fn load_obj(&mut self, filepath: &str) -> &mut Model{
        let data: String = fs::read_to_string(filepath).unwrap();

        for line in data.lines() {
            if line.is_empty() {continue;}
            let (pattern, values) = line.split_once(' ').unwrap();
            println!("Pattern: {}, Values: {}", pattern, values);
            match pattern {
                "v" => {
                    let coords: Vec<&str> = values.split(' ').collect();
            
                    let mut vertex: buffers::Vertex = buffers::Vertex::new();
                    vertex.set_position([coords[0].parse::<f32>().unwrap(), coords[1].parse::<f32>().unwrap()]);
                    vertex.set_color([1.0, 0.0, 0.0, 1.0]);
                    self.mesh.vb().add_vertex(vertex);
                }
                "f" => {
                    for index in values.split(' ') {
                        self.mesh.ib().add_index(index.parse::<u32>().unwrap() - 1);
                    }
                    
                }
                _ => {}
            }
        }
        self
    }

    pub fn get_indices(&mut self) -> &Vec<u32> {
        &self.mesh.ib().get_indices()
    }

    pub fn get_vertices(&mut self) -> &Vec<buffers::Vertex> {
        &self.mesh.vb().get_vertices()
    }

    pub fn update(&mut self) {
        self.mesh.vb().update();
    }
}
