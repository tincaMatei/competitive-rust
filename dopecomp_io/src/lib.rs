//! Read and write utilities for competitive programming.
//!
//! ```
//! use std::io::{Cursor, BufReader};
//! # use dopecomp_io::InParser;
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

use std::io::{Read, BufRead, Stdin, BufReader, Write, BufWriter, Stdout};
use std::fs::File;
use std::str::FromStr;
use std::fmt::Debug;

/// Parser used for reading from stdin, files, or any other source.
///
/// ```no_run
/// # use dopecomp_io::InParser;
/// #
/// // stdin:
/// //       1      2   asdf
///
/// let mut reader = InParser::from_stdin();
///
/// assert_eq!(reader.read::<i32>(), 1);
///
/// let val: i32 = reader.read();
/// assert_eq!(val, 2);
///
/// let val: String = reader.read();
/// assert_eq!(val, "asdf");
/// ```
pub struct InParser<T: Read> {
    reader: BufReader<T>,
    buffer: Vec<u8>,
    cursor: usize,
    eof_flag: bool,
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
            eof_flag: false,
        }
    }

    /// Returns the byte at the current position of the cursor or [None] if the
    /// entire input has been consumed.
    pub fn get_current_byte(&mut self) -> u8 {
        if self.cursor < self.buffer.len() {
            return self.buffer[self.cursor]; 
        }
        panic!("Outside of buffer")
    }

    /// Advance the cursor to the next position.
    pub fn advance_cursor(&mut self) {
        self.cursor += 1;
        if self.cursor >= self.buffer.len() {
            self.reader.consume(self.buffer.len());
            self.buffer = self.reader.fill_buf()
                .expect("Failed to fill buffer")
                .to_vec();

            self.eof_flag = self.buffer.is_empty();
            self.cursor = 0;
        }
    }

    fn skip_spaces(&mut self) {
        while !self.eof_flag && 
            (self.get_current_byte() == b' ' ||
             self.get_current_byte() == b'\n') {
            
            self.advance_cursor();
        }
    }

    fn get_token(&mut self) -> Vec<u8> {
        let mut token_buf: Vec<u8> = Vec::new();

        self.skip_spaces();

        while !self.eof_flag &&
            self.get_current_byte() != b' ' &&
            self.get_current_byte() != b'\n' {
            
            let byte = self.get_current_byte();
            token_buf.push(byte);

            self.advance_cursor();
        }

        token_buf
    }
   
    /// Read a string from the input.
    pub fn read_string(&mut self) -> String {
        let token = self.get_token();
        
        let strval = std::str::from_utf8(&token)
            .expect("Failed to convert into valid utf8")
            .trim();

        strval.to_string()
    }
    
    /// Read an integer number from the input.
    pub fn read_number<F: From<i64>>(&mut self) -> F {
        self.skip_spaces();

        let sgn = if self.get_current_byte() == b'-' {
            self.advance_cursor();
            -1
        } else {
            1
        };
        let mut nr = 0;

        while !self.eof_flag && self.get_current_byte().is_ascii_digit() {
            nr = nr * 10 + (self.get_current_byte() - b'0') as i64;
            self.advance_cursor();
        }

        F::from(nr * sgn)
    }

    /// Read an element from the input that can be parsed. 
    ///
    /// It is preferred to use
    /// [InParser::read_number] instead of this function, because the latter is optimized.
    ///
    /// When reading a string, use [InParser::read_string] instead, because there is no parsing
    /// needed to convert to the required type.
    pub fn read<F>(&mut self) -> F
    where F: FromStr,
          <F as FromStr>::Err: Debug {
        self.read_string().parse::<F>()
            .unwrap()
    }
}

/// Writer used for writing in stdout, a file, or any other place.
/// 
/// ```no_run
/// # use dopecomp_io::OutParser;
/// #
/// let mut writer = OutParser::from_stdout();
///
/// let x: i32 = 2;
/// let y: i32 = 3;
///
/// writer.write(x)
///     .write("\n")
///     .write(format!("Sum: {}\n", x + y));
///
/// // stdout:
/// // 2
/// // 5
///
/// ```
pub struct OutParser<T: Write> {
    writer: BufWriter<T>,
}

impl<T: Write> OutParser<T> {
    /// Create a writer from any item that implements [Write]
    pub fn new(writer: T) -> OutParser<T> {
        OutParser {
            writer: BufWriter::new(writer)
        }
    }

    /// Write a value to the target.
    pub fn write<F: ToString>(&mut self, val: F) -> &mut Self {
        self.writer.write(&val.to_string().as_bytes())
            .expect("Failed to write");
        
        self
    }
}

impl OutParser<Stdout> {
    /// Create a writer from stdout.
    pub fn from_stdout() -> OutParser<Stdout> {
        OutParser::new(std::io::stdout())
    }
}

impl OutParser<File> {
    /// Create a writer from a file at the given path.
    pub fn from_filename(name: &str) -> OutParser<File> {
        OutParser::new(File::create(name)
                       .expect("Failed to open file"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_normal_int() {
        use std::io::{Cursor, BufReader};

        let reader = Cursor::new(b"1 2");
        let mut reader = InParser::new(BufReader::new(reader));

        let x: i32 = reader.read();
        assert_eq!(x, 1);

        let y: i32 = reader.read();
        assert_eq!(y, 2);
    }

    #[test]
    fn read_lots_of_spaces() {
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

    #[test]
    fn read_negative() {
        use std::io::{Cursor, BufReader};

        let reader = Cursor::new(b" -1    -2   ");
        let mut reader = InParser::new(BufReader::new(reader));

        assert_eq!(reader.read::<i32>(), -1);
        assert_eq!(reader.read::<i32>(), -2);
    }

    #[test]
    fn write_simple() {
        let mut bytes = Vec::<u8>::new();

        {
            let mut writer = OutParser::new(&mut bytes);

            writer.write("Hello, world!\n");
        }

        assert_eq!(&mut bytes, b"Hello, world!\n");
    }

    #[test]
    fn write_chained() {
        let mut bytes = Vec::<u8>::new();

        let x = 2;
        let y = 3;

        {
            let mut writer = OutParser::new(&mut bytes);

            writer.write(x)
                .write(" ")
                .write(y)
                .write("\n")
                .write(format!("Sum: {}\n", x + y));
        }

        assert_eq!(&mut bytes, b"2 3\nSum: 5\n");
    }

    #[test]
    fn test_floats() {
        let mut bytes = Vec::<u8>::new();

        let pi = f64::acos(-1.0);

        {
            let mut writer = OutParser::new(&mut bytes);

            writer.write(format!("{:.10}", pi));
        }

        assert_eq!(&mut bytes, b"3.1415926536");
    }
}



