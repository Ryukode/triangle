use rand::random;
use std::{fmt};
use crate::matrix::Matrix4;

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: [f32; 3],
    color: [f32; 4],
}

impl fmt::Display for Vertex{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vertex: x: {}, y: {}, z: {}, r: {}, g: {}, b: {}, a: {}", self.pos[0], self.pos[1], self.pos[2], self.color[0], self.color[1], self.color[2], self.color[3])
    }
}

impl Vertex {
    pub fn new() -> Self {
        Self{
            pos: [0.0, 0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn set_position(&mut self, pos: [f32; 3]) {
        self.pos = pos;
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    pub fn _get_position(&mut self) -> [f32; 3]{
        self.pos
    }

    pub fn as_vec(&self) -> Vec<f32> {
        let a = [self.pos.to_vec(),self.color.to_vec()].concat();
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

    pub fn get_vertices(&mut self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn as_vec(&self) -> Vec<f32> {
        let mut verts = vec![];
        for v in &self.vertices[..] {
            verts = [verts, v.as_vec()].concat()
        }
        verts
    }

    pub fn update(&mut self){
        for vertex in &mut self.vertices {
            vertex.set_position([
                random::<f32>() * 2.0 - 1.0,
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
        array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x3,
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

impl IndexBuffer {
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

pub struct UniformBuffer {
    m_model: Matrix4,
    m_view: Matrix4,
    m_projection: Matrix4,
    time: Vec<f32>,
}

impl UniformBuffer {
    pub fn new(model: Matrix4, view: Matrix4, projection: Matrix4) -> Self {
        Self {
            m_model: model,
            m_view: view,
            m_projection: projection,
            time: vec![1., 1., 1., 1.],
        }
    }

    pub fn as_vec(&self) -> Vec<f32> {
        let a = [self.m_model.as_vec(), self.m_view.as_vec(), self.m_projection.as_vec(), self.time.clone()].concat();
        a
    }
}
