use crate::matrix::Matrix4;
use crate::transform::Transform;

pub struct Camera {
    pub transform: Transform,
    projection_matrix: Matrix4,
}

impl Camera{
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self{
            transform: Transform::default(),
            projection_matrix: Matrix4::project(fov, aspect, near, far),
        }
    }

    pub fn get_projection_matrix(&self) -> Matrix4 {
        self.projection_matrix
    }
}
