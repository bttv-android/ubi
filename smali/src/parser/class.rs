use crate::err::*;
use crate::parser::util::*;
use crate::smali_class::*;
use std::str::FromStr;

pub fn parse_line_class(line: &str) -> ParserResult<SmaliClass> {
    let tokens = line.split_whitespace();

    let mut class_path = None;
    let mut is_abstract = false;
    let mut access = SmaliAccessModifier::Package;

    for token in tokens {
        if token.starts_with('#') {
            break; // ignore comments
        }

        if token == "abstract" {
            is_abstract = true;
            continue;
        }

        if let Ok(access_modifier) = SmaliAccessModifier::from_str(token) {
            access = access_modifier;
            continue;
        }

        if token == ".class" || is_modifier(token) {
            continue;
        }

        class_path = Some(smali_to_java_path(token)?);
        break;
    }

    if class_path.is_none() {
        return Err(ParserError::MissingClassPath(line.to_string()));
    }

    let class = SmaliClass::new(class_path.unwrap(), access, is_abstract);
    Ok(class)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let line = ".class Lbttv/test/Util;";
        let expected = SmaliClass::new(
            "bttv.test.Util".to_string(),
            SmaliAccessModifier::Package,
            false,
        );
        assert_eq!(parse_line_class(line).unwrap(), expected);
    }

    #[test]
    fn test_abstract() {
        let line = ".class abstract Lbttv/test/Util;";
        let expected = SmaliClass::new(
            "bttv.test.Util".to_string(),
            SmaliAccessModifier::Package,
            true,
        );
        assert_eq!(parse_line_class(line).unwrap(), expected);
    }
}
