use rand::random;
use std::{fmt};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
}

impl fmt::Display for Vertex{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vertex: x: {}, y: {}, r: {}, g: {}, b: {}, a: {}", self.pos[0], self.pos[1], self.color[0], self.color[1], self.color[2], self.color[3])
    }
}

impl Vertex {
    pub fn new() -> Self {
        Self{
            pos: [0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn set_position(&mut self, pos: [f32; 2]) {
        self.pos = pos;
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    pub fn _get_position(&mut self) -> [f32; 2]{
        self.pos
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

    pub fn get_vertices(&mut self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn update(&mut self){
        for vertex in &mut self.vertices {
            vertex.set_position([
                random::<f32>() * 2.0 - 1.0, 
                random::<f32>() * 2.0 - 1.0
            ]); 
            vertex.set_color([
                random::<f32>(),
                random::<f32>(),
                random::<f32>(),
                random::<f32>(),
            ]);
        }
    }

    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x2,
            1 => Float32x4,
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

impl IndexBuffer{
    pub fn new() -> Self {
        Self {
            indices: vec![],
        }
    }

    pub fn add_index(&mut self, index: u32) {
        self.indices.append(&mut vec![index]);
    }

    pub fn get_indices(&mut self) -> &Vec<u32> {
       &self.indices
    }
}
