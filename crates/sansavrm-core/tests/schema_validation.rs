// JSONスキーマ検証テスト
// @hldocs.ref doc-20260504-000402Z-SV0P#sec_a8k3m2q1
// @hldocs.ref doc-20260504-000402Z-SV0P#sec_b7n4p9r2
// @hldocs.ref doc-20260504-000402Z-SV0P#sec_c6t5v8s3
// @hldocs.ref doc-20260504-000402Z-SV0P#sec_d5w6x7u4
// @hldocs.ref doc-20260504-000402Z-SV0P#sec_e4y7z6v5
// @hldocs.ref doc-20260504-000402Z-SV0P#sec_f3a8b5w6

use serde_json::Value;
use std::fs;
use std::path::PathBuf;

const SCHEMA_FILES: &[&str] = &[
    "defs.schema.json",
    "root.schema.json",
    "model.schema.json",
    "modules.schema.json",
    "slots.schema.json",
    "states.schema.json",
    "rights.schema.json",
    "revenue.schema.json",
    "compatibility.schema.json",
    "diagnostics.schema.json",
    "extension-layer.schema.json",
];

/// ワークスペースルート配下のスキーマファイルパスを生成する。
///
/// 役割:
/// - Cargo のテスト実行起点に依存せず、リポジトリ直下の schemas を参照する。
///
/// 注意点:
/// - `CARGO_MANIFEST_DIR` は `crates/sansavrm-core` を指すため、2階層上へ戻る。
fn schema_path(file_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("schemas")
        .join(file_name)
}

/// スキーマJSONを読み込む。
///
/// 役割:
/// - 各テストで同じ読み込み処理を共有する。
///
/// 引数:
/// - file_name: schemas 配下のファイル名。
///
/// 戻り値:
/// - serde_json::Value としてパースされたスキーマ。
fn load_schema(file_name: &str) -> Value {
    let content = fs::read_to_string(schema_path(file_name)).unwrap();
    serde_json::from_str(&content).unwrap()
}

#[test]
fn schema_001_all_schema_files_should_be_valid_json_objects() {
    for file_name in SCHEMA_FILES {
        let parsed = load_schema(file_name);
        assert!(parsed.is_object(), "{} should be object", file_name);
    }
}

#[test]
fn schema_002_all_schema_files_should_have_expected_id() {
    for file_name in SCHEMA_FILES {
        let parsed = load_schema(file_name);
        let expected_id = format!(
            "https://sansavrm.local/schema/sansavrm/v1/{}",
            file_name
        );
        assert_eq!(parsed.get("$id").and_then(Value::as_str), Some(expected_id.as_str()));
    }
}

#[test]
fn schema_003_root_schema_should_require_extensions() {
    let root = load_schema("root.schema.json");
    let required = root.get("required").and_then(Value::as_array).unwrap();

    assert!(required.iter().any(|value| value == "extensions"));
}

#[test]
fn schema_004_root_extensions_should_require_core_extensions() {
    let root = load_schema("root.schema.json");
    let required = root
        .pointer("/properties/extensions/required")
        .and_then(Value::as_array)
        .unwrap();

    assert!(required.iter().any(|value| value == "SansaVRM_model"));
    assert!(required.iter().any(|value| value == "SansaVRM_modules"));
    assert!(required.iter().any(|value| value == "SansaVRM_slots"));
}

#[test]
fn schema_005_defs_should_contain_property_definition() {
    let defs_schema = load_schema("defs.schema.json");
    let defs = defs_schema.get("$defs").unwrap();

    assert!(defs.get("Property").is_some());
    assert!(defs.get("PropertyValue").is_some());
    assert!(defs.get("PropertyConstraints").is_some());
}

#[test]
fn schema_006_property_should_require_type_context_and_value() {
    let defs_schema = load_schema("defs.schema.json");
    let required = defs_schema
        .pointer("/$defs/Property/required")
        .and_then(Value::as_array)
        .unwrap();

    assert!(required.iter().any(|value| value == "property_id"));
    assert!(required.iter().any(|value| value == "key"));
    assert!(required.iter().any(|value| value == "value"));
    assert!(required.iter().any(|value| value == "property_type"));
    assert!(required.iter().any(|value| value == "context"));
}

#[test]
fn schema_007_property_should_reject_additional_properties() {
    let defs_schema = load_schema("defs.schema.json");
    let additional_properties = defs_schema
        .pointer("/$defs/Property/additionalProperties")
        .and_then(Value::as_bool);

    assert_eq!(additional_properties, Some(false));
}

#[test]
fn schema_008_property_value_should_define_string_number_and_bool_variants() {
    let defs_schema = load_schema("defs.schema.json");
    let variants = defs_schema
        .pointer("/$defs/PropertyValue/oneOf")
        .and_then(Value::as_array)
        .unwrap();

    assert_eq!(variants.len(), 3);
}

#[test]
fn schema_009_model_schema_should_reference_property_definition() {
    let model = load_schema("model.schema.json");
    let property_ref = model
        .pointer("/properties/properties/items/$ref")
        .and_then(Value::as_str);

    assert_eq!(property_ref, Some("defs.schema.json#/$defs/Property"));
}
