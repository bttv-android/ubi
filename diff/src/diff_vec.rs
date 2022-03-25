use crate::{MethodDiff, ValueDiff};
use smali::{SmaliMethod, SmaliValue};

/// returns Some with all items that are in orig and not in cmp or None if empty
pub fn diff_string_vec<'a, 'b>(orig: &'a [String], cmp: &'b [String]) -> Option<Vec<&'a String>> {
    let mut vec = vec![];

    // perf: runs in O(n*m) maybe fix later
    for item in orig {
        if !cmp.contains(item) {
            vec.push(item);
        }
    }

    if vec.is_empty() {
        None
    } else {
        Some(vec)
    }
}

/// returns Some with all items that are in orig and not in cmp or None if empty
pub fn diff_value_vec<'a, 'b>(
    orig: &'a [SmaliValue],
    cmp: &'b [SmaliValue],
) -> Option<Vec<ValueDiff<'a, 'b>>> {
    let mut diffs = vec![];

    // perf: runs in O(n*m) maybe fix later
    for item in orig {
        let mut found = false;
        for other in cmp {
            if item.name != other.name {
                continue;
            }

            found = true;

            let mut any_changes_found = false;
            let mut diff = ValueDiff::new(&item.name);
            if item.is_final != other.is_final {
                any_changes_found = true;
                diff.is_final = Some((item.is_final, other.is_final));
            }
            if item.is_static != other.is_static {
                any_changes_found = true;
                diff.is_static = Some((item.is_static, other.is_static));
            }
            if item.access != other.access {
                any_changes_found = true;
                diff.access = Some((&item.access, &other.access));
            }
            if item.data_type != other.data_type {
                any_changes_found = true;
                diff.data_type = Some((&item.data_type, &other.data_type));
            }
            if any_changes_found {
                diffs.push(diff);
            } else {
                break;
            }
        }
        if !found {
            diffs.push(ValueDiff::not_found(&item.name));
        }
    }

    if diffs.is_empty() {
        None
    } else {
        Some(diffs)
    }
}

/// returns Some with all items that are in orig and not in cmp or None if empty
pub fn diff_method_vec<'a, 'b>(
    orig: &'a [SmaliMethod],
    cmp: &'b [SmaliMethod],
) -> Option<Vec<MethodDiff<'a, 'b>>> {
    let mut diffs = vec![];

    // perf: runs in O(n*m) maybe fix later
    for item in orig {
        let mut found = false;
        for other in cmp {
            if item.name != other.name {
                continue;
            }

            found = true;

            let mut any_changes_found = false;
            let mut diff = MethodDiff::new(&item.name);
            if item.is_final != other.is_final {
                any_changes_found = true;
                diff.is_final = Some((item.is_final, other.is_final));
            }
            if item.is_static != other.is_static {
                any_changes_found = true;
                diff.is_static = Some((item.is_static, other.is_static));
            }
            if item.access != other.access {
                any_changes_found = true;
                diff.access = Some((&item.access, &other.access));
            }
            if item.return_type != other.return_type {
                any_changes_found = true;
                diff.return_type = Some((&item.return_type, &other.return_type));
            }
            if item.parameter_types != other.parameter_types {
                any_changes_found = true;
                diff.parameter_types = Some((&item.parameter_types, &other.parameter_types));
            }
            if any_changes_found {
                diffs.push(diff);
            } else {
                break;
            }
        }
        if !found {
            diffs.push(MethodDiff::not_found(&item.name));
        }
    }

    if diffs.is_empty() {
        None
    } else {
        Some(diffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smali::{SmaliAccessModifier, SmaliType};

    #[test]
    fn test_diff_string_vec_none() {
        let a = ["a".to_string(), "b".to_string(), "c".to_string()];
        let b = ["b".to_string(), "c".to_string(), "a".to_string()];
        assert!(diff_string_vec(&a, &b).is_none());
    }

    #[test]
    fn test_diff_string_vec_some() {
        let a = ["a".to_string(), "b".to_string(), "c".to_string()];
        let b = ["b".to_string(), "d".to_string()];
        let diff = diff_string_vec(&a, &b);
        assert!(diff.is_some());
        let diff = diff.unwrap();
        assert!(diff.contains(&&"a".to_string()));
        assert!(diff.contains(&&"c".to_string()));
        assert_eq!(diff.len(), 2);
    }

    #[test]
    fn test_diff_value_vec_none() {
        let v1 = SmaliValue {
            name: "height".to_string(),
            access: SmaliAccessModifier::Package,
            data_type: SmaliType::Double,
            is_final: false,
            is_static: true,
        };
        let v2 = SmaliValue {
            name: "age".to_string(),
            access: SmaliAccessModifier::Public,
            data_type: SmaliType::Int,
            is_final: false,
            is_static: false,
        };
        let a = [v2.clone(), v1.clone()];
        let b = [v1, v2];
        let diff = diff_value_vec(&a, &b);
        assert!(diff.is_none());
    }

    #[test]
    fn test_diff_value_vec_some() {
        let mut v1 = SmaliValue {
            name: "height".to_string(),
            access: SmaliAccessModifier::Package,
            data_type: SmaliType::Double,
            is_final: false,
            is_static: true,
        };
        let v2 = SmaliValue {
            name: "age".to_string(),
            access: SmaliAccessModifier::Public,
            data_type: SmaliType::Int,
            is_final: false,
            is_static: false,
        };
        let a = [v2.clone(), v1.clone()];
        v1.is_static = false;
        v1.data_type = SmaliType::Class("java.lang.Double".to_string());

        let b = [v1];
        let diff = diff_value_vec(&a, &b);
        assert!(diff.is_some());
        let diff = diff.unwrap();

        assert_eq!(diff.len(), 2);
        assert!(diff.contains(&ValueDiff::not_found(&"age".to_string())));
        assert!(diff.contains(&ValueDiff {
            not_found: false,
            name: &"height".to_string(),
            access: None,
            is_final: None,
            is_static: Some((true, false)),
            data_type: Some((
                &SmaliType::Double,
                &SmaliType::Class("java.lang.Double".to_string())
            )),
        }));
    }
}
