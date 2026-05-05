// JSONスキーマ検証テスト
// @hldocs.ref doc-20260504-000402Z-SV0P#sec_a8k3m2q1
// @hldocs.ref doc-20260504-000402Z-SV0P#sec_b7n4p9r2

use std::fs;
use serde_json::Value;

#[test]
fn schema_001_load_defs_schema_should_be_valid_json() {
    let content = fs::read_to_string("schemas/defs.schema.json").unwrap();
    let parsed: Value = serde_json::from_str(&content).unwrap();
    assert!(parsed.is_object());
}

#[test]
fn schema_002_load_root_schema_should_be_valid_json() {
    let content = fs::read_to_string("schemas/root.schema.json").unwrap();
    let parsed: Value = serde_json::from_str(&content).unwrap();
    assert!(parsed.is_object());
}

#[test]
fn schema_003_defs_should_contain_property_definition() {
    let content = fs::read_to_string("schemas/defs.schema.json").unwrap();
    let parsed: Value = serde_json::from_str(&content).unwrap();

    let defs = parsed.get("$defs").unwrap();
    assert!(defs.get("Property").is_some());
}
