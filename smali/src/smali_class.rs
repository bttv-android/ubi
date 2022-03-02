pub type SmaliType = String;

#[derive(Debug, PartialEq)]
pub enum SmaliAccessModifier {
    Public,
    Private,
    Protected,
    Package,
}
impl SmaliAccessModifier {
    pub fn from_str(token: &str) -> Option<Self> {
        match token {
            "public" => Some(SmaliAccessModifier::Public),
            "private" => Some(SmaliAccessModifier::Private),
            "protected" => Some(SmaliAccessModifier::Protected),
            _ => None,
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
    pub is_static: bool,
    pub is_final: bool,
}
