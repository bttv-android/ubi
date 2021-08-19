use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
pub struct SmaliClass {
    path: String,
    super_path: Option<String>,
    implements: Vec<String>,
    values: Vec<SmaliValue>,
    methods: Vec<SmaliMethod>,
}

impl SmaliClass {
    fn new(path: String) -> Self {
        Self {
            path,
            super_path: None,
            implements: vec![],
            values: vec![],
            methods: vec![],
        }
    }
}

#[derive(Debug)]
struct SmaliMethod {
    name: String,
    parameter_types: Vec<String>,
    return_type: String,
}

#[derive(Debug)]
struct SmaliValue {
    name: String,
    data_type: String,
}

#[derive(Debug)]
enum SmaliLine {
    Class(SmaliClass),   // class declaration
    Super(String),       // super class path
    Implements(String),  // impl. interface path
    Value(SmaliValue),   // value declaration
    Method(SmaliMethod), // method head
    Other,
}

pub fn parse_file(path: &std::path::PathBuf) -> Result<Option<SmaliClass>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter(|l| l.is_ok()).map(|l| l.unwrap());

    let path = path.to_str().unwrap();

    let mut current_class = None;

    for line in lines {
        let parsed = parse_line(&line);

        match parsed {
            SmaliLine::Class(class) => {
                if current_class.is_some() {
                    error!("this parser assumes only one class per file, but found two for file {}\noffending line: {}", path, line);
                    continue;
                }
                current_class = Some(class);
            }
            SmaliLine::Super(super_path) => {
                if current_class.is_none() {
                    error!(
                        "super declaration came before class declaration, line: {}",
                        line
                    );
                    continue;
                }
                let mut class = current_class.unwrap();
                class.super_path = Some(super_path);
                current_class = Some(class);
            }
            SmaliLine::Implements(impl_path) => {
                if current_class.is_none() {
                    error!(
                        "implements declaration came before class declaration, line: {}",
                        line
                    );
                    continue;
                }
                let mut class = current_class.unwrap();
                class.implements.push(impl_path);
                current_class = Some(class);
            }
            SmaliLine::Value(value) => {
                if current_class.is_none() {
                    error!(
                        "value declaration came before class declaration, line: {}",
                        line
                    );
                    continue;
                }
                let mut class = current_class.unwrap();
                class.values.push(value);
                current_class = Some(class);
            }
            SmaliLine::Method(method) => {
                if current_class.is_none() {
                    error!(
                        "method declaration came before class declaration, line: {}",
                        line
                    );
                    continue;
                }
                let mut class = current_class.unwrap();
                class.methods.push(method);
                current_class = Some(class);
            }
            _ => {}
        }
    }

    return Ok(current_class);
}

fn parse_line(line: &String) -> SmaliLine {
    let line = line.trim();
    if line.starts_with(".class") {
        return parse_line_class(line);
    } else if line.starts_with(".super") {
        return parse_line_super(line);
    } else if line.starts_with(".implements") {
        return parse_line_implements(line);
    } else if line.starts_with(".field") {
        return parse_line_field(line);
    } else if line.starts_with(".method") {
        return parse_line_method(line);
    }

    return SmaliLine::Other;
}

fn parse_line_class(line: &str) -> SmaliLine {
    let tokens = line.split_whitespace();

    let mut path = None;
    for token in tokens {
        if token.starts_with("#") {
            break; // ignore comments
        }
        if token.starts_with(".") || is_modifier(token) {
            continue;
        }
        path = smali_to_java_path(token);
        if path.is_none() {
            error!("class path seems invalid");
        }
        break;
    }
    if path.is_none() {
        error!("class path could not be extracted for line: '{}'", line);
        path = Some("unknown".to_string());
    }

    let class = SmaliClass::new(path.unwrap());
    return SmaliLine::Class(class);
}

fn parse_line_super(line: &str) -> SmaliLine {
    let tokens = line.split_whitespace();

    let mut path = None;
    for token in tokens {
        if token.starts_with("#") {
            break; // ignore comments
        }
        if token.starts_with(".") {
            continue;
        }
        path = smali_to_java_path(token);
        if path.is_none() {
            error!("super class path seems invalid");
        }
        break;
    }
    if path.is_none() {
        error!(
            "super class path could not be extracted for line: '{}'",
            line
        );
        path = Some("unknown".to_string());
    }
    return SmaliLine::Super(path.unwrap());
}

fn parse_line_implements(line: &str) -> SmaliLine {
    let tokens = line.split_whitespace();

    let mut path = None;
    for token in tokens {
        if token.starts_with("#") {
            break; // ignore comments
        }
        if token.starts_with(".") {
            continue;
        }
        path = smali_to_java_path(token);
        if path.is_none() {
            error!("implements path seems invalid");
        }
        break;
    }
    if path.is_none() {
        error!(
            "implements path could not be extracted for line: '{}'",
            line
        );
        path = Some("unknown".to_string());
    }
    return SmaliLine::Implements(path.unwrap());
}

fn parse_line_field(line: &str) -> SmaliLine {
    let tokens = line.split_whitespace();

    let mut name = None;
    let mut typ = None;

    for token in tokens {
        if token.starts_with("#") {
            break; // ignore comments
        }
        if token.starts_with(".") || is_modifier(token) {
            continue;
        }
        let (n, t) = parse_field(token);
        name = n;
        typ = t;
        if typ.is_none() {
            error!("type of this field seems invalid: {}", line);
        }
    }
    if name.is_none() || typ.is_none() {
        error!("field could not be extracted for line: '{}'", line);
        return SmaliLine::Other;
    }
    let value = SmaliValue {
        name: name.unwrap(),
        data_type: typ.unwrap(),
    };
    return SmaliLine::Value(value);
}

