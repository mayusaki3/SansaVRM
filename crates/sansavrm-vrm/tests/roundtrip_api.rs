use sansavrm_core::{IoOptions, VrmVersion};
use sansavrm_vrm::{export_vrm, import_vrm};

#[test]
fn vrm_adapter_010_roundtrip_vrm_1_0_humanoid_should_preserve_bones() {
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
    "VRMC_vrm": {
      "specVersion": "1.0",
      "humanoid": {
        "humanBones": {
          "hips": { "node": 0 },
          "head": { "node": 1 }
        }
      }
    }
  }
}
"#;

    // Import
    let import_result = import_vrm(document.into());
    assert!(import_result.success);
    let model = import_result.data.expect("model");

    // Export
    let export_result = export_vrm(&model, VrmVersion::V1_0, IoOptions::default());
    assert!(export_result.success);
    let exported = export_result.data.expect("document");

    // Re-import
    let reimport_result = import_vrm(exported.into());
    assert!(reimport_result.success);
    let re_model = reimport_result.data.expect("model");

    // 検証：hips
    assert!(re_model.properties.iter().any(|p| {
        p.key == "vrm.humanoid.human_bones.hips.node"
            && p.value == "Hips"
    }));

    // 検証：head
    assert!(re_model.properties.iter().any(|p| {
        p.key == "vrm.humanoid.human_bones.head.node"
            && p.value == "Head"
    }));
}
