use rand::random;
use std::{fmt};

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
    color: [f32; 4],
}

impl fmt::Display for Vertex{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vertex: x: {}, y: {}, z: {}, r: {}, g: {}, b: {}, a: {}", self.pos[0], self.pos[1], self.pos[2], self.color[0], self.color[1], self.color[2], self.color[3])
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], normal: [f32; 3], uv: [f32; 2], color: [f32; 4]) -> Self {
        Self {
            pos, normal, uv, color
        }
    }

    pub fn as_vec(&self) -> Vec<f32> {
        let a = [self.pos.to_vec(),self.normal.to_vec(), self.uv.to_vec(), self.color.to_vec()].concat();
        a
    }
}

pub struct VertexBuffer {
    vertices: Vec<Vertex>
}

impl fmt::Display for VertexBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "VertexBuffer [")?;
        for vertex in &self.vertices {
            writeln!(f, "{}", vertex)?;
        }
        write!(f, "]")
    }
}

impl VertexBuffer {
    pub fn new() -> Self{
        Self{
            vertices: vec![],              
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        let mut vec = vec![vertex];
        self.vertices.append(&mut vec);
    }

    pub fn get_vertices(&self) -> Vec<f32> {
        let mut verts = vec![];
        for v in &self.vertices[..] {
            verts = [verts, v.as_vec()].concat()
        }
        verts
    }

    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x3,
            2 => Float32x2,
            3 => Float32x4,
        ],
    };
}

pub struct IndexBuffer{
    indices: Vec<u32>,
}

impl fmt::Display for IndexBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "IndexBuffer [")?;
        for index in &self.indices {
            writeln!(f, "{}", index)?;
        }
        write!(f, "]")
    }
}

impl IndexBuffer {
    pub fn new() -> Self {
        Self {
            indices: vec![],
        }
    }

    pub fn add_index(&mut self, index: u32) {
        self.indices.append(&mut vec![index]);
    }

    pub fn get_indices(&self) -> Vec<u32> {
       self.indices.clone()
    }
}