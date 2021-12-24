use crate::args::UbiArgs;
use crate::smali::{SmaliClass, SmaliMethod, SmaliValue};

pub fn print_diff(
    args: &UbiArgs,
    rel: String,
    smali_mod: SmaliClass,
    smali_disass: SmaliClass,
) -> bool {
    let diff = gen_diff(args, rel, smali_mod, smali_disass);
    if diff.is_different {
        warn!("{:#?}", diff);
        return true;
    }
    return false;
}

fn gen_diff(
    args: &UbiArgs,
    rel: String,
    smali_mod: SmaliClass,
    smali_disass: SmaliClass,
) -> ClassDiff {
    let mut class_diff = ClassDiff::new(rel);

    if smali_mod.path != smali_disass.path {
        class_diff.is_different = true;
        class_diff.path = Some((smali_mod.path, smali_disass.path));
    }
    if smali_mod.is_abstract != smali_disass.is_abstract {
        class_diff.is_different = true;
        class_diff.is_abstract = Some((smali_mod.is_abstract, smali_disass.is_abstract));
    }
    if smali_mod.super_path != smali_disass.super_path {
        if !(args.ignore_object_super
            && smali_mod.super_path == Some("java.lang.Object".to_string()))
        {
            class_diff.is_different = true;
            class_diff.super_path = Some((smali_mod.super_path, smali_disass.super_path));
        }
    }

    let mut wrong_implementations = vec![];
    for implmt in smali_mod.implements {
        if !smali_disass.implements.contains(&implmt) {
            wrong_implementations.push(implmt);
        }
    }
    if !wrong_implementations.is_empty() {
        class_diff.is_different = true;
        class_diff.wrong_implementations = Some(wrong_implementations);
    }

    let mut wrong_values: Vec<(SmaliValue, Option<SmaliValue>)> = vec![];
    for value in &smali_mod.values {
        let mut found = false;
        let mut alternative: Option<SmaliValue> = None;
        for disass_value in &smali_disass.values {
            if disass_value.name == value.name
                && disass_value.data_type == value.data_type
                && disass_value.is_static == value.is_static
            {
                found = true;
                break;
            } else if disass_value.name == value.name {
                alternative = Some(disass_value.clone());
            }
        }
        if !found {
            wrong_values.push((value.clone(), alternative));
        }
    }
    if !wrong_values.is_empty() {
        class_diff.is_different = true;
        class_diff.wrong_values = Some(wrong_values);
    }

    let mut wrong_methods = vec![];
    for method in &smali_mod.methods {
        let mut alternatives = vec![];

        let mut found = false;
        for disass_method in &smali_disass.methods {
            if method.name == disass_method.name {
                if parameter_types_equal(&method.parameter_types, &disass_method.parameter_types) {
                    if method.return_type == disass_method.return_type {
                        found = true;
                        break;
                    } else {
                        alternatives.clear();
                        alternatives.push(disass_method.clone());
                    }
                } else if args.ignore_default_constructors
                    && method.name == "<init>"
                    && method.parameter_types.is_empty()
                {
                    found = true;
                    break;
                } else {
                    alternatives.push(disass_method.clone());
                }
            }
        }

        if !found {
            wrong_methods.push((method.clone(), alternatives));
        }
    }

    if !wrong_methods.is_empty() {
        class_diff.is_different = true;
        class_diff.wrong_methods = Some(wrong_methods);
    }

    return class_diff;
}

fn parameter_types_equal(mods: &Vec<String>, disass: &Vec<String>) -> bool {
    if mods.len() != disass.len() {
        return false;
    }

    let mut res = true;
    for i in 0..mods.len() {
        let mod_val = mods.get(i).unwrap();
        let disass_val = disass.get(i).unwrap();

        if mod_val != disass_val
            && !(mod_val == "kotlin.jvm.internal.BTTVDefaultConstructorMarker"
                && disass_val == "kotlin.jvm.internal.DefaultConstructorMarker")
        {
            res = false;
            break;
        }
    }

    return res;
}

struct ClassDiff {
    rel: String,
    is_different: bool,
    path: Option<(String, String)>,
    is_abstract: Option<(bool, bool)>,
    super_path: Option<(Option<String>, Option<String>)>,
    wrong_implementations: Option<Vec<String>>,
    wrong_values: Option<Vec<(SmaliValue, Option<SmaliValue>)>>,
    wrong_methods: Option<Vec<(SmaliMethod, Vec<SmaliMethod>)>>,
}
impl ClassDiff {
    fn new(rel: String) -> Self {
        Self {
            rel,
            is_different: false,
            path: None,
            is_abstract: None,
            super_path: None,
            wrong_implementations: None,
            wrong_values: None,
            wrong_methods: None,
        }
    }
}
impl std::fmt::Debug for ClassDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut f = f.debug_struct(&format!("ClassDiff (\"{}\")", self.rel)[..]);
        if self.path.is_some() {
            f.field("path", &self.path.clone().unwrap());
        }
        if self.is_abstract.is_some() {
            f.field("is_abstract", &self.is_abstract.clone().unwrap());
        }
        if self.super_path.is_some() {
            f.field("super_path", &self.super_path.clone().unwrap());
        }
        if self.wrong_implementations.is_some() {
            f.field(
                "wrong_implementations",
                &self.wrong_implementations.clone().unwrap(),
            );
        }
        if self.wrong_values.is_some() {
            f.field("wrong_values", &self.wrong_values.clone().unwrap());
        }
        if self.wrong_methods.is_some() {
            f.field("wrong_methods", &self.wrong_methods.clone().unwrap());
        }

        return f.finish();
    }
}
