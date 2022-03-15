use crate::ValueDiff;
use smali::SmaliValue;

/// returns Some with all items that are in orig and not in cmp or None if empty
pub fn diff_string_vec<'a, 'b>(
    orig: &'a Vec<String>,
    cmp: &'b Vec<String>,
) -> Option<Vec<&'a String>> {
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
    orig: &'a Vec<SmaliValue>,
    cmp: &'b Vec<SmaliValue>,
) -> Option<Vec<ValueDiff<'a, 'b>>> {
    let mut diffs = vec![];

    // perf: runs in O(n*m) maybe fix later
    for item in orig {
        for other in cmp {
            if item.name != other.name {
                continue;
            }

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
    }

    if diffs.is_empty() {
        None
    } else {
        Some(diffs)
    }
}
