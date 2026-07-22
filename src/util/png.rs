use std::convert::TryFrom;
use std::fs;
use std::io::Read;
use inflate::inflate_bytes;

use crate::Color;
use crate::texture::Texture;
use crate::util::util::ReadUtils;
use crate::util::filestream::FileStream;

#[derive(Copy, Clone)]
pub struct PNGHeader{
    width: u32,
    height: u32,
    bit_depth: u32,
    color_type: PNGColorType,
    compression_method: PNGCompressionMethod,
    filter_method: PNGFilterMethod,
    interlace_method: PNGInterlaceMethod,
}
#[derive(Clone)]
pub struct PNG{
    header: PNGHeader,
    palette: Vec<Color<u8>>,
    pixels: Vec<u16>,
}
impl PNG{
    pub const SIGNATURE_SIZE: usize = 8;
    pub const SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    pub const LENGTH_SIZE: usize = 4;
    pub const CHUNK_TYPE_SIZE: usize = 4;
    pub const CRC_SIZE: usize = 4;
    pub const PALETTE_CHANNELS: u32 = 3;
    pub const MAX_BIT_DEPTH: u32 = 16;
    pub fn new(header: PNGHeader) -> Self{
        PNG{
            header,
            palette: Vec::default(),
            pixels: Vec::default(),
        }
    }
    pub fn add_color(&mut self, c: Color<u8>){
        self.palette.push(c);
    }
    pub fn add_pixel(&mut self, pixel: u16){
        self.pixels.push(pixel);
    }
    pub fn num_samples(&self) -> u32{
        match self.header.color_type {
            PNGColorType::Indexed | PNGColorType::Grayscale => {
                1
            },
            PNGColorType::GrayscaleAlpha => {
                2
            },
            PNGColorType::TrueColor => {
                3
            }
            PNGColorType::TrueColorAlpha => {
                4
            }
        }
    }
    pub fn bpp(&self) -> u32{
        self.header.bit_depth * self.num_samples()
    }
    pub fn as_texture(&self) -> Texture{
        let mut t = Texture::new(self.header.width, self.header.height, 4);
        for pixel in self.pixels.get(0..self.pixels.len()-1).unwrap() {

        }
        match self.header.color_type {
            PNGColorType::Indexed => {

            },
            PNGColorType::Grayscale => {

            },
            PNGColorType::GrayscaleAlpha => {

            },
            PNGColorType::TrueColor => {

            },
            PNGColorType::TrueColorAlpha => {

            },
        }
        t
    }

