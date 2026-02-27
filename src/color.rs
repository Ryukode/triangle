pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
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

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 1.,
        }
    }
}