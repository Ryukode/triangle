use crate::buffers;
use std::fmt;
use crate::buffers::{IndexBuffer, VertexBuffer};

pub struct Mesh {
    pub vb: VertexBuffer,
    pub ib: IndexBuffer,
}

impl Mesh {
    pub fn new(vb: VertexBuffer, ib: IndexBuffer) -> Self {
        Self {
            vb,
            ib
        }
    }
}

impl fmt::Display for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Mesh [")?;
        writeln!(f, "{}", self.vb)?;
        writeln!(f, "{}", self.ib)?;
        writeln!(f, "]")
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vb: VertexBuffer::new(),
            ib: IndexBuffer::new()
        }
    }
}