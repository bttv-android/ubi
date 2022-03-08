use crate::err::*;
use crate::parser::util::smali_to_java_path;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum SmaliAccessModifier {
    Public,
    Private,
    Protected,
    Package,
}
impl std::str::FromStr for SmaliAccessModifier {
    type Err = ();
    fn from_str(token: &str) -> Result<Self, Self::Err> {
        match token {
            "public" => Ok(SmaliAccessModifier::Public),
            "private" => Ok(SmaliAccessModifier::Private),
            "protected" => Ok(SmaliAccessModifier::Protected),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SmaliClass {
    // parsed from .class line
    pub class_path: String,
    pub access: SmaliAccessModifier,
    pub is_abstract: bool,

    // parsed from .super line
    pub super_path: Option<String>,

    // parsed from .implements lines
    pub interfaces: Vec<String>,
    pub values: Vec<SmaliValue>,
    pub methods: Vec<SmaliMethod>,
}
impl SmaliClass {
    pub fn new(class_path: String, access: SmaliAccessModifier, is_abstract: bool) -> Self {
        Self {
            class_path,
            super_path: None,
            access,
            interfaces: vec![],
            values: vec![],
            methods: vec![],
            is_abstract,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SmaliMethod {
    pub name: String,
    pub parameter_types: Vec<SmaliType>,
    pub return_type: SmaliType,
    pub is_static: bool,
    pub is_final: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SmaliValue {
    pub name: String,
    pub data_type: SmaliType,
    pub access: SmaliAccessModifier,
    pub is_static: bool,
    pub is_final: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SmaliType {
    Void,
    Boolean,
    Float,
    Double,
    Int,
    Long,
    Arr(Box<SmaliType>),
    Class(String),
}

impl FromStr for SmaliType {
    type Err = ParserError;
    fn from_str(token: &str) -> ParserResult<Self> {
        match token {
            "V" => Ok(Self::Void),
            "Z" => Ok(Self::Boolean),
            "F" => Ok(Self::Float),
            "D" => Ok(Self::Double),
            "I" => Ok(Self::Int),
            "J" => Ok(Self::Long),
            _ => {
                if let Some(rest) = token.strip_prefix('[') {
                    return Ok(Self::Arr(Box::new(Self::from_str(rest)?)));
                }
                Ok(Self::Class(smali_to_java_path(token)?))
            }
        }
    }
}

#[cfg(test)]
mod smali_type_tests {
    use super::*;

    #[test]
    fn void() {
        let input = "V";
        let expected = SmaliType::Void;
        let res = SmaliType::from_str(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected)
    }

    #[test]
    fn arr() {
        let input = "[Lbttv/test/Util;";
        let expected = SmaliType::Arr(Box::new(SmaliType::Class("bttv.test.Util".to_string())));
        let res = SmaliType::from_str(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected)
    }
}
