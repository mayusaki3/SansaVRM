use sansavrm_core::{IoOptions, VrmVersion};
use sansavrm_vrm::{export_vrm, import_vrm};
use sansavrm_test_utils::json::normalize_json;

#[test]
fn vrm_rt_api_tc_001_vrm1_roundtrip_preserve_humanoid() {
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

#[test]
fn vrm_rt_api_tc_002_vrm0_roundtrip_preserve_humanoid() {
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

#[test]
fn vrm_rt_api_tc_003_vrm1_json_roundtrip_should_match_semantically() {
    let document = r#"
{
  "asset": { "version": "2.0" },
  "nodes": [
    { "name": "Hips" },
    { "name": "Head" }
  ],
  "extensions": {
    "VRMC_vrm": {
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

    let import_result = import_vrm(document.into());
    assert!(import_result.success);

    let model = import_result.data.unwrap();

    let export_result = export_vrm(&model, VrmVersion::V1_0, IoOptions::default());
    assert!(export_result.success);

    let exported = export_result.data.unwrap();

    let original_json: serde_json::Value = serde_json::from_str(document).unwrap();
    let exported_json: serde_json::Value = serde_json::from_str(&exported).unwrap();

    assert_eq!(
        normalize_json(&original_json["extensions"]["VRMC_vrm"]["humanoid"]["humanBones"]),
        normalize_json(&exported_json["extensions"]["VRMC_vrm"]["humanoid"]["humanBones"])
    );
}
