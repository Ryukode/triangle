#[derive(Copy, Clone)]
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl Color<f32> {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r,g,b,a
        }
    }
    pub fn as_vec(&self) -> Vec<f32>{
        let mut v: Vec<f32> = Vec::new();
        v.push(self.r);
        v.push(self.g);
        v.push(self.b);
        v.push(self.a);
        v
    }
}

impl Default for Color<f32> {
    fn default() -> Self {
        Self {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 1.,
        }
    }
}

impl Color<u8> {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self{
            r,g,b,a,
        }
    }

    pub fn as_vec(&self) -> Vec<u8>{
        let mut v: Vec<u8> = Vec::new();
        v.push(self.r);
        v.push(self.g);
        v.push(self.b);
        v.push(self.a);
        v
    }
}

impl Default for Color<u8> {
    fn default() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}