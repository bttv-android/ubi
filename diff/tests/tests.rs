use diff::{ClassDiff, MethodDiff};
use smali::SmaliType;
use std::path::PathBuf;

fn files_paths(dir: &'static str) -> (PathBuf, PathBuf) {
    let base_path = std::env::current_dir().unwrap().join("tests/").join(dir);
    (base_path.join("orig.smali"), base_path.join("cmp.smali"))
}

#[test]
fn one() {
    let (orig_path, cmp_path) = files_paths("1/");
    let orig = smali::parse_file(orig_path).unwrap();
    let cmp = smali::parse_file(cmp_path).unwrap();
    let class_diff = diff::diff(&orig, &cmp).unwrap();

    let init_params_before = vec![SmaliType::Int];
    let init_params_after = vec![
        SmaliType::Arr(Box::new(SmaliType::Int)),
        SmaliType::Arr(Box::new(SmaliType::Int)),
    ];
    let init_method_diff = MethodDiff {
        name: &"<init>".to_string(),
        not_found: false,
        return_type: None,
        access: None,
        is_static: None,
        is_final: None,
        parameter_types: Some((&init_params_before, &init_params_after)),
    };

    let name = "onClick".to_string();
    let on_click_method_diff = MethodDiff::not_found(&name);

    let expected = ClassDiff {
        class_path: None,
        access: None,
        is_abstract: Some((true, false)),
        super_path: None,
        interfaces: None,
        values: None,
        methods: Some(vec![init_method_diff, on_click_method_diff]),
    };
    dbg!(&expected, &class_diff);
    assert_eq!(expected, class_diff);
}
