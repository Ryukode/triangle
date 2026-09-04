use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use inflate::{inflate_bytes, inflate_bytes_zlib};
use inflate::InflateStream;

use crate::Color;
use crate::texture::Texture;
use crate::util::util::ReadUtils;
use crate::util::filestream::FileStream;

#[derive(Copy, Clone)]
pub struct PNGHeader{
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: PNGColorType,
    compression_method: PNGCompressionMethod,
    filter_method: PNGFilterMethod,
    interlace_method: PNGInterlaceMethod,
}
#[derive(Clone)]
pub struct PNG{
    header: PNGHeader,
    palette: Vec<Color<u8>>,
    data: Vec<u8>,
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
            data: Vec::default(),
        }
    }
    pub fn add_color(&mut self, c: Color<u8>){
        self.palette.push(c);
    }
    pub fn add_pixel(&mut self, pixel: u8){
        self.data.push(pixel);
    }
    fn add_bytes(&mut self, bytes: &[u8]){
        self.data.extend_from_slice(bytes);
    }
    pub fn num_samples(&self) -> u32 {
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
    pub fn bytes_per_pixel(&self) -> u32{
        f32::ceil(self.header.bit_depth as f32 * self.num_samples() as f32 / 8f32) as u32
    }
    pub fn bits_per_pixel(&self) -> u32{
        self.header.bit_depth as u32 * self.num_samples()
    }
    pub fn scanline_length(&self) -> u32 {
        f32::ceil((self.bits_per_pixel() * self.header.width) as f32 / 8f32) as u32 + 1u32
    }
    pub fn as_texture(&self) -> Texture{
        let channels = self.num_samples();
        let mut t = Texture::new(self.header.width, self.header.height, channels);
        let bits = ReadUtils::bytes_to_bits(&self.data);
        let iter = bits.chunks(self.header.bit_depth as usize);
        for chunk in iter {
            let val = ReadUtils::bits_to_u8(chunk);
            println!("{val}");
            t.img.push(val);
        }
        t
    }

    fn filter_scanline(&self, filter_type: PNGFilterType, prev_scanline: Option<&[u8]>, scanline: &[u8]) -> Vec<u8>{
        let mut filtered: Vec<u8> = Vec::new();
        let raw = |x: i32| {
            let mut raw: u16 = 0;
            if x >= 0 {
                raw = scanline[x as usize] as u16;
            }
            raw
        };
        let prior = |x: i32| {
            match prev_scanline {
                Some(prev_scanline) => {
                    let mut prior = 0;
                    if x >= 0 {
                        prior = prev_scanline[x as usize] as u16;
                    }
                    prior
                },
                None => {
                    0
                }
            }
        };
        let peath = |a: u16, b: u16, c: u16| {
            let p: i32 = a as i32 + b as i32 - c as i32;
            let pa = ((p - a as i32) as i8).abs();
            let pb = ((p - b as i32) as i8).abs();
            let pc = ((p - c as i32) as i8).abs();

            if pa <= pb && pa <= pc {
                a
            }
            else if pb <= pc {
                b
            }
            else {
                c
            }
        };
        let fit = |val: u16| {
            (val % 255) as u8
        };
        match filter_type {
            PNGFilterType::None => {
                filtered.extend_from_slice(scanline);
            },
            PNGFilterType::Sub => {
                for i in 0..scanline.len() {
                    let prev_index: i32 = i as i32 - self.bytes_per_pixel() as i32;
                    let res = &raw(i as i32) + &raw(prev_index);
                    filtered.push((res % 255) as u8);
                }
            },
            PNGFilterType::Up => {
                for i in 0..scanline.len() {
                    let res = &raw(i as i32) + &prior(i as i32);
                    filtered.push(fit(res));
                }
            },
            PNGFilterType::Average => {
                for i in 0..scanline.len() {
                    let prev_index: i32 = i as i32 - self.bytes_per_pixel() as i32;
                    let res = &raw(i as i32) + (&raw(prev_index) + &prior(i as i32)) / 2;
                    filtered.push(fit(res));
                }
            },
            PNGFilterType::Paeth => {
                for i in 0..scanline.len() {
                    let prev_index: i32 = i as i32 - self.bytes_per_pixel() as i32;
                    let res = &raw(i as i32) + &peath(raw(prev_index), prior(i as i32), prior(prev_index));
                    filtered.push(fit(res));
                }
            }
        }

        filtered
    }

    pub fn load_static(filename: &str) -> Texture {
        // read png
        let mut fd = FileStream::new(match fs::File::open(filename){
            Ok(f) => f,
            Err(e) => {
                eprintln!("Unable to read png texture file! Returning default texture. Error: [{}]", e);
                return Texture::default()
            },
        });
        let mut header = [0; PNG::SIGNATURE_SIZE];
        fd.read(&mut header);
        if PNG::SIGNATURE != header{
            eprintln!("PNG signature not found. Returning default texture.");
            return Texture::default();
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
            let data_length: u32 = u32::from_be_bytes(length_buf);

            // Read Chunk Type: four Bytes, where the 5th bit of each specifies an attribute
            let mut chunk_type_buf = [0u8; PNG::CHUNK_TYPE_SIZE];
            fd.read(&mut chunk_type_buf);
            let ancillary_bit = ReadUtils::byte_to_bits(chunk_type_buf[0])[3];
            let private_bit = ReadUtils::byte_to_bits(chunk_type_buf[1])[3];
            let reserved_bit = ReadUtils::byte_to_bits(chunk_type_buf[2])[3];
            // Irrelevant for decoders, but for the sake of completion
            let save_to_copy_bit = ReadUtils::byte_to_bits(chunk_type_buf[3])[3];

            if reserved_bit{
                eprintln!("reserved bit is not 0, returning default texture");
                return Texture::default();
            }

            if ancillary_bit || private_bit{
                eprintln!("ignoring ancillary and private chunks (for now)");
                fd.read(&mut vec![0u8; data_length as usize]);
                fd.read(&mut vec![0u8; PNG::CRC_SIZE]);
                continue;
            }

            let chunk_type: String = match String::from_utf8(chunk_type_buf.to_vec()) {
                Ok(s) => {
                    s
                },
                Err(e) => {
                    eprintln!("Unable to read chunk type as string, returning default Texture. [{}]", e.to_string());
                    return Texture::default();
                }
            };
            let mut chunk_data_buf: Vec<u8> = vec![0u8; data_length as usize];
            match chunk_type.as_str() {
                "IHDR" => {
                    if chunk_number != 1 {
                        eprintln!("IHDR at wrong position, returning default texture");
                        return Texture::default();
                    }

                    fd.read(&mut chunk_data_buf);
                    let bit_depth = chunk_data_buf[8];
                    let color_type = match PNGColorType::try_from(chunk_data_buf[9]){
                        Ok(t) => {
                            match PNGColorType::validate(t, bit_depth) {
                                true => t,
                                false => {
                                    eprintln!("Invalid color type and bit depth combination.");
                                    return Texture::default();
                                },
                            }
                        },
                        Err(()) => {
                            eprintln!("Unable to convert color type.");
                            return Texture::default();
                        }
                    };
                    png = Some(PNG::new(PNGHeader {
                        width: ReadUtils::read_be_u32(&mut &chunk_data_buf[0..4]),
                        height: ReadUtils::read_be_u32(&mut &chunk_data_buf[4..8]),
                        bit_depth,
                        color_type,
                        compression_method: match chunk_data_buf[10] {
                            0 => PNGCompressionMethod::Deflate,
                            _ => {
                                eprintln!("Unrecognized Compression Method! Returning default texture.");
                                return Texture::default();
                            }
                        },
                        filter_method: match chunk_data_buf[11] {
                            0 => PNGFilterMethod::Adaptive,
                            _ => {
                                eprintln!("Unrecognized Filter Method! Returning default texture.");
                                return Texture::default();
                            }
                        },
                        interlace_method: match chunk_data_buf[12]{
                            0 => PNGInterlaceMethod::None,
                            1 => PNGInterlaceMethod::Adam7,
                            _ => {
                                eprintln!("Unrecognized Interlace Method! Returning default texture.");
                                return Texture::default();
                            }
                        },
                    }));
                },
                "PLTE" => { // Color Palette
                    if data_length % PNG::PALETTE_CHANNELS != 0{
                        eprintln!("Invalid Palette length, returning default texture");
                        return Texture::default();
                    }
                    let p = match png.as_mut(){
                        Some(p) => p,
                        None => {
                            eprintln!("No png object found. Returning default texture.");
                            return Texture::default();
                        }
                    };
                    let h = p.header;
                    match h.color_type {
                        PNGColorType::Indexed => {
                            if data_length / PNG::PALETTE_CHANNELS != h.bit_depth as u32 {
                                eprintln!("Invalid Palette length, returning default texture");
                                return Texture::default();
                            }

                            for c in (0..data_length).step_by(PNG::PALETTE_CHANNELS as usize) {
                                let r:u8 = chunk_data_buf[c as usize];
                                let g:u8 = chunk_data_buf[(c+1) as usize];
                                let b:u8 = chunk_data_buf[(c+2) as usize];
                                let color: Color<u8> = Color{
                                    r, g, b, a:255
                                };
                                p.add_color(color);
                            }
                        },
                        PNGColorType::TrueColor | PNGColorType::TrueColorAlpha => {
                            if data_length / PNG::PALETTE_CHANNELS > 256 {
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
                    let p = match png.as_mut(){
                        Some(p) => p,
                        None => {
                            eprintln!("No png object found. Returning default texture.");
                            return Texture::default();
                        }
                    };
                    let h = p.header;
                    fd.read(&mut chunk_data_buf);
                    match h.compression_method {
                        PNGCompressionMethod::Deflate => {
                            let mut is = InflateStream::from_zlib();
                            let mut num_decoded_bytes: usize = 0;
                            let mut decoded: Vec<u8> = Vec::new();
                            loop {
                                let range = num_decoded_bytes..chunk_data_buf.len();
                                let data = &chunk_data_buf[range.clone()];
                                let mut tmp_file = match File::create("tmp/data.bin") {
                                    Ok(f) => {
                                        f
                                    },
                                    Err(e) => {
                                        eprintln!("Error creating file. Error: [{}]", e);
                                        return Texture::default();
                                    }
                                };
                                match tmp_file.write_all(chunk_data_buf.as_slice()) {
                                    Ok(_) => {

                                    },
                                    Err(e) => {
                                        eprintln!("Error writing file. Error: [{}]", e);
                                    }
                                }

                                match is.update(data) {
                                    Ok((size, res)) => {
                                        if size == 0 {
                                            break;
                                        }
                                        num_decoded_bytes += size;
                                        decoded.append(&mut res.to_vec());
                                    },
                                    Err(e) => {
                                        eprintln!("PNG Inflation failed! Error: {}", e);
                                        return Texture::default();
                                    }
                                };
                            }
                            let scanline_length = p.scanline_length();
                            let iter = decoded.chunks(scanline_length as usize);
                            let mut prev_scanline = None;
                            for chunk in iter {
                                let filter_type = match PNGFilterType::try_from(chunk[0]) {
                                    Ok(f) => f,
                                    Err(_) => {
                                        eprintln!("Invalid PNG filter type provided. Error: [{}]", chunk[0]);
                                        return Texture::default();
                                    }
                                };
                                let data = &chunk[1..];
                                let filtered = p.filter_scanline(filter_type, prev_scanline, data);
                                p.add_bytes(&filtered);
                                println!("Data length: {}", p.data.len());
                                prev_scanline = Some(data);
                            }
                        },
                    }
                },
                "IEND" => {
                    eof = true;
                },
                _ => {
                    eprintln!("png chunk type not recognized, skipping");
                    fd.read(&mut vec![0u8; data_length as usize]);
                    fd.read(&mut vec![0u8; PNG::CRC_SIZE]);
                    continue
                }
            }

            let mut crc_buf = [0; PNG::CRC_SIZE];
            fd.read(&mut crc_buf);
        }
        let res = match png {
            Some(p) => p.as_texture(),
            None => {
                return Texture::default();
            },
        };
        res
    }
}
#[derive(Copy, Clone)]
pub enum PNGColorType {
    Indexed = 0, TrueColor = 2, Grayscale = 3, GrayscaleAlpha = 4, TrueColorAlpha = 6,
}
impl PNGColorType {
    fn validate(color_type: PNGColorType, bit_depth: u8) -> bool {
        match color_type {
            PNGColorType::Grayscale => {
                match bit_depth {
                    1|2|4|8|16 => true,
                    _ => false
                }
            },
            PNGColorType::TrueColor => {
                match bit_depth {
                    8|16 => true,
                    _ => false
                }
            },
            PNGColorType::Indexed => {
                match bit_depth {
                    1|2|4|8 => true,
                    _ => false
                }
            },
            PNGColorType::GrayscaleAlpha => {
                match bit_depth {
                    8|16 => true,
                    _ => false
                }
            },
            PNGColorType::TrueColorAlpha => {
                match bit_depth {
                    8|16 => true,
                    _ => false
                }
            }
        }
    }
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
#[derive(Copy, Clone, Debug)]
pub enum PNGFilterType {
    None, Sub, Up, Average, Paeth
}
impl TryFrom<u8> for PNGFilterType {
    type Error = ();
    fn try_from(v: u8) -> Result<PNGFilterType, ()> {
        match v {
            0 => Ok(PNGFilterType::None),
            1 => Ok(PNGFilterType::Sub),
            2 => Ok(PNGFilterType::Up),
            3 => Ok(PNGFilterType::Average),
            4 => Ok(PNGFilterType::Paeth),
            _ => Err(())
        }
    }
}
impl Display for PNGFilterType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}