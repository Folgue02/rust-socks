use lnpkg;
use std::collections::HashMap;
#[test]
fn test_exist() {
    let mut hm = HashMap::new();
    hm.insert("exists".to_string(), lnpkg::LnPkgValue::Null);
    hm.insert("stillexists".to_string(), lnpkg::LnPkgValue::Null);
    let pkg = lnpkg::LnPkg::from_hashmap(hm, lnpkg::LnPkgType::Message);

    assert!(pkg.exist(&["exists", "stillexists"]));
    assert!(!pkg.exist(&[&"doesntexist"]));
}