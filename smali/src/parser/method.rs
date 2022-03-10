use super::util::is_modifier;
use crate::err::*;
use crate::smali_class::*;
use std::str::FromStr;

pub fn parse_line(line: &str) -> ParserResult<SmaliMethod> {
    let tokens = line.split_ascii_whitespace();

    let mut name = None;
    let mut params = None;
    let mut return_type = None;
    let mut is_final = false;
    let mut is_static = false;
    let mut access = SmaliAccessModifier::Package;

    for token in tokens {
        if token.starts_with("#") {
            break; // ignore comments
        }

        if token == "static" {
            is_static = true;
        }

        if token == "final" {
            is_final = true;
        }

        if let Ok(access_modifier) = SmaliAccessModifier::from_str(token) {
            access = access_modifier;
        }

        if token.starts_with(".method") || is_modifier(token) {
            continue;
        }

        let parse_result = parse_method(token)?;

        name = parse_result.0;
        params = parse_result.1;
        return_type = parse_result.2;
    }

    if name.is_none() || params.is_none() || return_type.is_none() {
        return Err(ParserError::InvalidMethod());
    }

    let method = SmaliMethod {
        name: name.unwrap(),
        parameter_types: params.unwrap(),
        return_type: return_type.unwrap(),
        is_static,
        is_final,
        access,
    };

    Ok(method)
}

/// returns name, params and return type in that order
fn parse_method(
    token: &str,
) -> ParserResult<(Option<String>, Option<Vec<SmaliType>>, Option<SmaliType>)> {
    let (name, token) = token.split_once('(').ok_or(ParserError::InvalidMethod())?;

    let (params, token) = parse_type_stream(token)?;

    let return_t = SmaliType::from_str(token)?;

    Ok((Some(name.to_string()), Some(params), Some(return_t)))
}

/// expects a stream of smali types in a &str and parses them, returns once it sees an invalid char
fn parse_type_stream(stream: &str) -> ParserResult<(Vec<SmaliType>, &str)> {
    let mut char_buffer = [0; 4];

    let mut vec = vec![];

    let mut started_collecting_class_at = None;
    let mut is_array = false;

    let mut last_i = 0;

    for (i, char) in stream.char_indices() {
        if let Some(started) = started_collecting_class_at {
            if char == ';' {
                let str_to_parse = &stream[started..=i];
                let parsed = SmaliType::from_str(str_to_parse)?;
                vec.push(maybe_wrap_in_arr(parsed, is_array));
                is_array = false;
                started_collecting_class_at = None;
            }
            continue;
        }

        if char == 'L' {
            started_collecting_class_at = Some(i);
            continue;
        }

        if char == '[' {
            is_array = true;
            continue;
        }

        let parsed = SmaliType::from_str(char.encode_utf8(&mut char_buffer));
        if parsed.is_err() {
            last_i = i;
            break;
        }
        let parsed = parsed.unwrap();
        vec.push(maybe_wrap_in_arr(parsed, is_array));
        is_array = false;
    }

    Ok((vec, &stream[last_i + 1..]))
}

fn maybe_wrap_in_arr(type_p: SmaliType, should_wrap: bool) -> SmaliType {
    if !should_wrap {
        return type_p;
    }
    return SmaliType::Arr(Box::new(type_p));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod parse_method_tests {
        use super::*;

        #[test]
        fn no_param_complex_return() {
            let input =
                "$values()[Ltv/twitch/android/api/resumewatching/ResumeWatchingApi$VideoType;";
            let res = parse_method(input);
            let (name, params, return_t) = res.unwrap();
            assert_eq!(name.unwrap(), "$values".to_string());
            assert!(params.unwrap().is_empty());
            assert_eq!(
                return_t.unwrap(),
                SmaliType::Arr(Box::new(SmaliType::Class(
                    "tv.twitch.android.api.resumewatching.ResumeWatchingApi$VideoType".to_string()
                )))
            );
        }
    }

    #[cfg(test)]
    mod parse_type_stream {
        use super::*;
        #[test]
        fn simple() {
            let input = "VZFDIJ)test";
            let expected = vec![
                SmaliType::Void,
                SmaliType::Boolean,
                SmaliType::Float,
                SmaliType::Double,
                SmaliType::Int,
                SmaliType::Long,
            ];

            let res = parse_type_stream(input).unwrap();
            assert_eq!(res.0, expected);
            assert_eq!(res.1, "test");
        }

        #[test]
        fn simple_arr() {
            let input = "[VZF[DIJ)test";
            let expected = vec![
                SmaliType::Arr(Box::new(SmaliType::Void)),
                SmaliType::Boolean,
                SmaliType::Float,
                SmaliType::Arr(Box::new(SmaliType::Double)),
                SmaliType::Int,
                SmaliType::Long,
            ];

            let res = parse_type_stream(input).unwrap();
            assert_eq!(res.0, expected);
            assert_eq!(res.1, "test");
        }

        #[test]
        fn complex() {
            let input = "[Ltest/test/Test;VZF[DIJLtest/test/Test;)test";
            let expected = vec![
                SmaliType::Arr(Box::new(SmaliType::Class("test.test.Test".to_string()))),
                SmaliType::Void,
                SmaliType::Boolean,
                SmaliType::Float,
                SmaliType::Arr(Box::new(SmaliType::Double)),
                SmaliType::Int,
                SmaliType::Long,
                SmaliType::Class("test.test.Test".to_string()),
            ];

            let res = parse_type_stream(input).unwrap();
            assert_eq!(res.0, expected);
            assert_eq!(res.1, "test");
        }
    }
}
