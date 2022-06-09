use crate::{LnPkg, LnPkgType, LnPkgValue as lpv};
use std::collections::HashMap;
#[test]
fn test_lnpkgvalues() {
    let sample = vec![
        "".to_string(),       // Null
        "false".to_string(),  // Bool(false)
        "true".to_string(),   // Bool(true)
        "hello!".to_string(), // String("hello!")
        "123".to_string(),    // Int(123)
    ];
    let target = vec![
        lpv::Null,
        lpv::Bool(false),
        lpv::Bool(true),
        lpv::String(String::from("hello!")),
        lpv::Int(123),
    ];

    for i in 0..sample.len() {
        assert_eq!(lpv::from_string(sample[i].clone()), target[i])
    }
}

#[test]
fn test_lnpkg_parsing() {
    let sample = String::from("key:key2:key3");
    let mut target = HashMap::new();
    target.insert(String::from("key"), lpv::Null);
    target.insert(String::from("key2"), lpv::Null);
    target.insert(String::from("key3"), lpv::Null);
    assert_eq!(LnPkg::from_string(sample), LnPkg::from_hashmap(target));

    let sample = String::from("key=This is a string:key2=false:key3=32");
    let mut target = HashMap::new();
    target.insert(
        String::from("key"),
        lpv::String(String::from("This is a string")),
    );
    target.insert(String::from("key2"), lpv::Bool(false));
    target.insert(String::from("key3"), lpv::Int(32));
    assert_eq!(LnPkg::from_string(sample), LnPkg::from_hashmap(target));

    let sample = String::from("key=This is a string:key2=false:key3=32::null key");
    let mut target = HashMap::new();
    target.insert(
        String::from("key"),
        lpv::String(String::from("This is a string")),
    );
    target.insert(String::from("key2"), lpv::Bool(false));
    target.insert(String::from("key3"), lpv::Int(32));
    target.insert(String::from("null key"), lpv::Null);
    assert_eq!(LnPkg::from_string(sample), LnPkg::from_hashmap(target));
}
#[test]
fn test_lnpkg_type() {
    let sample = "type=msg:msg=Hello starshine".to_string();
    let mut hmOutput = HashMap::new();
    hmOutput.insert(
        String::from("msg"),
        lpv::String("Hello starshine".to_string()),
    );
    let pkg_type = LnPkgType::Message;
    assert_eq!(LnPkg::from_string(sample.clone()).content, hmOutput);
    assert_eq!(LnPkg::from_string(sample).pkg_type, pkg_type);

    let sample = "content=Im null".to_string();
    let mut hmOutput = HashMap::new();
    hmOutput.insert(String::from("content"), lpv::String("Im null".to_string()));
    let pkg_type = LnPkgType::Unknown;
    assert_eq!(LnPkg::from_string(sample.clone()).content, hmOutput);
    assert_eq!(LnPkg::from_string(sample).pkg_type, pkg_type);

    let sample = "type=dontknow:content=Im null".to_string();
    let mut hmOutput = HashMap::new();
    hmOutput.insert(String::from("content"), lpv::String("Im null".to_string()));
    let pkg_type = LnPkgType::Unknown;
    assert_eq!(LnPkg::from_string(sample.clone()).content, hmOutput);
    assert_eq!(LnPkg::from_string(sample).pkg_type, pkg_type);
}

#[test]
fn test_to_string() {
    let sample = "type=msg:msg=Hello!:".to_string();
    let output = sample.clone();
    assert_eq!(LnPkg::from_string(sample).to_string(), output);

    let sample = "msg=Hello!:name=folgue:".to_string();
    let output = sample.clone();
    assert_eq!(LnPkg::from_string(sample).to_string(), output);
}
