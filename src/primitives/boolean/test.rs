use std::str::FromStr;

use super::*;

#[test]
fn test_new_and_accessors() {
    let b_true = Boolean::new(true);
    assert_eq!(b_true.as_bool(), true);
    assert_eq!(b_true.as_str(), "true");
    assert_eq!(b_true.into_inner(), true);

    let b_false = Boolean::new(false);
    assert_eq!(b_false.as_bool(), false);
    assert_eq!(b_false.as_str(), "false");
    assert_eq!(b_false.into_inner(), false);
}

#[test]
fn test_from_and_into_bool() {
    let b: Boolean = true.into();
    assert_eq!(b, Boolean::new(true));

    let val: bool = b.into();
    assert_eq!(val, true);
}

#[test]
fn test_try_from_str_valid() {
    assert_eq!(Boolean::try_from("true"), Ok(Boolean::new(true)));
    assert_eq!(Boolean::try_from("false"), Ok(Boolean::new(false)));
}

#[test]
fn test_try_from_str_invalid() {
    assert!(Boolean::try_from("True").is_err());
    assert!(Boolean::try_from("FALSE").is_err());
    assert!(Boolean::try_from("1").is_err());
    assert!(Boolean::try_from("0").is_err());
    assert!(Boolean::try_from("yes").is_err());
    assert!(Boolean::try_from("no").is_err());
    assert!(Boolean::try_from("").is_err());

    let err = Boolean::try_from("invalid").unwrap_err();
    assert_eq!(
        err.to_string(),
        "invalid value 'invalid' for type 'boolean': expected 'true' or 'false'"
    );
}

#[test]
fn test_try_from_string() {
    assert_eq!(
        Boolean::try_from(String::from("true")),
        Ok(Boolean::new(true))
    );
    assert_eq!(
        Boolean::try_from(String::from("false")),
        Ok(Boolean::new(false))
    );
    assert!(Boolean::try_from(String::from("invalid")).is_err());
}

#[test]
fn test_from_str() {
    assert_eq!(Boolean::from_str("true"), Ok(Boolean::new(true)));
    assert_eq!(Boolean::from_str("false"), Ok(Boolean::new(false)));
    assert!(Boolean::from_str("invalid").is_err());
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", Boolean::new(true)), "true");
    assert_eq!(format!("{}", Boolean::new(false)), "false");
}

#[test]
fn test_default() {
    assert_eq!(Boolean::default(), Boolean::new(false));
}

#[test]
fn test_serde_serialization() {
    let b_true = Boolean::new(true);
    assert_eq!(serde_json::to_string(&b_true).unwrap(), "true");

    let b_false = Boolean::new(false);
    assert_eq!(serde_json::to_string(&b_false).unwrap(), "false");
}

#[test]
fn test_serde_deserialization_from_str() {
    let b_true: Boolean = serde_json::from_str("true").unwrap();
    assert_eq!(b_true, Boolean::new(true));

    let b_false: Boolean = serde_json::from_str("false").unwrap();
    assert_eq!(b_false, Boolean::new(false));
}

#[test]
fn test_serde_deserialization_from_reader() {
    let b_true: Boolean = serde_json::from_reader(&b"true"[..]).unwrap();
    assert_eq!(b_true, Boolean::new(true));

    let b_false: Boolean = serde_json::from_reader(&b"false"[..]).unwrap();
    assert_eq!(b_false, Boolean::new(false));
}

#[test]
fn test_serde_deserialization_from_value() {
    let val_true = serde_json::Value::Bool(true);
    let b_true: Boolean = serde_json::from_value(val_true).unwrap();
    assert_eq!(b_true, Boolean::new(true));

    let val_false = serde_json::Value::Bool(false);
    let b_false: Boolean = serde_json::from_value(val_false).unwrap();
    assert_eq!(b_false, Boolean::new(false));
}

#[test]
fn test_serde_deserialization_invalid_type() {
    // Strings, numbers, null, objects, arrays should fail
    assert!(serde_json::from_str::<Boolean>("\"true\"").is_err());
    assert!(serde_json::from_str::<Boolean>("1").is_err());
    assert!(serde_json::from_str::<Boolean>("0").is_err());
    assert!(serde_json::from_str::<Boolean>("null").is_err());
    assert!(serde_json::from_str::<Boolean>("[]").is_err());
    assert!(serde_json::from_str::<Boolean>("{}").is_err());
}

#[test]
fn test_serde_roundtrip() {
    let original = Boolean::new(true);
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Boolean = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);

    let original_false = Boolean::new(false);
    let json_false = serde_json::to_string(&original_false).unwrap();
    let deserialized_false: Boolean = serde_json::from_str(&json_false).unwrap();
    assert_eq!(original_false, deserialized_false);
}
