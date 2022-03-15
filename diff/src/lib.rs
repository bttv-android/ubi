mod diff_vec;

use crate::diff_vec::*;
use smali::*;

pub struct ClassDiff<'orig, 'cmp> {
    class_path: Option<(&'orig String, &'cmp String)>,
    access: Option<(&'orig SmaliAccessModifier, &'cmp SmaliAccessModifier)>,
    is_abstract: Option<(bool, bool)>,
    super_path: Option<(&'orig Option<String>, &'cmp Option<String>)>,
    interfaces: Option<Vec<&'orig String>>,
    values: Option<Vec<ValueDiff<'orig, 'cmp>>>,
}

impl<'a, 'b> ClassDiff<'a, 'b> {
    fn new() -> Self {
        Self {
            class_path: None,
            access: None,
            is_abstract: None,
            super_path: None,
            interfaces: None,
            values: None,
        }
    }
}

pub struct ValueDiff<'orig, 'cmp> {
    name: &'orig String,
    data_type: Option<(&'orig SmaliType, &'cmp SmaliType)>,
    access: Option<(&'orig SmaliAccessModifier, &'cmp SmaliAccessModifier)>,
    is_static: Option<(bool, bool)>,
    is_final: Option<(bool, bool)>,
}

impl<'orig, 'cmp> ValueDiff<'orig, 'cmp> {
    fn new(name: &'orig String) -> Self {
        Self {
            name,
            data_type: None,
            access: None,
            is_static: None,
            is_final: None,
        }
    }
}

pub fn diff<'a, 'b>(orig: &'a SmaliClass, cmp: &'b SmaliClass) -> Option<ClassDiff<'a, 'b>> {
    let mut any_diff_found = false;

    let mut diff = ClassDiff::new();

    if orig.class_path != cmp.class_path {
        any_diff_found = true;
        diff.class_path = Some((&orig.class_path, &cmp.class_path));
    }

    if orig.access != cmp.access {
        any_diff_found = true;
        diff.access = Some((&orig.access, &cmp.access));
    }

    if orig.is_abstract != cmp.is_abstract {
        any_diff_found = true;
        diff.is_abstract = Some((orig.is_abstract, cmp.is_abstract));
    }

    if orig.super_path != cmp.super_path {
        any_diff_found = true;
        diff.super_path = Some((&orig.super_path, &cmp.super_path));
    }

    if let Some(vec_diff) = diff_string_vec(&orig.interfaces, &cmp.interfaces) {
        any_diff_found = true;
        diff.interfaces = Some(vec_diff);
    }

    if let Some(vec_diff) = diff_value_vec(&orig.values, &cmp.values) {
        any_diff_found = true;
        diff.values = Some(vec_diff);
    }

    if !any_diff_found {
        return None;
    }

    Some(diff)
}
