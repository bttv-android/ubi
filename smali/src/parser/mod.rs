mod class;
mod super_p;
mod util;

use crate::err::*;
use crate::parser::class::parse_line_class;
use crate::parser::super_p::parse_line_super;
use crate::smali_class::*;

pub fn parse_smali<'a>(
    lines: impl IntoIterator<Item = impl AsRef<str>>,
) -> ParserResult<SmaliClass> {
    let mut current_class = None;
    let mut super_path = None;

    for line in lines {
        match parse_line(line.as_ref())? {
            Line::Class(class) => {
                current_class = Some(class);
            }
            Line::Super(super_p) => {
                super_path = Some(super_p);
            }
            _ => todo!(),
        }
    }

    if current_class.is_none() {
        return Err(ParserError::MissingClass());
    }

    let mut current_class = current_class.unwrap();
    current_class.super_path = super_path;

    return Ok(current_class);
}

#[derive(Debug)]
enum Line {
    Class(SmaliClass),   // class declaration
    Super(String),       // super class path
    Implements(String),  // impl. interface path
    Value(SmaliValue),   // value declaration
    Method(SmaliMethod), // method head
    Other,
}

fn parse_line(line: &str) -> ParserResult<Line> {
    if line.starts_with(".class") {
        let class = parse_line_class(line)?;
        return Ok(Line::Class(class));
    } else if line.starts_with(".super") {
        let super_path = parse_line_super(line)?;
        return Ok(Line::Super(super_path));
    }

    todo!()
}
