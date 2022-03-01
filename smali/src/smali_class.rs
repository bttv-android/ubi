pub type SmaliType<'a> = &'a str;

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
pub struct SmaliClass<'a> {
    pub class_path: String,
    pub super_path: Option<&'a str>,
    pub access: SmaliAccessModifier,
    pub interfaces: Vec<&'a str>,
    pub is_abstract: bool,
    pub values: Vec<SmaliValue<'a>>,
    pub methods: Vec<SmaliMethod<'a>>,
}
impl<'a> SmaliClass<'a> {
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
pub struct SmaliMethod<'a> {
    pub name: &'a str,
    pub parameter_types: Vec<SmaliType<'a>>,
    pub return_type: SmaliType<'a>,
    pub is_static: bool,
    pub is_final: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SmaliValue<'a> {
    pub name: &'a str,
    pub data_type: SmaliType<'a>,
    pub is_static: bool,
    pub is_final: bool,
}
