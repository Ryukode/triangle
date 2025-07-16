use rand::random;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
}

impl Vertex {
    pub fn new() -> Self {
        Self{
            pos: [0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn position(&mut self, pos: [f32; 2]) {
        self.pos = pos;
    }

    pub fn color(&mut self, color: [f32; 4]) {
        self.color = color;
    }
}

pub struct VertexBuffer {
    vertices: Vec<Vertex>
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

    pub fn add_vertices(&mut self, mut vertices: Vec<Vertex>){
        self.vertices.append(&mut vertices);
    }

    pub fn get_vertices(&mut self) -> Vec<Vertex> {
        self.vertices.clone()
    }

    pub fn update(&mut self){
        for vertex in &mut self.vertices {
            vertex.position([random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0]); 
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
