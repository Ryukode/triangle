use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

use crate::util::util::ReadUtils;

pub struct FileStream {
    fd: File,
    overflow: VecDeque<bool>,
    overflow_pointer: usize,
}

impl FileStream {
    pub fn new(f: File) -> FileStream {
        Self{
            fd: f,
            overflow: VecDeque::new(),
            overflow_pointer: 0,
        }
    }
    
    pub fn read_bits(&mut self, buf: &mut [bool]) {
        let buf_len = buf.len();

        let mut bits: VecDeque<bool> = VecDeque::new();
        for i in 0..buf_len {
            if !self.overflow.is_empty(){
                buf[i] = self.overflow.pop_front().expect("Error popping overflow buffer.");
                continue;
            }

            if bits.is_empty() {
                let mut byte_buf = [0u8; 1];
                match self.fd.read(&mut byte_buf) {
                    Ok(n) => {
                        if 0 <= n && n <= byte_buf.len(){
                            //TODO: Handle!
                        }
                        else {
                            eprintln!("Unable to read bits from file: {}", n);
                        }
                    },
                    Err(n) => {
                        eprintln!("Error reading byte from file: {}", n);
                    }
                }
                for j in ReadUtils::byte_to_bits(byte_buf[0]) {
                    bits.push_back(j);
                }
            }
            buf[i] = bits.pop_front().expect("Error popping bit buffer.");
        }
        for i in bits {
            self.overflow.push_back(i);
        }
    }
    
    pub fn read(&mut self, buf: &mut [u8]){
        match self.fd.read(buf) {
            Ok(n) => {
                if 0 == n && n <= buf.len(){
                    //TODO: Handle!
                }
                else {
                    eprintln!("Unable to read bytes from file: {}", n);
                }
            },
            Err(e) => {
                eprintln!("Error reading byte from file: {}", e);
            }
        }
        self.overflow.clear();
    }
}