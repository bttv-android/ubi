use crate::parser::ParserError;
use crate::ParserResult;
use parking_lot::Mutex;

pub fn is_access_modifier(token: &str) -> bool {
    matches!(token, "private" | "public" | "protected")
}

pub fn is_modifier(token: &str) -> bool {
    is_access_modifier(token)
        || matches!(
            token,
            "static" | "final" | "synthetic" | "constructor" | "enum" | "varargs" | "abstract"
        )
}

pub fn smali_to_java_path(input: &str) -> ParserResult<String> {
    let error = Err(ParserError::InvalidClassPath(input.to_string()));
    if input.len() < 2 {
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

    Ok(string)
}

/// Sets the value of a Mutex<Option<I>> and errors when I was not None
pub fn set_mutex_once_or_err<I>(
    mutex: &Mutex<Option<I>>,
    value: I,
    error: ParserError,
) -> ParserResult<()> {
    let mut mutex = mutex.lock();

    if mutex.is_some() {
        return Err(error);
    }
    *mutex = Some(value);
    Ok(())
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

    #[cfg(test)]
    mod test_set_mutex_once_or_err {
        use super::super::*;

        #[test]
        fn green() {
            let mutex = Mutex::new(None);
            let value = 5;
            let result = set_mutex_once_or_err(&mutex, value, ParserError::TooManyClasses());
            assert!(result.is_ok());
            assert_eq!(mutex.into_inner(), Some(value));
        }

        #[test]
        fn set_before() {
            let prev = Some(10);
            let mutex = Mutex::new(prev);
            let value = 5;
            let err = ParserError::TooManyClasses();
            let result = set_mutex_once_or_err(&mutex, value, err);
            assert!(result.is_err());
            assert!(matches!(result.unwrap_err(), ParserError::TooManyClasses()));
            assert_eq!(mutex.into_inner(), prev);
        }
    }
}
