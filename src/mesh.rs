use crate::buffers;
use std::fmt;

pub struct Mesh {
    vb: buffers::VertexBuffer,
    ib: buffers::IndexBuffer,
}

impl fmt::Display for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Mesh [")?;
        writeln!(f, "{}", self.vb)?;
        writeln!(f, "{}", self.ib)?;
        writeln!(f, "]")
    }
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
