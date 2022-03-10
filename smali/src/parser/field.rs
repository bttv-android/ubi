use crate::err::*;
use crate::parser::util::is_modifier;
use crate::smali_class::*;
use std::str::FromStr;

pub fn parse_line_field(line: &str) -> ParserResult<SmaliValue> {
    let tokens = line.split_ascii_whitespace();

    let mut is_static = false;
    let mut is_final = false;
    let mut access = SmaliAccessModifier::Package;

    for token in tokens {
        if token.starts_with('#') {
            break; // ignore comments
        }

        if token == "static" {
            is_static = true;
            continue;
        }

        if token == "final" {
            is_final = true;
            continue;
        }

        if let Ok(parsed_access) = SmaliAccessModifier::from_str(token) {
            access = parsed_access;
        }

        if token == ".field" || is_modifier(token) {
            continue;
        }

        let mut parts = token.splitn(2, ':');

        let name = get_next(&mut parts)?.to_string();
        let typ = SmaliType::from_str(get_next(&mut parts)?)?;

        return Ok(SmaliValue {
            name,
            data_type: typ,
            access,
            is_final,
            is_static,
        });
    }

    Err(ParserError::InvalidField())
}

fn get_next<'a>(split: &'a mut std::str::SplitN<char>) -> ParserResult<&'a str> {
    match split.next() {
        Some(next) => Ok(next),
        None => Err(ParserError::InvalidField()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let input =
            ".field public mUrlDrawable:Ltv/twitch/android/shared/ui/elements/span/UrlDrawable;";
        let expected = SmaliValue {
            name: "mUrlDrawable".to_string(),
            data_type: SmaliType::Class(
                "tv.twitch.android.shared.ui.elements.span.UrlDrawable".to_string(),
            ),
            access: SmaliAccessModifier::Public,
            is_static: false,
            is_final: false,
        };
        let res = parse_line_field(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected)
    }

    #[test]
    fn final_static() {
        let input = ".field private final static mUrlDrawable:I";
        let expected = SmaliValue {
            name: "mUrlDrawable".to_string(),
            data_type: SmaliType::Int,
            access: SmaliAccessModifier::Private,
            is_static: true,
            is_final: true,
        };
        let res = parse_line_field(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected)
    }
}
