extern crate thiserror;

mod err;
mod parser;
mod smali_class;

pub use smali_class::*;

use err::ParserResult;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn parse_file(file_path: &str) -> ParserResult<SmaliClass> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter(|l| l.is_ok()).map(|l| l.unwrap());

    parser::parse_smali(lines)
}

pub fn parse_class<'a>(class_string: &'a str) -> ParserResult<SmaliClass<'a>> {
    parser::parse_smali(class_string.lines())
}
