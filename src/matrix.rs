use std::ops::{self, Index, IndexMut};
use std::f32;
use crate::vector::Vector3;

#[derive(Copy, Clone)]
pub struct Matrix4 {
    m00: f32, m10: f32, m20: f32, m30: f32,
    m01: f32, m11: f32, m21: f32, m31: f32,
    m02: f32, m12: f32, m22: f32, m32: f32,
    m03: f32, m13: f32, m23: f32, m33: f32,
}

impl Index<(usize, usize)> for Matrix4 {
    type Output = f32;
    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        match (r, c) {
            (0, 0) => &self.m00, (1, 0) => &self.m10, (2, 0) => &self.m20, (3, 0) => &self.m30,
            (0, 1) => &self.m01, (1, 1) => &self.m11, (2, 1) => &self.m21, (3, 1) => &self.m31,
            (0, 2) => &self.m02, (1, 2) => &self.m12, (2, 2) => &self.m22, (3, 2) => &self.m32,
            (0, 3) => &self.m03, (1, 3) => &self.m13, (2, 3) => &self.m23, (3, 3) => &self.m33,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<(usize, usize)> for Matrix4 {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        let (r, c) = idx;
        match (r, c) {
            (0, 0) => &mut self.m00, (1, 0) => &mut self.m10, (2, 0) => &mut self.m20, (3, 0) => &mut self.m30,
            (0, 1) => &mut self.m01, (1, 1) => &mut self.m11, (2, 1) => &mut self.m21, (3, 1) => &mut self.m31,
            (0, 2) => &mut self.m02, (1, 2) => &mut self.m12, (2, 2) => &mut self.m22, (3, 2) => &mut self.m32,
            (0, 3) => &mut self.m03, (1, 3) => &mut self.m13, (2, 3) => &mut self.m23, (3, 3) => &mut self.m33,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut result = Matrix4 {
            m00: 0.0, m10: 0.0, m20: 0.0, m30: 0.0,
            m01: 0.0, m11: 0.0, m21: 0.0, m31: 0.0,
            m02: 0.0, m12: 0.0, m22: 0.0, m32: 0.0,
            m03: 0.0, m13: 0.0, m23: 0.0, m33: 0.0,
        };

        // Column-major math: result = self * rhs
        for c in 0..4 {          // column of result
            for r in 0..4 {      // row of result
                let mut sum = 0.0;
                for k in 0..4 {  // dot product
                    sum += self[(r, k)] * rhs[(k, c)];
                }
                result[(r, c)] = sum;
            }
        }

        result
    }

}

impl Matrix4{
    pub fn new() -> Matrix4 {
        Self {
            m00: 0., m10: 0., m20: 0., m30: 0.,
            m01: 0., m11: 0., m21: 0., m31: 0.,
            m02: 0., m12: 0., m22: 0., m32: 0.,
            m03: 0., m13: 0., m23: 0., m33: 0.,
        }
    }

    pub fn identity() -> Matrix4 {
        Self {
            m00: 1., m10: 0., m20: 0., m30: 0.,
            m01: 0., m11: 1., m21: 0., m31: 0.,
            m02: 0., m12: 0., m22: 1., m32: 0.,
            m03: 0., m13: 0., m23: 0., m33: 1.,
        }
    }

    pub fn translate(vec: Vector3<f32>) -> Matrix4 {
        let mut t: Matrix4 = Matrix4::identity();
        t.m30 = vec.x;
        t.m31 = vec.y;
        t.m32 = vec.z;
        t
    }

    pub fn scale(vec: Vector3<f32>) -> Matrix4 {
        let mut s: Matrix4 = Matrix4::identity();
        s.m00 = vec.x;
        s.m11 = vec.y;
        s.m22 = vec.z;
        s
    }

    pub fn rotate_euler(angles: Vector3<f32>) -> Matrix4 {
        let a: Matrix4 = Matrix4::rotate_euler_x(angles.x);
        let b: Matrix4 = Matrix4::rotate_euler_y(angles.y);
        let c: Matrix4 = Matrix4::rotate_euler_z(angles.z);
        c * b * a
    }

    pub fn rotate_euler_x(x: f32) -> Matrix4 {
        Self {
            m00: 1., m10: 0.,          m20: 0.,          m30: 0.,
            m01: 0., m11: f32::cos(x), m21: -f32::sin(x),m31: 0.,
            m02: 0., m12: f32::sin(x), m22: f32::cos(x), m32: 0.,
            m03: 0., m13: 0.,          m23: 0.,          m33: 1.,
        }
    }

    pub fn rotate_euler_y(y: f32) -> Matrix4 {
        Self {
            m00: f32::cos(y),  m10: 0., m20: f32::sin(y), m30: 0.,
            m01: 0.,           m11: 1., m21: 0.,          m31: 0.,
            m02: -f32::sin(y), m12: 0., m22: f32::cos(y), m32: 0.,
            m03: 0.,           m13: 0., m23: 0.,          m33: 1.,
        }
    }

    pub fn rotate_euler_z(z: f32) -> Matrix4 {
        Self {
            m00: f32::cos(z), m10: -f32::sin(z), m20: 0., m30: 0.,
            m01: f32::sin(z), m11: f32::cos(z),  m21: 0., m31: 0.,
            m02: 0.,          m12: 0.,           m22: 1., m32: 0.,
            m03: 0.,          m13: 0.,           m23: 0., m33: 1.,
        }  
    }

    pub fn transpose(self) -> Matrix4 {
        Matrix4 { 
            m00: (self.m00), m10: (self.m01), m20: (self.m02), m30: (self.m03),
            m01: (self.m10), m11: (self.m11), m21: (self.m12), m31: (self.m13),
            m02: (self.m20), m12: (self.m21), m22: (self.m22), m32: (self.m23),
            m03: (self.m30), m13: (self.m31), m23: (self.m32), m33: (self.m33),
        }
    }

    pub fn project(fov: f32, aspect: f32, z_near: f32, z_far: f32) -> Matrix4{
        let f = 1./(fov/2.).tan();
        Self{
            m00: f / aspect,      m10: 0.,           m20: 0.,                                m30: 0.,
            m01: 0.,              m11: f,            m21: 0.,                                m31: 0.,
            m02: 0.,              m12: 0.,           m22: (z_far + z_near)/(z_near - z_far), m32: -2. * z_far * z_near /(z_near - z_far),
            m03: 0.,              m13: 0.,           m23: -1., m33: 1.,
        }
    }

    pub fn as_vec(&self) -> Vec<f32> {
        let m = self.transpose();
        let a = vec![
             m.m00, m.m10, m.m20, m.m30,
             m.m01, m.m11, m.m21, m.m31,
             m.m02, m.m12, m.m22, m.m32,
             m.m03, m.m13, m.m23, m.m33];

        a
    }
}

