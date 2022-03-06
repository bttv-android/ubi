pub type SmaliType = String;

#[derive(Debug, PartialEq)]
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
    pub is_static: bool,
    pub is_final: bool,
}
