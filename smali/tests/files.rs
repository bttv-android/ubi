use smali::*;
use std::path::PathBuf;

fn files_path() -> PathBuf {
    std::env::current_dir().unwrap().join("tests/files")
}

#[test]
fn one() {
    let path = files_path().join("one.smali");
    let res = parse_file(path);
    let class = res.unwrap();

    assert_eq!(class.class_path, "bttv.SleepTimer$2".to_string());
    assert_eq!(class.access, SmaliAccessModifier::Package);
    assert_eq!(class.is_abstract, false);
    assert_eq!(class.super_path.unwrap(), "java.lang.Object".to_string());

    assert_eq!(
        class.interfaces,
        vec!["android.content.DialogInterface$OnClickListener".to_string()]
    );

    assert_eq!(class.values.len(), 2);
    assert!(class.values.contains(&SmaliValue {
        name: "val$minutes".to_string(),
        is_final: true,
        is_static: false,
        access: SmaliAccessModifier::Package,
        data_type: SmaliType::Arr(Box::new(SmaliType::Int))
    }));
    assert!(class.values.contains(&SmaliValue {
        name: "val$selected".to_string(),
        is_final: true,
        is_static: false,
        access: SmaliAccessModifier::Package,
        data_type: SmaliType::Arr(Box::new(SmaliType::Int))
    }));

    assert_eq!(class.methods.len(), 2);
    assert!(class.methods.contains(&SmaliMethod {
        name: "<init>".to_string(),
        access: SmaliAccessModifier::Package,
        parameter_types: vec![
            SmaliType::Arr(Box::new(SmaliType::Int)),
            SmaliType::Arr(Box::new(SmaliType::Int))
        ],
        return_type: SmaliType::Void,
        is_static: false,
        is_final: false
    }));
    assert!(class.methods.contains(&SmaliMethod {
        name: "onClick".to_string(),
        access: SmaliAccessModifier::Public,
        parameter_types: vec![
            SmaliType::Class("android.content.DialogInterface".to_string()),
            SmaliType::Int
        ],
        return_type: SmaliType::Void,
        is_static: false,
        is_final: false
    }));
}
