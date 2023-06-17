#![allow(dead_code)]

use std::io::{Write, BufWriter, Stdout};
use std::fs::File;

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

