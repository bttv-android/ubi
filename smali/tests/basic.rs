use crate::parse_class;
use smali::*;

#[test]
fn test_simple_class() {
    let input = ".class Lbttv/test/Util;";
    let expected = SmaliClass::new(
        "bttv.test.Util".to_string(),
        SmaliAccessModifier::Package,
        false,
    );
    assert_eq!(parse_class(input).unwrap(), expected);
}

#[test]
fn test_super_class() {
    let input = ".class public abstract Lbttv/test/Util;
.super Lbttv/test/SuperClass$1;";
    let mut expected = SmaliClass::new(
        "bttv.test.Util".to_string(),
        SmaliAccessModifier::Public,
        true,
    );
    expected.super_path = Some("bttv.test.SuperClass$1".to_string());
    assert_eq!(parse_class(input).unwrap(), expected);
}

#[test]
fn test_interfaces_class() {
    let input = ".class Lbttv/test/Util;
.implements Lbttv/test/Interface$1;
.implements Lbttv/test/Interface$2;
.implements Lbttv/test/Interface$3;";
    let class = parse_class(input).unwrap();
    assert!(class
        .interfaces
        .contains(&"bttv.test.Interface$1".to_string()));
    assert!(class
        .interfaces
        .contains(&"bttv.test.Interface$2".to_string()));
    assert!(class
        .interfaces
        .contains(&"bttv.test.Interface$3".to_string()));

    assert_eq!(class.interfaces.len(), 3);
}

#[test]
fn test_class_with_fields() {
    let input = ".class public final enum Lbttv/test/Util$1;
.field private static final synthetic $VALUES:[Lbttv/test/Util$1;
.field public static final enum LIVE:Lbttv/test/Util$1;
.field public static final VOD:I
.field private final gqlVideoType:[Lbttv/test/Util$1;
.field private notSure:I";

    let res = parse_class(input).unwrap();

    assert_eq!(res.class_path, "bttv.test.Util$1".to_string());

    assert!(res.values.contains(&SmaliValue {
        name: "$VALUES".to_string(),
        access: SmaliAccessModifier::Private,
        is_final: true,
        is_static: true,
        data_type: SmaliType::Arr(Box::new(SmaliType::Class("bttv.test.Util$1".to_string())))
    }));

    assert!(res.values.contains(&SmaliValue {
        name: "LIVE".to_string(),
        access: SmaliAccessModifier::Public,
        is_final: true,
        is_static: true,
        data_type: SmaliType::Class("bttv.test.Util$1".to_string())
    }));

    assert!(res.values.contains(&SmaliValue {
        name: "VOD".to_string(),
        access: SmaliAccessModifier::Public,
        is_final: true,
        is_static: true,
        data_type: SmaliType::Int
    }));

    assert!(res.values.contains(&SmaliValue {
        name: "gqlVideoType".to_string(),
        access: SmaliAccessModifier::Private,
        is_final: true,
        is_static: false,
        data_type: SmaliType::Arr(Box::new(SmaliType::Class("bttv.test.Util$1".to_string())))
    }));

    assert!(res.values.contains(&SmaliValue {
        name: "notSure".to_string(),
        access: SmaliAccessModifier::Private,
        is_final: false,
        is_static: false,
        data_type: SmaliType::Int
    }));

    assert_eq!(res.values.len(), 5);
}
