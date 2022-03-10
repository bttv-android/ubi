mod class;
mod field;
mod implements;
mod method;
mod super_p;
pub mod util;

use crate::err::*;
use crate::smali_class::*;
use crossbeam_queue::SegQueue;
use rayon::iter::ParallelIterator;
use std::sync::Mutex;
use util::set_mutex_once_or_err;

const ERR_TOO_MANY_CLASSES: ParserError = ParserError::TooManyClasses();
const ERR_TOO_MANY_SUPERS: ParserError = ParserError::TooManySupers();

pub fn parse_smali(
    lines: impl ParallelIterator<Item = impl AsRef<str> + Send> + Send,
) -> ParserResult<SmaliClass> {
    let current_class = Mutex::new(None);
    let super_path = Mutex::new(None);
    let interfaces = SegQueue::new();
    let values = SegQueue::new();

    let res: ParserResult<()> = lines
        .map(|line| parse_line(line.as_ref()))
        .try_for_each(|line| {
            match line? {
                Line::Class(class) => {
                    set_mutex_once_or_err(&current_class, class, ERR_TOO_MANY_CLASSES)?;
                }
                Line::Super(super_p) => {
                    set_mutex_once_or_err(&super_path, super_p, ERR_TOO_MANY_SUPERS)?;
                }
                Line::Implements(interface_path) => {
                    interfaces.push(interface_path);
                }
                Line::Value(value) => {
                    values.push(value);
                }
                _ => todo!(),
            }
            Ok(())
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

    current_class.interfaces = interfaces.into_iter().collect();
    current_class.values = values.into_iter().collect();

    Ok(current_class)
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
        let class = class::parse_line(line)?;
        return Ok(Line::Class(class));
    } else if line.starts_with(".super") {
        let super_path = super_p::parse_line(line)?;
        return Ok(Line::Super(super_path));
    } else if line.starts_with(".implements") {
        let interface = implements::parse_line(line)?;
        return Ok(Line::Implements(interface));
    } else if line.starts_with(".field") {
        let field = field::parse_line(line)?;
        return Ok(Line::Value(field));
    }

    todo!()
}
