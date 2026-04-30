use sansavrm_core::Model;
use sansavrm_gltf::{export_gltf, import_gltf};

#[test]
fn gltf_adapter_001_import_minimal_gltf_should_create_model() {
    let document = r#"
{
  "asset": {
    "version": "2.0"
  },
  "nodes": [
    { "name": "Root" },
    { "name": "Arm" }
  ]
}
"#;

    let result = import_gltf(document.into());

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.modules.len(), 2);
    assert_eq!(model.modules[0].module_id, "Root");
    assert_eq!(model.modules[1].module_id, "Arm");
}

#[test]
fn gltf_adapter_002_export_returns_not_implemented() {
    let model = Model::new();
    let result = export_gltf(&model);
    assert!(!result.success);
}

#[test]
fn gltf_adapter_003_import_invalid_json_should_fail() {
    let result = import_gltf("{ invalid json".into());

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn gltf_adapter_004_import_without_asset_should_fail() {
    let result = import_gltf(r#"{ "nodes": [] }"#.into());

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}
