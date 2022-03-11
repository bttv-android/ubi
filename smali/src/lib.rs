//! smali is a crate that parses the metadata of smali classes
//! to be used in the ubi cli tool

extern crate thiserror;

mod err;
mod parser;
mod smali_class;

pub use smali_class::*;

use err::ParserResult;
use rayon::prelude::ParallelBridge;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/// Given a file_path `parse_file` reads the file and parses it's content into a SmaliClass
pub fn parse_file(file_path: impl AsRef<std::path::Path>) -> ParserResult<SmaliClass> {
    let file = File::open(file_path.as_ref())?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter(|l| l.is_ok()).map(Result::unwrap);

    parser::parse_smali(lines.par_bridge())
}

/// Parses a smali class (in form of a String or alike) into a SmaliClass
pub fn parse_class(class_string: &str) -> ParserResult<SmaliClass> {
    parser::parse_smali(class_string.lines().par_bridge())
}
