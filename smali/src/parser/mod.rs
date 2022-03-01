mod class;
mod util;

use crate::err::*;
use crate::parser::class::parse_line_class;
use crate::smali_class::*;

pub fn parse_smali<'a>(
    lines: impl IntoIterator<Item = impl AsRef<str>>,
) -> ParserResult<SmaliClass<'a>> {
    for line in lines {
        match parse_line(line.as_ref())? {
            Line::Class(class) => {
                todo!();
            }
            _ => todo!(),
        }
    }
    todo!();
}

#[derive(Debug)]
enum Line<'a> {
    Class(SmaliClass<'a>),   // class declaration
    Super(&'a str),          // super class path
    Implements(&'a str),     // impl. interface path
    Value(SmaliValue<'a>),   // value declaration
    Method(SmaliMethod<'a>), // method head
    Other,
}

fn parse_line(line: &str) -> ParserResult<Line> {
    if line.starts_with(".class") {
        return Ok(Line::Class(parse_line_class(line)?));
    }

    todo!()
}
