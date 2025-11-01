use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Vector3<f32> {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn normalize(&mut self) -> &Self{
        let n = self.length();
        if n != 0. {
            self.x /= n;
            self.y /= n;
            self.z /= n;
        }
        self
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        let mut res = Vector3::new(0., 0., 0.);

        res.x = self.y * rhs.z - self.z * rhs.y;
        res.y = self.z * rhs.x - self.x * rhs.z;
        res.z = self.x * rhs.y - self.y * rhs.x;

        res
    }
}

impl Default for Vector3<f32> {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Add for Vector3<f32>{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;

        res.x += rhs.x;
        res.y += rhs.y;
        res.z += rhs.z;

        res
    }
}

impl Add<f32> for Vector3<f32> {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        let mut res = self;

        res.x += rhs;
        res.y += rhs;
        res.z += rhs;

        res
    }
}

impl Sub for Vector3<f32> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self;

        res.x -= rhs.x;
        res.y -= rhs.y;
        res.z -= rhs.z;

        res
    }
}

impl Sub<f32> for Vector3<f32> {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        let mut res = self;

        res.x -= rhs;
        res.y -= rhs;
        res.z -= rhs;

        res
    }
}

impl Mul<f32> for Vector3<f32> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut res = self;

        res.x *= rhs;
        res.y *= rhs;
        res.z *= rhs;

        res
    }
}

impl Div<f32> for Vector3<f32> {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        let mut res = self;

        res.x /= rhs;
        res.y /= rhs;
        res.z /= rhs;

        res
    }
}
