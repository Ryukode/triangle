use std::ops::{Div, DivAssign, Mul, MulAssign};

use crate::{matrix::Matrix4, vector::Vector3};

#[derive(Clone, Copy)]
pub struct Quaternion{
    w: f32,
    i: f32,
    j: f32,
    k: f32,
}

impl Quaternion{
    pub fn new(w: f32, i: f32, j: f32, k: f32) -> Self{
        Self{
            w,
            i,
            j,
            k,
        }
    }

    pub fn from_vector_3(v: Vector3<f32>) -> Self {
        Self::from_vec(v.as_vec())
    }

    pub fn from_vec(v: Vec<f32>) -> Self {
        match v.len() {
            0 => {
                Self::default()
            },
            1 => {
                Self::from_scalar(*v.get(0).unwrap())
            },
            2 => {
                Self::new(*v.get(0).unwrap(), *v.get(1).unwrap(), *v.get(1).unwrap(), *v.get(1).unwrap())
            },
            3 => {
                Self::new(0.0, *v.get(0).unwrap(), *v.get(1).unwrap(), *v.get(2).unwrap())
            },
            _ => {
                Self::new(*v.get(0).unwrap(), *v.get(1).unwrap(), *v.get(2).unwrap(), *v.get(3).unwrap())
            }
        }
    }

    pub fn from_scalar(v: f32) -> Self {
        Self::new(v, v, v, v)
    }

    pub fn from_angle_axis(angle: f32, mut axis: Vector3<f32>) -> Self {
        let n: &Vector3<f32> = axis.normalize();
        let half_sin = f32::sin(angle/2.0);
        let half_cos = f32::cos(angle/2.0);
        Self{
            w: half_cos,
            i: half_sin * n.x,
            j: half_sin * n.y,
            k: half_sin * n.z,
        }
    }

    pub fn from_euler_angles(yaw: f32, pitch: f32, roll: f32) -> Self {
        let half_yaw = 0.5 * yaw;
        let half_pitch = 0.5 * pitch;
        let half_roll = 0.5 * roll;
        
        let cx = f32::cos(half_pitch);
        let cy = f32::cos(half_yaw);
        let cz = f32::cos(half_roll);

        let sx = f32::sin(half_pitch);
        let sy = f32::sin(half_yaw);
        let sz = f32::sin(half_roll);

        Self {
            w: cx * cy * cz + sx * sy * sz,
            i: sx * cy * cz - cx * sy * sz,
            j: cx * sy * cz + sx * cy * sz,
            k: cx * cy * sz - sx * sy * cz,
        }
    }

    pub fn from_euler_vector(angles: Vector3<f32>) -> Self {
        Quaternion::from_euler_angles(angles.x, angles.y, angles.z)
    }

    pub fn normalized(self) -> Self{
        self.clone()/self.norm()
    }

    pub fn normalize(mut self) -> Self {
        self/=self.norm();
        self
    }

    pub fn conjugate(self) -> Self {
        Self { 
            w: self.w, 
            i: -1.0 * self.i, 
            j: -1.0 * self.j, 
            k: -1.0 * self.k 
        }
    }

    pub fn norm(self) -> f32 {
        let norm = f32::sqrt(self.dot(self));
        norm
    }

    pub fn inverse(self) -> Self {
        let inv = self.conjugate() / self.dot(self);
        inv
    }

    pub fn dot(self, rhs: Self) -> f32 {
        let dot = self.w * rhs.w + self.i * rhs.i + self.j * rhs.j + self.k * rhs.k;
        dot
    }

    pub fn as_mat4(self) -> Matrix4 {
        let q: Quaternion = self.normalized();
        let (w, i, j, k) = (q.w, q.i, q.j, q.k);
        let (i2, j2, k2) = (i*2.0, j*2.0, k*2.0);
        let (ii, ij, ik, jj, jk, kk, wi, wj, wk) = (i*i2, i*j2, i*k2, j*j2, j*k2, k*k2, w*i2, w*j2, w*k2);

        let mat: Matrix4 = Matrix4::from_values(
            1.0 - (jj + kk), ij - wk, ik + wj, 0.0,
            ij + wk, 1.0 - (ii + kk), jk - wi, 0.0,
            ik - wj, jk + wi, 1.0 - (ii + jj), 0.0,
            0.0, 0.0, 0.0, 1.0);

        mat
    }

    pub fn as_vec(self) -> Vec<f32> {
        self.as_mat4().as_vec()
    }
}

impl Default for Quaternion{
    fn default() -> Self {
        Self{
            w: 1.,
            i: 0.,
            j: 0.,
            k: 0.,
        }
    }
}

impl Mul for Quaternion{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut tmp: Quaternion = self.clone();
        tmp *= rhs;
        tmp
    }
}

impl MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) { 
        self.w = self.w * rhs.w - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k;
        self.i = self.w * rhs.i + self.i * rhs.w + self.j * rhs.k - self.k * rhs.j;
        self.j = self.w * rhs.j - self.i * rhs.k + self.j * rhs.w + self.k * rhs.i;
        self.k = self.w * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.w;
    }
}

impl Div<f32> for Quaternion{
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let mut tmp: Quaternion = self.clone();
        tmp /= rhs;
        tmp
    }
}

impl DivAssign<f32> for Quaternion {
    fn div_assign(&mut self, rhs: f32) {
        self.w /= rhs;
        self.i /= rhs;
        self.j /= rhs;
        self.k /= rhs;
    }
}
