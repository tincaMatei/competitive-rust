#![allow(dead_code)]

use std::io::{Read, BufRead, Stdin, BufReader};
use std::fs::File;
use std::str::FromStr;
use std::fmt::Debug;

pub struct InParser<T: Read> {
    reader: BufReader<T>,
    buffer: Vec<u8>,
    cursor: usize
}

impl InParser<Stdin> {
    pub fn from_stdin() -> InParser<Stdin> {
        InParser::new(std::io::stdin())
    }
}

impl InParser<File> {
    pub fn from_filename(name: &str) -> InParser<File> {
        InParser::new(File::open(name)
                      .expect("Failed to open file"))
    }
}

impl<T: Read> InParser<T> {
    pub fn new(reader: T) -> InParser<T> {
        let mut reader = BufReader::new(reader);

        let buffer = reader.fill_buf()
            .expect("Failed to fill buffer")
            .to_vec();
        
        InParser {
            reader,
            buffer,
            cursor: 0,
        }
    }

    pub fn get_current_byte(&mut self) -> Option<u8> {
        if self.cursor < self.buffer.len() {
            return Some(self.buffer[self.cursor]); 
        }
        return None
    }

    pub fn advance_cursor(&mut self) {
        self.cursor += 1;
        if self.cursor >= self.buffer.len() {
            self.reader.consume(self.buffer.len());
            self.buffer = self.reader.fill_buf()
                .expect("Failed to fill buffer")
                .to_vec();

            self.cursor = 0;
        }
    }

    fn skip_spaces(&mut self) {
        while self.get_current_byte() == Some(b' ') ||
              self.get_current_byte() == Some(b'\n') {
            
            self.advance_cursor();
        }
    }

    fn get_token(&mut self) -> Option<String> {
        let mut token_buf: Vec<u8> = Vec::new();

        self.skip_spaces();

        while self.get_current_byte() != None &&
            self.get_current_byte() != Some(b' ') &&
            self.get_current_byte() != Some(b'\n') {
            
            let byte = self.get_current_byte().unwrap();
            token_buf.push(byte);

            self.advance_cursor();
        }

        let strval = std::str::from_utf8(&token_buf)
            .expect("Failed to convert into valid utf8")
            .trim();
        
        if strval.is_empty() {
            return None;
        } else {
            Some(strval.to_string())
        }
    }
    
    pub fn read<F: FromStr>(&mut self) -> F
    where <F as FromStr>::Err: Debug{
        let token = self.get_token()
            .expect("Tried to read from empty token");

        token.parse::<F>()
            .unwrap()
    }
}

