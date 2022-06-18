use lnpkg::*;
use std::collections::HashMap;
#[test]
fn basic_list() {
    let sample = "[hello;world]";
    let output = LnPkgValue::List(vec!["hello".to_string(), "world".to_string()]);
    assert_eq!(LnPkgValue::from_string_to_list(sample), Some(output))
}

#[test]
fn one_element() {
    let sample = "[hello]";
    let output = LnPkgValue::List(vec!["hello".to_string()]);
    assert_eq!(LnPkgValue::from_string_to_list(sample), Some(output));
}

#[test]
fn empty() {
    let sample = "[]";
    let output = LnPkgValue::List(vec![]);
    assert_eq!(LnPkgValue::from_string_to_list(sample), Some(output))
}

#[test]
fn parse_basic_list() {
    let sample = "type=msg:names=[jhonny;mark]:".to_string();
    let mut hm = HashMap::new();
    hm.insert(
        "names".to_string(),
        LnPkgValue::List(vec!["jhonny".to_string(), "mark".to_string()]),
    );
    assert_eq!(
        LnPkg::from_string(&sample),
        LnPkg::from_hashmap(hm, LnPkgType::Message)
    )
}
#[test]
fn parse_emtpy_list() {
    let sample = "type=msg:names=[]";
    let mut hm = HashMap::new();
    hm.insert("names".to_string(), LnPkgValue::List(Vec::new()));

    assert_eq!(
        LnPkg::from_string(sample),
        LnPkg::from_hashmap(hm, LnPkgType::Message)
    );
}
