use crate::parser::ParserError;
use crate::ParserResult;

pub fn is_access_modifier(token: &str) -> bool {
    match token {
        "private" | "public" | "protected" => true,
        _ => false,
    }
}

pub fn is_modifier(token: &str) -> bool {
    if is_access_modifier(token) {
        return true;
    }
    match token {
        "static" | "final" | "synthetic" | "constructor" | "enum" | "varargs" | "abstract" => true,
        _ => false,
    }
}

pub fn smali_to_java_path(input: &str) -> ParserResult<String> {
    let error = Err(ParserError::InvalidClassPath(input.to_string()));
    if input.len() == 0 {
        return error;
    }

    let mut string = String::with_capacity(input.len() - 2);

    for (i, ch) in input.chars().enumerate() {
        if i == 0 {
            if ch != 'L' {
                return error;
            }
            continue;
        }

        if i == input.len() - 1 {
            if ch != ';' {
                return error;
            }
            break;
        }

        if ch == '/' {
            string.push('.');
        } else {
            string.push(ch);
        }
    }

    return Ok(string);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::err::*;

    #[test]
    fn test_smali_to_java_path() {
        assert_eq!(
            smali_to_java_path("Lbttv/test/Util;").unwrap(),
            "bttv.test.Util".to_string()
        );
        let s = "bttv/test/Util;";
        match smali_to_java_path(s).unwrap_err() {
            ParserError::InvalidClassPath(token) => {
                assert_eq!(token, s)
            }
            _ => panic!(),
        }
        let s = "Lbttv/test/Util";
        match smali_to_java_path(s).unwrap_err() {
            ParserError::InvalidClassPath(token) => {
                assert_eq!(token, s)
            }
            _ => panic!(),
        }
        let s = "";
        match smali_to_java_path(s).unwrap_err() {
            ParserError::InvalidClassPath(token) => {
                assert_eq!(token, s)
            }
            _ => panic!(),
        }
    }
}
