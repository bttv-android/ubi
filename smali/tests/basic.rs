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
}