    pub fn load_static(filename: &str) -> Texture {
        // read png
        let mut fd = FileStream::new(fs::File::open(filename).expect("Failed to open png texture file!"));
        let mut header = [0; PNG::SIGNATURE_SIZE];
        fd.read(&mut header);
        if PNG::SIGNATURE != header{
            panic!("PNG signature not found. Panicing")
        }

        let mut eof: bool = false;
        let mut chunk_number: u16 = 0;

        let mut header: Option<PNGHeader> = None;
        let mut texture: Option<Texture> = None;
        let mut png: Option<PNG> = None;
        // iterate over chunks
        while !eof{
            chunk_number += 1;
            let mut length_buf= [0u8; PNG::LENGTH_SIZE];
            fd.read(&mut length_buf);
            let length: u32 = u32::from_be_bytes(length_buf);

            let mut chunk_type_buf = [0u8; PNG::CHUNK_TYPE_SIZE];
            fd.read(&mut chunk_type_buf);
            let ancillary_bit = ReadUtils::byte_to_bits(chunk_type_buf[0])[5];
            let private_bit = ReadUtils::byte_to_bits(chunk_type_buf[1])[5];
            let reserved_bit = ReadUtils::byte_to_bits(chunk_type_buf[2])[5];

            if reserved_bit{
                eprintln!("reserved bit is not 0, returning default texture");
                return Texture::default();
            }

            if !ancillary_bit || private_bit{
                eprintln!("ignoring ancillary and private chunks (for now)");
                fd.read(&mut vec![0u8; length as usize]);
                fd.read(&mut vec![0u8; PNG::CRC_SIZE]);
                continue;
            }

            let mut chunk_type: String = String::from("");
            if chunk_type_buf.is_ascii() {
                chunk_type = String::from_utf8(chunk_type_buf.to_vec()).unwrap();
            } else {
                eprintln!("invalid ASCII for chunk type");
            }

            let mut chunk_data_buf: Vec<u8> = Vec::default();

            match chunk_type.as_str() {
                "IHDR" => { // Header
                    fd.read(&mut chunk_data_buf);

                    if chunk_number != 1 {
                        eprintln!("IHDR at wrong position, returning default texture");
                        return Texture::default();
                    }

                    let bit_depth: u8 = chunk_data_buf[8];
                    png = Some(PNG::new(PNGHeader {
                        width: ReadUtils::read_be_u32(&mut &chunk_data_buf[0..4]),
                        height: ReadUtils::read_be_u32(&mut &chunk_data_buf[4..8]),
                        bit_depth: ReadUtils::read_be_u32(&mut &chunk_data_buf[8..9]),
                        color_type: match PNGColorType::try_from(chunk_data_buf[9]).unwrap() {
                            PNGColorType::Grayscale => {
                                match bit_depth {
                                    1|2|4|8|16 => {

                                    },
                                    _ => {
                                        eprintln!("invalid bit_depth, returning default texture");
                                        return Texture::default();
                                    }
                                }
                                PNGColorType::Grayscale
                            },
                            PNGColorType::TrueColor => {
                                match bit_depth {
                                    8|16 => {

                                    },
                                    _ => {
                                        eprintln!("invalid bit_depth, returning default texture");
                                        return Texture::default();
                                    }
                                }
                                PNGColorType::TrueColor
                            },
                            PNGColorType::Indexed => {
                                match bit_depth {
                                    1|2|4|8 => {

                                    },
                                    _ => {
                                        eprintln!("invalid bit_depth, returning default texture");
                                        return Texture::default();
                                    }
                                }
                                PNGColorType::Indexed
                            },
                            PNGColorType::GrayscaleAlpha => {
                                match bit_depth {
                                    8|16 => {

                                    },
                                    _ => {
                                        eprintln!("invalid bit_depth, returning default texture");
                                        return Texture::default();
                                    }
                                }
                                PNGColorType::GrayscaleAlpha
                            },
                            PNGColorType::TrueColorAlpha => {
                                match bit_depth {
                                    8|16 => {

                                    },
                                    _ => {
                                        eprintln!("invalid bit_depth, returning default texture");
                                        return Texture::default();
                                    }
                                }
                                PNGColorType::TrueColorAlpha
                            },
                            _ => {
                                eprintln!("invalid color type, returning default texture");
                                return Texture::default();
                            }
                        },
                        compression_method: match chunk_data_buf[10] {
                            0 => PNGCompressionMethod::Deflate,
                            _ => {
                                eprintln!("Unrecognized Compression Method! Assuming DEFLATE");
                                PNGCompressionMethod::Deflate
                            }
                        },
                        filter_method: match chunk_data_buf[11] {
                            0 => PNGFilterMethod::Adaptive,
                            _ => {
                                eprintln!("Unrecognized Filter Method! Assuming Adaptive Filtering");
                                PNGFilterMethod::Adaptive
                            }
                        },
                        interlace_method: match chunk_data_buf[12]{
                            0 => PNGInterlaceMethod::None,
                            1 => PNGInterlaceMethod::Adam7,
                            _ => {
                                eprintln!("Unrecognized Interlace Method! Assuming None");
                                PNGInterlaceMethod::None
                            }
                        },
                    }));
                },
                "PLTE" => { // Color Palette
                    if length % PNG::PALETTE_CHANNELS != 0{
                        eprintln!("Invalid Palette length, returning default texture");
                        return Texture::default();
                    }
                    match header.as_mut().unwrap().color_type {
                        PNGColorType::Indexed => {
                            if length / PNG::PALETTE_CHANNELS != header.as_mut().unwrap().bit_depth{
                                eprintln!("Invalid Palette length, returning default texture");
                            }

                            for c in (0..length).step_by(PNG::PALETTE_CHANNELS as usize) {
                                let r:u8 = chunk_data_buf[c as usize];
                                let g:u8 = chunk_data_buf[(c+1) as usize];
                                let b:u8 = chunk_data_buf[(c+2) as usize];
                                let color: Color<u8> = Color{
                                    r, g, b, a:255
                                };
                                png.as_mut().expect("No png object found").add_color(color);
                            }
                        },
                        PNGColorType::TrueColor | PNGColorType::TrueColorAlpha => {
                            if length / PNG::PALETTE_CHANNELS > 256 {
                                eprintln!("Invalid Palette length, returning default texture");
                                return Texture::default();
                            }
                        }
                        _ => {
                            eprintln!("Palette not expected for this color type, returning default texture");
                            return Texture::default();
                        }
                    }
                },
                "IDAT" => {
                    let p = png.as_mut().expect("No png object found");
                    let h = p.header;
                    match h.compression_method {
                        PNGCompressionMethod::Deflate => {
                            let decoded = inflate_bytes(&chunk_data_buf).expect("inflation failed");
                            let scanline_length = f32::ceil((h.width * p.bpp()) as f32 / 8.0) as u32 + 1;
                            let mut scanline;
                            for i in 0..h.height {
                                let line_start = (i * scanline_length) as usize;
                                let line_end = ((i+1) * scanline_length - 1) as usize;
                                scanline = decoded.get(line_start..line_end).unwrap();

                                {
                                    //let mut fs = FileStream::new();
                                    // TODO: Get bits for one pixel
                                    let bits_start = 0;
                                    //let bits_end = f32::ceil(p.bpp());
                                }

                                for j in (1..scanline_length).step_by(2) {
                                    let range_start: usize = j as usize;
                                    let range_end = (j + 2) as usize;
                                    let range = range_start ..range_end;
                                    let bits = ReadUtils::bytes_to_bits(&scanline[range]);
                                    let num_pixels = PNG::MAX_BIT_DEPTH/h.bit_depth;
                                    for bit in 0..num_pixels {
                                        let pixel_start = (bit*h.bit_depth) as usize;
                                        let pixel_end = pixel_start + h.bit_depth as usize;
                                        let pixel = bits.get(pixel_start..pixel_end).expect("missing pixel");
                                        let value = ReadUtils::bits_to_u16(pixel);
                                        p.add_pixel(value);
                                    }
                                }

                                let filter_type = scanline.get(0).expect("missing filter type byte");
                                match PNGFilterTypes::try_from(*filter_type).expect("unknown filter type") {
                                    PNGFilterTypes::None => {

                                    },
                                    PNGFilterTypes::Sub => {

                                    },
                                    PNGFilterTypes::Up => {

                                    },
                                    PNGFilterTypes::Average => {

                                    },
                                    PNGFilterTypes::Paeth => {

                                    }
                                }
                            }
                        },
                    }
                },
                "IEND" => {
                    eof = true;
                },
                _ => {
                    eprintln!("png chunk type not recognized, skipping");
                    fd.read(&mut vec![0u8; length as usize]);
                    fd.read(&mut vec![0u8; PNG::CRC_SIZE]);
                    continue
                }
            }

            let mut crc_buf = [0; PNG::CRC_SIZE];
            fd.read(&mut crc_buf);
        }

        // add pixel values to img array
        // return texture

        Texture::default()
    }
}
#[derive(Copy, Clone)]
pub enum PNGColorType {
    Indexed = 0, TrueColor = 2, Grayscale = 3, GrayscaleAlpha = 4, TrueColorAlpha = 6,
}
impl TryFrom<u8> for PNGColorType{
    type Error = ();
    fn try_from(v: u8) -> Result<PNGColorType, ()> {
        match v {
            0 => Ok(PNGColorType::Grayscale),
            2 => Ok(PNGColorType::TrueColor),
            3 => Ok(PNGColorType::Indexed),
            4 => Ok(PNGColorType::GrayscaleAlpha),
            6 => Ok(PNGColorType::TrueColorAlpha),
            _ => Err(())
        }
    }
}
#[derive(Copy, Clone)]
pub enum PNGCompressionMethod {
    Deflate,
}
#[derive(Copy, Clone)]
pub enum PNGFilterMethod {
    Adaptive,
}
#[derive(Copy, Clone)]
pub enum PNGInterlaceMethod {
    None,
    Adam7
}
#[derive(Copy, Clone)]
pub enum PNGFilterTypes {
    None, Sub, Up, Average, Paeth
}
impl TryFrom<u8> for PNGFilterTypes{
    type Error = ();
    fn try_from(v: u8) -> Result<PNGFilterTypes, ()> {
        match v {
            0 => Ok(PNGFilterTypes::None),
            1 => Ok(PNGFilterTypes::Sub),
            2 => Ok(PNGFilterTypes::Up),
            3 => Ok(PNGFilterTypes::Average),
            4 => Ok(PNGFilterTypes::Paeth),
            _ => Err(())
        }
    }
}