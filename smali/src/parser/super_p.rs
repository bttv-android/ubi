use crate::err::*;
use crate::parser::util::smali_to_java_path;

pub fn parse_line_super(line: &str) -> ParserResult<String> {
    let tokens = line.split_whitespace();

    let mut super_path = None;

    for token in tokens {
        if token.starts_with('#') {
            break; // ignore comments
        }

        if token == ".super" {
            continue;
        }

        super_path = Some(smali_to_java_path(token)?);
        break;
    }

    if super_path.is_none() {
        return Err(ParserError::MissingSuperPath(line.to_string()));
    }

    Ok(super_path.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let input = ".super Lbttv/test/Util;";
        let expected = "bttv.test.Util";
        assert_eq!(parse_line_super(input).unwrap(), expected);
    }

    #[test]
    fn test_valid_with_comment() {
        let input = ".super Lbttv/test/Util; #Just ignore me";
        let expected = "bttv.test.Util";
        assert_eq!(parse_line_super(input).unwrap(), expected);
    }

    #[test]
    fn test_invalid() {
        let input = ".super bttv test Util";
        assert!(parse_line_super(input).is_err());
    }
}
