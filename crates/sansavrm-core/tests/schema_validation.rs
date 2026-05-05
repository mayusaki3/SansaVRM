// JSONスキーマ検証テスト
// @hldocs.ref doc-20260504-000402Z-SV0P#sec_a8k3m2q1
// @hldocs.ref doc-20260504-000402Z-SV0P#sec_b7n4p9r2

use std::fs;
use std::path::PathBuf;
use serde_json::Value;

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

#[test]
fn schema_001_load_defs_schema_should_be_valid_json() {
    let content = fs::read_to_string(schema_path("defs.schema.json")).unwrap();
    let parsed: Value = serde_json::from_str(&content).unwrap();
    assert!(parsed.is_object());
}

#[test]
fn schema_002_load_root_schema_should_be_valid_json() {
    let content = fs::read_to_string(schema_path("root.schema.json")).unwrap();
    let parsed: Value = serde_json::from_str(&content).unwrap();
    assert!(parsed.is_object());
}

#[test]
fn schema_003_defs_should_contain_property_definition() {
    let content = fs::read_to_string(schema_path("defs.schema.json")).unwrap();
    let parsed: Value = serde_json::from_str(&content).unwrap();

    let defs = parsed.get("$defs").unwrap();
    assert!(defs.get("Property").is_some());
}
