//! An easy-to-use input parser special for competitive programming. Due to
//! the nature of competitive programming, when reading, any type of error
//! will make the program panic.
//!
//! ```
//! use std::io::{Cursor, BufReader};
//! # use competitive_rust::io::inparser::InParser;
//!
//! let reader = Cursor::new(b"1 2 asdf");
//! let mut reader = InParser::new(BufReader::new(reader));
//!
//! assert_eq!(reader.read::<i32>(), 1);
//!
//! let val: i32 = reader.read();
//! assert_eq!(val, 2);
//!
//! let val: String = reader.read();
//! assert_eq!(val, "asdf");
//! ```

#![allow(dead_code)]

use std::io::{Read, BufRead, Stdin, BufReader};
use std::fs::File;
use std::str::FromStr;
use std::fmt::Debug;

/// Parser used to read from the input.
pub struct InParser<T: Read> {
    reader: BufReader<T>,
    buffer: Vec<u8>,
    cursor: usize
}

impl InParser<Stdin> {
    /// Create a parser from stdin.
    pub fn from_stdin() -> InParser<Stdin> {
        InParser::new(std::io::stdin())
    }
}

impl InParser<File> {
    /// Create a parser from a file specified by the name of the file.
    pub fn from_filename(name: &str) -> InParser<File> {
        InParser::new(File::open(name)
                      .expect("Failed to open file"))
    }
}

impl<T: Read> InParser<T> {
    /// Create a parser from any object that implements [Read].
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

    /// Returns the byte at the current position of the cursor or [None] if the
    /// entire input has been consumed.
    pub fn get_current_byte(&mut self) -> Option<u8> {
        if self.cursor < self.buffer.len() {
            return Some(self.buffer[self.cursor]); 
        }
        return None
    }

    /// Advance the cursor to the next position.
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
    
    /// Read the next element from the input.
    pub fn read<F: FromStr>(&mut self) -> F
    where <F as FromStr>::Err: Debug{
        let token = self.get_token()
            .expect("Tried to read from empty token");

        token.parse::<F>()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_int() {
        use std::io::{Cursor, BufReader};

        let reader = Cursor::new(b"1 2");
        let mut reader = InParser::new(BufReader::new(reader));

        let x: i32 = reader.read();
        assert_eq!(x, 1);

        let y: i32 = reader.read();
        assert_eq!(y, 2);
    }

    #[test]
    fn lots_of_spaces() {
        use std::io::{Cursor, BufReader};

        let reader = Cursor::new(b"1 2");
        let mut reader = InParser::new(BufReader::new(reader));

        let x: i32 = reader.read();
        assert_eq!(x, 1);

        let y: i32 = reader.read();
        assert_eq!(y, 2);
    }
    
    #[test]
    fn read_string() {
        use std::io::{Cursor, BufReader};

        let reader = Cursor::new(b"asdf");
        let mut reader = InParser::new(BufReader::new(reader));

        let val: String = reader.read();

        assert_eq!(val, "asdf");
    }
    
    #[test]
    fn read_shuffled() {
        use std::io::{Cursor, BufReader};

        let reader = Cursor::new(b" 1     asdf    2");
        let mut reader = InParser::new(BufReader::new(reader));

        assert_eq!(reader.read::<i32>(), 1);
        assert_eq!(reader.read::<String>(), "asdf");
        assert_eq!(reader.read::<i32>(), 2);
    }
}

