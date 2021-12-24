use crate::smali::SmaliMethod;
use yaml_rust::{yaml::Hash, Yaml, YamlLoader};

#[derive(Debug)]
pub struct UbiIgnoreClass {
    pub name: String,
    pub ignore_methods: Option<Vec<SmaliMethod>>,
    pub ignore_super: Option<String>,
}

#[derive(Debug)]
pub struct UbiIgnore {
    pub ignore: Vec<UbiIgnoreClass>,
    pub ignore_not_found: Vec<String>,
}

pub fn parse_ubignore(file_content: String) -> Result<UbiIgnore, yaml_rust::ScanError> {
    let result = YamlLoader::load_from_str(&file_content)?;

    if result.len() == 0 {
        return Ok(UbiIgnore {
            ignore: vec![],
            ignore_not_found: vec![],
        });
    }

    if result.len() > 1 {
        warn!(".ubignore does not use yaml multi-doc, only the first doc will be used!");
    }

    let result = &result[0];

    let result = match result {
        Yaml::Hash(map) => map,
        _ => {
            error!(".ubignore's root is invalid");
            std::process::exit(1);
        }
    };

    Ok(UbiIgnore {
        ignore_not_found: parse_ignore_not_found(result),
        ignore: parse_ignore(result),
    })
}

fn parse_ignore(yaml: &Hash) -> Vec<UbiIgnoreClass> {
    let yaml = yaml.get(&Yaml::from_str("ignore"));
    if yaml.is_none() {
        return vec![];
    }
    let yaml = yaml.unwrap();

    match yaml {
        Yaml::Array(array) => array
            .into_iter()
            .map(move |element| match element {
                Yaml::Hash(map) => {
                    let name = map.get(&Yaml::from_str("name"));
                    if name.is_none() {
                        error!("name required");
                        std::process::exit(1);
                    }
                    let name = name.unwrap();
                    let name = match name {
                        Yaml::String(s) => s.clone(),
                        _ => {
                            error!("name must be a string");
                            std::process::exit(1);
                        }
                    };

                    let ignore_super =
                        map.get(&Yaml::from_str("ignore_super"))
                            .and_then(|ignore_super| match ignore_super {
                                Yaml::String(x) => Some(x.clone()),
                                _ => None,
                            });

                    UbiIgnoreClass {
                        name,
                        ignore_super,
                        ignore_methods: parse_methods(map),
                    }
                }
                _ => {
                    error!("ignore must be an array of objects, but sth else was found");
                    std::process::exit(1);
                }
            })
            .collect(),
        _ => {
            error!("ignore is not an array");
            std::process::exit(1);
        }
    }
}

fn parse_ignore_not_found(yaml: &Hash) -> Vec<String> {
    let yaml = yaml.get(&Yaml::from_str("ignore_not_found"));
    if yaml.is_none() {
        return vec![];
    }
    let yaml = yaml.unwrap();

    match yaml {
        Yaml::Array(array) => array
            .into_iter()
            .map(move |element| match element {
                Yaml::String(s) => s.clone(),
                _ => {
                    error!("ignore_not_found is not an array of strings");
                    std::process::exit(1);
                }
            })
            .collect(),
        _ => {
            error!("ignore_not_found is not an array of strings");
            std::process::exit(1);
        }
    }
}

fn parse_methods(map: &Hash) -> std::option::Option<Vec<SmaliMethod>> {
    let ignore_methods = map.get(&Yaml::from_str("ignore_methods"));
    if ignore_methods.is_none() {
        return None;
    }
    let ignore_methods = ignore_methods.unwrap();

    let arr = match ignore_methods {
        Yaml::Array(a) => a,
        _ => {
            error!("ignore_methods must be an array");
            std::process::exit(1);
        }
    };

    let map = arr.into_iter().map(|entry| parse_method(entry));

    return Some(map.collect());
}

fn parse_method(yaml: &Yaml) -> SmaliMethod {
    let obj = match yaml {
        Yaml::Hash(map) => map,
        _ => {
            error!("a method must be an object");
            std::process::exit(1);
        }
    };

    let name = obj.get(&Yaml::from_str("name"));
    if name.is_none() {
        error!("a method must have a name");
        std::process::exit(1);
    }
    let name = name.unwrap();
    let name = match name {
        Yaml::String(s) => s.clone(),
        _ => {
            error!("a method's name must be a string");
            std::process::exit(1);
        }
    };

    let return_type = obj.get(&Yaml::from_str("return_type"));
    if return_type.is_none() {
        error!("a method must have a return_type");
        std::process::exit(1);
    }
    let return_type = return_type.unwrap();
    let return_type = match return_type {
        Yaml::String(s) => s.clone(),
        _ => {
            error!("a method's return_type must be a string");
            std::process::exit(1);
        }
    };

    let fallback = vec![];
    let parameter_types = obj.get(&Yaml::from_str("parameter_types"));
    let parameter_types: &Vec<Yaml> = match parameter_types {
        None => &fallback,
        Some(v) => match v {
            Yaml::Array(array) => array,
            _ => {
                error!("a method's parameter_types must be an array of strings");
                std::process::exit(1);
            }
        },
    };

    let parameter_types = parameter_types
        .into_iter()
        .map(|s| match s {
            Yaml::String(string) => string.clone(),
            _ => {
                error!("a method's parameter_types must be an array of strings");
                std::process::exit(1);
            }
        })
        .collect();

    SmaliMethod {
        name,
        return_type,
        parameter_types,
    }
}
