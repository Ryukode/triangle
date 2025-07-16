use crate::buffers;

pub struct Mesh {
    vb: buffers::VertexBuffer,
    ib: buffers::IndexBuffer,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vb: buffers::VertexBuffer::new(),
            ib: buffers::IndexBuffer::new(),
        }
    }

    pub fn vb(&mut self) -> &mut buffers::VertexBuffer {
        &mut self.vb
    }

    pub fn ib(&mut self) -> &mut buffers::IndexBuffer {
        &mut self.ib
    }
}
