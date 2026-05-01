// crates/sansavrm-vrm/tests/roundtrip.rs

use sansavrm_core::{IoOptions, VrmVersion};
use sansavrm_vrm::{export_vrm, import_vrm};

#[test]
fn vrm_roundtrip_001_import_export_import_vrm_1_0_should_keep_nodes() {
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

    let reimported = import_vrm(exported_document);
    assert!(reimported.success);

    let reimported_model = reimported.data.expect("model should be returned");

    assert_eq!(reimported_model.modules.len(), 2);
    assert_eq!(reimported_model.modules[0].module_id, "Root");
    assert_eq!(reimported_model.modules[1].module_id, "Head");
    assert_eq!(reimported_model.vrm_version, Some(VrmVersion::V1_0));
}

#[test]
fn vrm_roundtrip_002_import_export_import_vrm_0x_should_keep_nodes() {
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

    let reimported = import_vrm(exported_document);
    assert!(reimported.success);

    let reimported_model = reimported.data.expect("model should be returned");

    assert_eq!(reimported_model.modules.len(), 2);
    assert_eq!(reimported_model.modules[0].module_id, "Root");
    assert_eq!(reimported_model.modules[1].module_id, "Body");
    assert_eq!(reimported_model.vrm_version, Some(VrmVersion::V0x));
}

#[test]
fn vrm_adapter_013_roundtrip_vrm_0x_humanoid_should_preserve_bones() {
    let document = r#"
{
  "asset": {
    "version": "2.0"
  },
  "nodes": [
    { "name": "Hips" },
    { "name": "Head" }
  ],
  "extensions": {
    "VRM": {
      "specVersion": "0.0",
      "humanoid": {
        "humanBones": [
          { "bone": "hips", "node": 0 },
          { "bone": "head", "node": 1 }
        ]
      }
    }
  }
}
"#;

    let import_result = import_vrm(document.into());
    assert!(import_result.success);

    let model = import_result.data.expect("model should be returned");

    let export_result = export_vrm(&model, VrmVersion::V0x, IoOptions::default());
    assert!(export_result.success);

    let exported = export_result.data.expect("document should be returned");

    let reimport_result = import_vrm(exported.into());
    assert!(reimport_result.success);

    let re_model = reimport_result.data.expect("model should be returned");

    assert!(re_model.properties.iter().any(|property| {
        property.key == "vrm.humanoid.human_bones.hips.node"
            && property.value == "Hips"
    }));

    assert!(re_model.properties.iter().any(|property| {
        property.key == "vrm.humanoid.human_bones.head.node"
            && property.value == "Head"
    }));
}
