#![allow(unused_variables)] // You might want to remove this
#![allow(dead_code)]

use std::io::{Read, BufRead, Stdin, BufReader};
use std::fs::File;
use std::str::FromStr;
use std::fmt::Debug;

pub struct InParser<T: Read> {
    reader: BufReader<T>,
    buffer: Vec<u8>,
    cursor: usize,
    eof_flag: bool,
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
            eof_flag: false,
        }
    }
    
    pub fn get_current_byte(&mut self) -> u8 {
        if self.cursor < self.buffer.len() {
            return self.buffer[self.cursor]; 
        }
        panic!("Outside of buffer")
    }

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
   
    pub fn read_string(&mut self) -> String {
        let token = self.get_token();
        
        let strval = std::str::from_utf8(&token)
            .expect("Failed to convert into valid utf8")
            .trim();

        strval.to_string()
    }
    
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

    pub fn read<F>(&mut self) -> F
    where F: FromStr,
          <F as FromStr>::Err: Debug {
        self.read_string().parse::<F>()
            .unwrap()
    }
}

use std::io::{Write, BufWriter, Stdout};

pub struct OutParser<T: Write> {
    writer: BufWriter<T>,
}

impl<T: Write> OutParser<T> {
    pub fn new(writer: T) -> OutParser<T> {
        OutParser {
            writer: BufWriter::new(writer)
        }
    }

    pub fn write<F: ToString>(&mut self, val: F) -> &mut Self {
        self.writer.write(&val.to_string().as_bytes())
            .expect("Failed to write");
        
        self
    }
}

impl OutParser<Stdout> {
    pub fn from_stdout() -> OutParser<Stdout> {
        OutParser::new(std::io::stdout())
    }
}

impl OutParser<File> {
    pub fn from_filename(name: &str) -> OutParser<File> {
        OutParser::new(File::create(name)
                       .expect("Failed to open file"))
    }
}

// Solve the task with the given input and print it to the given output.
fn solve_test<I: Read, O: Write>(fin: &mut InParser<I>, fout: &mut OutParser<O>) {
    // TODO: Put your solution here.
} 

// Run a sample with the given input and check with the correct output.
fn try_sample(input: &[u8], ok: &str) {
    use std::io::Cursor;

    let mut output = Vec::<u8>::new();

    solve_test(&mut InParser::new(BufReader::new(Cursor::new(input))),
               &mut OutParser::new(BufWriter::new(&mut output)));

    assert_eq!(std::str::from_utf8(&output).unwrap(), ok);
}

// Run the first sample of the problem.
fn sample_1() {
    try_sample(br#"1 2"#, r#"3"#);
}

fn main() {
    sample_1();
    
    solve_test(&mut InParser::from_filename("mex2d.in"),
               &mut OutParser::from_filename("mex2d.out"));
}


