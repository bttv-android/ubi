use super::util::smali_to_java_path;
use crate::err::*;

pub fn parse_line_implements(line: &str) -> ParserResult<String> {
    let tokens = line.split_whitespace();
    for token in tokens {
        if token.starts_with("#") {
            break; // ignore comments
        }

        if token == ".implements" {
            continue;
        }

        return Ok(smali_to_java_path(token)?);
    }
    Err(ParserError::MissingInterfacePath(line.to_string()))
}
