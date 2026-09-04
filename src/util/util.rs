use std::convert::TryInto;
pub struct ReadUtils;
impl ReadUtils {
    pub fn read_be_u32(input: &mut &[u8]) -> u32 {
        let (int_bytes, rest) = input.split_at(size_of::<u32>());
        *input = rest;
        u32::from_be_bytes(int_bytes.try_into().unwrap())
    }

    pub fn byte_to_bits(byte: u8) -> [bool; 8] {
        let mut bits = [false; 8];
        for i in 0..8 {
            bits[7-i] = (byte >> i) & 1 == 1;
        }
        bits
    }

    pub fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
        let mut bits = Vec::<bool>::new();
        for byte in bytes {
            for i in 0..8 {
                bits.push((byte >> i) & 1 == 1);
            }
        }
        bits
    }

    pub fn bits_to_u16(bits: &[bool]) -> u16 {
        let mut result: u16 = 0;
        for i in 0..bits.len() {
            if bits[i] {
                result += 2u16.pow(i as u32);
            }
        }
        result
    }
    pub fn bits_to_u8(bits: &[bool]) -> u8 {
        let mut result: u8 = 0;
        for i in 0..bits.len() {
            if bits[i] {
                result += &2u8.pow(i as u32);
            }
        }
        result
    }
}