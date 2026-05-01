// crates/sansavrm-vrm/tests/roundtrip.rs

use sansavrm_core::{IoOptions, VrmVersion};
use sansavrm_vrm::{export_vrm, import_vrm};
use sansavrm_test_utils::json::normalize_json;

#[test]
fn vrm_rt_core_tc_001_vrm1_import_export_import_should_keep_nodes() {
    let input = r#"
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

    let imported = import_vrm(input.into());
    assert!(imported.success);

    let model = imported.data.expect("model should be returned");
    assert_eq!(model.modules.len(), 2);

    let exported = export_vrm(&model, VrmVersion::V1_0, IoOptions::default());
    assert!(exported.success);

    let exported_document = exported.data.expect("document should be returned");

    let reimported = import_vrm(exported_document.clone());
    assert!(reimported.success);

    let reimported_model = reimported.data.expect("model should be returned");

    assert_eq!(reimported_model.modules.len(), 2);

    let original_json: serde_json::Value = serde_json::from_str(input).unwrap();
    let exported_json: serde_json::Value =
        serde_json::from_str(&exported_document).unwrap();

    assert_eq!(
        normalize_json(&original_json["nodes"]),
        normalize_json(&exported_json["nodes"])
    );

    assert_eq!(reimported_model.modules[0].module_id, "Root");
    assert_eq!(reimported_model.modules[1].module_id, "Head");
    assert_eq!(reimported_model.vrm_version, Some(VrmVersion::V1_0));
}

#[test]
fn vrm_rt_core_tc_002_vrm0_import_export_import_should_keep_nodes() {
    let input = r#"
{
  "asset": {
    "version": "2.0"
  },
  "nodes": [
    { "name": "Root" },
    { "name": "Body" }
  ],
  "extensions": {
    "VRM": {
      "specVersion": "0.0"
    }
  }
}
"#;

    let imported = import_vrm(input.into());
    assert!(imported.success);

    let model = imported.data.expect("model should be returned");
    assert_eq!(model.modules.len(), 2);

    let exported = export_vrm(&model, VrmVersion::V0x, IoOptions::default());
    assert!(exported.success);

    let exported_document = exported.data.expect("document should be returned");

    let reimported = import_vrm(exported_document.clone());
    assert!(reimported.success);

    let reimported_model = reimported.data.expect("model should be returned");

    assert_eq!(reimported_model.modules.len(), 2);

    let original_json: serde_json::Value = serde_json::from_str(input).unwrap();
    let exported_json: serde_json::Value =
        serde_json::from_str(&exported_document).unwrap();

    assert_eq!(
        normalize_json(&original_json["nodes"]),
        normalize_json(&exported_json["nodes"])
    );

    assert_eq!(reimported_model.modules[0].module_id, "Root");
    assert_eq!(reimported_model.modules[1].module_id, "Body");
    assert_eq!(reimported_model.vrm_version, Some(VrmVersion::V0x));
}
