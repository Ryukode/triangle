
use crate::Color;
use crate::util::png::PNG;


pub struct Texture {
    width: u32,
    height: u32,
    img: Vec<u8>,
}

impl Texture {

    pub fn new(width: u32, height: u32, channels: u32) -> Texture {
        Self {
            width,
            height,
            img: vec![0; (width * height * channels) as usize],
        }
    }

}

impl Default for Texture {
    fn default() -> Self {
        let width: u32 = 16;
        let height: u32 = 16;
        let dims: u32 = 4;

        Self{
            width,
            height,
            img: vec![0; (width * height * dims) as usize]
        }
    }
}