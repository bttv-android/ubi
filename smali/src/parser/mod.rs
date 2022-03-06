mod class;
mod super_p;
mod util;

use std::sync::Mutex;
use crate::err::*;
use crate::parser::class::parse_line_class;
use crate::parser::super_p::parse_line_super;
use crate::smali_class::*;
use rayon::iter::ParallelIterator;

pub fn parse_smali<'a>(
    lines: impl ParallelIterator<Item = impl AsRef<str> + Send> + Send,
) -> ParserResult<SmaliClass> {
    let current_class = Mutex::new(None);
    let mut super_path = Mutex::new(None);

    let res: ParserResult<()> = lines
        .map(|line| parse_line(line.as_ref()))
        .try_for_each(|line| {
            Ok(match line? {
                Line::Class(class) => {

                    // unwrap: other threads holding this lock can only panic in this line, thus the lock never gets poisoned
                    let mut current_class = current_class.lock().unwrap();

                    if let Some(_) = *current_class {
                        return Err(ParserError::TooManyClasses());
                    }
                    *current_class = Some(class);
                }
                Line::Super(super_p) => {

                    // unwrap: other threads holding this lock can only panic in this line, thus the lock never gets poisoned
                    let mut super_path = super_path.lock().unwrap();

                    if let Some(_) = *super_path {
                        return Err(ParserError::TooManySupers());
                    }
                    *super_path = Some(super_p);
                }
                _ => todo!(),
            })
        });

    if let Err(err) = res {
        return Err(err);
    }

    // unwrap: the other threads have died after proccessing, so we are the only thread with access to the Mutex
    let current_class = current_class.into_inner().unwrap();

    if current_class.is_none() {
        return Err(ParserError::MissingClass());
    }

    // unwrap: guarded by if above
    let mut current_class = current_class.unwrap();

    // unwrap: the other threads have died after proccessing, so we are the only thread with access to the Mutex
    current_class.super_path = super_path.into_inner().unwrap();

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
