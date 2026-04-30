use sansavrm_core::{IoOptions, Model, VrmVersion};
use sansavrm_vrm::{export_vrm, import_vrm};

#[test]
fn vrm_adapter_001_import_minimal_vrm_should_create_model() {
    let document = r#"
{
  "asset": {
    "version": "2.0"
  },
  "nodes": [
    { "name": "Root" },
    { "name": "Head" }
  ],
  "extensions": {
    "VRMC_vrm": {
      "specVersion": "1.0"
    }
  }
}
"#;

    let result = import_vrm(document.into());

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.modules.len(), 2);
    assert_eq!(model.modules[0].module_id, "Root");
    assert_eq!(model.modules[1].module_id, "Head");
}

#[test]
fn vrm_adapter_002_export_vrm_1_0_returns_not_implemented() {
    let model = Model::new();
    let result = export_vrm(&model, VrmVersion::V1_0, IoOptions::default());
    assert!(!result.success);
}

#[test]
fn vrm_adapter_003_import_invalid_json_should_fail() {
    let result = import_vrm("{ invalid json".into());

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}
