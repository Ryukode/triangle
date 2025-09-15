use std::ops;
use std::f32;

pub struct Matrix4 {
    m11: f32, m12: f32, m13: f32, m14: f32,
    m21: f32, m22: f32, m23: f32, m24: f32,
    m31: f32, m32: f32, m33: f32, m34: f32,
    m41: f32, m42: f32, m43: f32, m44: f32,
}


impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;
    
    fn mul(self, rhs: Matrix4) -> Self::Output {
        Self{
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31 + self.m14 * rhs.m41,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31 + self.m24 * rhs.m41,
            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31 + self.m34 * rhs.m41,
            m41: self.m41 * rhs.m11 + self.m42 * rhs.m21 + self.m43 * rhs.m31 + self.m44 * rhs.m41,

            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32 + self.m14 * rhs.m42,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32 + self.m24 * rhs.m42,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32 + self.m34 * rhs.m42,
            m42: self.m41 * rhs.m12 + self.m42 * rhs.m22 + self.m43 * rhs.m32 + self.m44 * rhs.m42,

            m13: self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33 + self.m14 * rhs.m43,
            m23: self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33 + self.m24 * rhs.m43,
            m33: self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33 + self.m34 * rhs.m43,
            m43: self.m41 * rhs.m13 + self.m42 * rhs.m23 + self.m43 * rhs.m33 + self.m44 * rhs.m43,

            m14: self.m11 * rhs.m14 + self.m12 * rhs.m24 + self.m13 * rhs.m34 + self.m14 * rhs.m44,
            m24: self.m21 * rhs.m14 + self.m22 * rhs.m24 + self.m23 * rhs.m34 + self.m24 * rhs.m44,
            m34: self.m31 * rhs.m14 + self.m32 * rhs.m24 + self.m33 * rhs.m34 + self.m34 * rhs.m44,
            m44: self.m41 * rhs.m14 + self.m42 * rhs.m24 + self.m43 * rhs.m34 + self.m44 * rhs.m44,
        }
    }
}

impl Matrix4{
    pub fn new() -> Matrix4 {
        Self {
            m11: 0., m12: 0., m13: 0., m14: 0.,
            m21: 0., m22: 0., m23: 0., m24: 0.,
            m31: 0., m32: 0., m33: 0., m34: 0.,
            m41: 0., m42: 0., m43: 0., m44: 0.,
        }
    }

    pub fn identity() -> Matrix4 {
        Self {
            m11: 1., m12: 0., m13: 0., m14: 0.,
            m21: 0., m22: 1., m23: 0., m24: 0.,
            m31: 0., m32: 0., m33: 1., m34: 0.,
            m41: 0., m42: 0., m43: 0., m44: 1.,
        }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Matrix4 {
        let mut t: Matrix4 = Matrix4::identity();
        t.m14 = x;
        t.m24 = y;
        t.m34 = z;
        t
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Matrix4 {
        let mut s: Matrix4 = Matrix4::identity();
        s.m11 = x;
        s.m22 = y;
        s.m33 = z;
        s
    }

    pub fn rotateEuler(x: f32, y: f32, z: f32) -> Matrix4 {
        let a: Matrix4 = Matrix4::rotateEulerX(x);
        let b: Matrix4 = Matrix4::rotateEulerY(y);
        let c: Matrix4 = Matrix4::rotateEulerZ(z);
        c * b * a
    }

    pub fn rotateEulerX(x: f32) -> Matrix4 {
        Self {
            m11: 1., m12: 0., m13: 0., m14: 0.,
            m21: 0., m22: f32::cos(x), m23: -f32::sin(x), m24: 0.,
            m31: 0., m32: f32::sin(x), m33: f32::cos(x), m34: 0.,
            m41: 0., m42: 0., m43: 0., m44: 1.,
        }  
    }

    pub fn rotateEulerY(y: f32) -> Matrix4 {
        Self {
            m11: f32::cos(y), m12: 0., m13: f32::sin(y), m14: 0.,
            m21: 0., m22: 1., m23: 0., m24: 0.,
            m31: -f32::sin(y), m32: 0., m33: f32::cos(y), m34: 0.,
            m41: 0., m42: 0., m43: 0., m44: 1.,
        }  
    }

    pub fn rotateEulerZ(z: f32) -> Matrix4 {
        Self {
            m11: f32::cos(z), m12: -f32::sin(z), m13: 0., m14: 0.,
            m21: f32::sin(z), m22: f32::cos(z), m23: 0., m24: 0.,
            m31: 0., m32: 0., m33: 1., m34: 0.,
            m41: 0., m42: 0., m43: 0., m44: 1.,
        }  
    }

    pub fn transpose(self) -> Matrix4 {
        Matrix4 { 
            m11: (self.m11), m12: (self.m21), m13: (self.m31), m14: (self.m41), 
            m21: (self.m12), m22: (self.m22), m23: (self.m32), m24: (self.m42), 
            m31: (self.m13), m32: (self.m23), m33: (self.m33), m34: (self.m43), 
            m41: (self.m14), m42: (self.m24), m43: (self.m34), m44: (self.m44),
        }
    }
}

