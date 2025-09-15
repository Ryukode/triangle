use crate::matrix;

struct Camera {
    screen_width: u32,
    screen_height: u32,
    view_matrix: matrix::Matrix4,
}
