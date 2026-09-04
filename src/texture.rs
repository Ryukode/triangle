use rand::Rng;
use crate::Color;
use crate::util::png::PNG;


pub struct Texture {
    width: u32,
    height: u32,
    channels: u32,
    pub(crate) img: Vec<u8>,
}

impl Texture {
    pub fn new(width: u32, height: u32, channels: u32) -> Texture {
        Self {
            width,
            height,
            channels,
            img: vec![0; (width * height * channels) as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn channels(&self) -> u32 {
        self.channels
    }
    pub fn disco(size: u32) -> Texture {
        Self{
            width: size,
            height: size,
            channels: 4,
            img: random_tex(size*size*4),
        }
    }
}
fn random_tex(length: u32) -> Vec<u8>{
    let mut pixels: Vec<u8> = Vec::new();
    let mut rng = rand::rng();
    for _ in 0..length {
        let mut val: u8 = rng.random();
        pixels.push(val);
    }
    pixels
}
impl Default for Texture {
    fn default() -> Self {
        let width: u32 = 32;
        let height: u32 = 32;
        let channels: u32 = 4;

        Self{
            width,
            height,
            channels,
            img: vec![255; (width * height * channels) as usize]
        }
    }
}