fn parse_line_method(line: &str) -> SmaliLine {
    let tokens = line.split_whitespace();

    let mut name = None;
    let mut params = None;
    let mut return_type = None;

    for token in tokens {
        if token.starts_with("#") {
            break; // ignore comments
        }
        if token.starts_with(".") || is_modifier(token) {
            continue;
        }
        let (n, p, r) = parse_method(token);
        name = n;
        params = Some(p);
        return_type = r;
    }

    if name.is_none() || params.is_none() || return_type.is_none() {
        error!("method declaration is invalid: {}", line);
        return SmaliLine::Other;
    }

    let method = SmaliMethod {
        name: name.unwrap().to_string(),
        parameter_types: params.unwrap(),
        return_type: return_type.unwrap(),
    };
    return SmaliLine::Method(method);
}

fn is_access_modifier(token: &str) -> bool {
    return match token {
        "public" | "private" | "protected" | "static" | "final" | "synthetic" | "enum" => true,
        _ => false,
    };
}

/** Things like "private", "static" or "final" */
fn is_modifier(token: &str) -> bool {
    if is_access_modifier(token) {
        return true;
    }
    match token {
        "static" | "final" | "constructor" => true,
        _ => false,
    }
}

fn smali_to_java_path(token: &str) -> Option<String> {
    if token.len() < 2 {
        return None;
    }
    let first_last_off: &str = &token[1..token.len() - 1];
    return Some(first_last_off.replace("/", "."));
}

fn parse_field(token: &str) -> (Option<String>, Option<String>) {
    let parts = token.splitn(2, ":");

    let mut name = None;
    let mut typ = None;

    for part in parts {
        if name.is_none() {
            name = Some(part.to_string());
        } else {
            typ = Some(part.to_string());
        }
    }

    typ = typ.map(|v| parse_data_type(&v)).flatten();

    return (name, typ);
}

fn parse_data_type(token: &str) -> Option<String> {
    match token {
        "V" => Some("void".to_string()),
        "Z" => Some("boolean".to_string()),
        "F" => Some("float".to_string()),
        "I" => Some("int".to_string()),
        _ => smali_to_java_path(token),
    }
}

fn parse_method(token: &str) -> (Option<&str>, Vec<String>, Option<String>) {
    enum FindingMode {
        Name,
        Params,
        ParamsLong,
        Return,
        ReturnLong,
    }
    let mut mode = FindingMode::Name;

    let mut name = None;
    let mut params = vec![];
    let mut return_type = None;

    let mut long_start = 0;

    for (i, char) in token.chars().enumerate() {
        match &mode {
            FindingMode::Name => {
                if char == '(' {
                    name = Some(&token[0..i]);
                    mode = FindingMode::Params;
                }
            }
            FindingMode::Params => {
                if char == 'L' {
                    long_start = i;
                    mode = FindingMode::ParamsLong;
                } else if char == ')' {
                    mode = FindingMode::Return;
                } else {
                    let mut parsed = parse_data_type(&token[i..=i]);
                    if parsed.is_none() {
                        error!("failed to parse parameter type: {}", &token[i..=i]);
                        parsed = Some("???".to_string());
                    }
                    params.push(parsed.unwrap());
                }
            }
            FindingMode::ParamsLong | FindingMode::ReturnLong => {
                if char == ';' {
                    let mut parsed = parse_data_type(&token[long_start..=i]);
                    if parsed.is_none() {
                        error!("failed to parse type: {}", &token[long_start..=i]);
                        parsed = Some("???".to_string());
                    }
                    match &mode {
                        FindingMode::ParamsLong => {
                            params.push(parsed.unwrap());
                            mode = FindingMode::Params;
                        }
                        FindingMode::ReturnLong => {
                            return_type = parsed;
                        }
                        _ => unreachable!(),
                    }
                }
            }
            FindingMode::Return => {
                if char == 'L' {
                    mode = FindingMode::ReturnLong;
                    long_start = i;
                } else {
                    let parsed = parse_data_type(&token[i..=i]);
                    return_type = parsed;
                }
            }
        }
    }

    return (name, params, return_type);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_class() {
        let res = parse_line(
            &".class public final Ltv/twitch/android/preferences/BooleanDelegate;".to_string(),
        );
        let class = match res {
            SmaliLine::Class(class) => class,
            _ => panic!("not a Class"),
        };
        assert_eq!(class.path, "tv.twitch.android.preferences.BooleanDelegate");
    }
    #[test]
    fn parse_line_super() {
        let res = parse_line(&".super Ltv/twitch/android/preferences/BooleanDelegate;".to_string());
        let path = match res {
            SmaliLine::Super(path) => path,
            _ => panic!("not a Super"),
        };
        assert_eq!(path, "tv.twitch.android.preferences.BooleanDelegate");
    }

    #[test]
    fn parse_line_implements() {
        let res =
            parse_line(&".implements Ltv/twitch/android/preferences/BooleanDelegate;".to_string());
        let path = match res {
            SmaliLine::Implements(path) => path,
            _ => panic!("not an Implements"),
        };
        assert_eq!(path, "tv.twitch.android.preferences.BooleanDelegate");
    }

    #[test]
    fn parse_line_field() {
        let res = parse_line(&".field private final test:Z".to_string());
        let value = match res {
            SmaliLine::Value(value) => value,
            _ => panic!("not a Value"),
        };
        assert_eq!(value.name, "test");
        assert_eq!(value.data_type, "boolean");
    }
}
