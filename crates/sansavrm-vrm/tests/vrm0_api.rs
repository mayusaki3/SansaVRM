use sansavrm_core::{
    IoOptions, Model, Module, ModuleType, Property, PropertyContext, PropertyType,
    VrmVersion,
};
use sansavrm_vrm::{export_vrm, import_vrm};

/// VRM0 exportでglTF JSONとVRM拡張を生成できることを検証する。
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p3
#[test]
fn vrm0_tc_001_export_should_create_gltf_json() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "Root".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.properties.push(Property::from_typed_value(
        "property_vrm_meta_name",
        "vrm.meta.name",
        sansavrm_core::PropertyValue::String("Exported VRM".into()),
        PropertyType::Metadata,
        PropertyContext::Description,
    ));

    let result = export_vrm(&model, VrmVersion::V0x, IoOptions::default());

    assert!(result.success);

    let document = result.data.expect("document should be returned");
    assert!(document.contains("\"version\": \"2.0\""));
    assert!(document.contains("\"name\": \"Root\""));
    assert!(document.contains("\"VRM\""));
    assert!(document.contains("\"specVersion\": \"0.0\""));
    assert!(document.contains("\"title\": \"Exported VRM\""));
}

/// VRM0 importでvrm_versionとmetaを復元できることを検証する。
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p3
#[test]
fn vrm0_tc_002_import_should_set_version() {
    let document = r#"
{
  "asset": {
    "version": "2.0"
  },
  "nodes": [
    { "name": "Root" }
  ],
  "extensions": {
    "VRM": {
      "specVersion": "0.0",
      "meta": {
        "title": "Old VRM",
        "version": "0.99",
        "author": "SansaVRM",
        "licenseName": "Redistribution_Prohibited"
      }
    }
  }
}
"#;

    let result = import_vrm(document.into());

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.vrm_version, Some(VrmVersion::V0x));
    assert!(model
        .properties
        .iter()
        .any(|property| property.key == "vrm.meta.name"
            && matches!(
                &property.value,
                sansavrm_core::PropertyValue::String(value) if value == "Old VRM"
            )
        ));

    assert!(model
        .properties
        .iter()
        .any(|property| property.key == "vrm.meta.version"
            && matches!(
                &property.value,
                sansavrm_core::PropertyValue::String(value) if value == "0.99"
            )
        ));

    assert!(model
        .properties
        .iter()
        .any(|property| property.key == "vrm.meta.authors"
            && matches!(
                &property.value,
                sansavrm_core::PropertyValue::String(value) if value == "SansaVRM"
            )
        ));

    assert!(model
        .properties
        .iter()
        .any(|property| {
            property.key == "vrm.meta.license_url"
                && matches!(
                    &property.value,
                    sansavrm_core::PropertyValue::String(value) if value == "Redistribution_Prohibited"
                )
        }));
}

/// VRM0 humanoid importでhuman bonesをProperty化できることを検証する。
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p3
#[test]
fn vrm0_tc_003_import_humanoid_should_create_properties() {
    let document = r#"
{
  "asset": { "version": "2.0" },
  "nodes": [
    { "name": "Hips" },
    { "name": "Head" }
  ],
  "extensions": {
    "VRM": {
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

    let result = import_vrm(document.into());
    assert!(result.success);

    let model = result.data.unwrap();

    assert!(model.properties.iter().any(|p| p.key.contains("hips")));
    assert!(model.properties.iter().any(|p| p.key.contains("head")));
}

/// VRM0 humanoid exportでhumanBones配列を生成できることを検証する。
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p3
#[test]
fn vrm0_tc_004_export_humanoid_should_create_array() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "Hips".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.properties.push(Property::from_typed_value(
        "p_hips",
        "vrm.humanoid.human_bones.hips.node",
        sansavrm_core::PropertyValue::String("Hips".into()),
        PropertyType::Rig,
        PropertyContext::Binding,
    ));

    let result = export_vrm(&model, VrmVersion::V0x, IoOptions::default());
    assert!(result.success);

    let doc = result.data.unwrap();

    assert!(doc.contains("\"humanBones\""));
    assert!(doc.contains("\"bone\""));
}

/// VRM0 importで未知boneを無視できることを検証する。
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p3
#[test]
fn vrm0_tc_005_import_ignore_unknown_bone() {
    let document = r#"
{
  "asset": {
    "version": "2.0"
  },
  "nodes": [
    { "name": "Hips" },
    { "name": "Head" },
    { "name": "UnknownPart" }
  ],
  "extensions": {
    "VRM": {
      "specVersion": "0.0",
      "humanoid": {
        "humanBones": [
          { "bone": "hips", "node": 0 },
          { "bone": "head", "node": 1 },
          { "bone": "customTail", "node": 2 }
        ]
      }
    }
  }
}
"#;

    let result = import_vrm(document.into());

    assert!(result.success);

    let model = result.data.expect("model should be returned");

    assert!(model.properties.iter().any(|property| {
        property.key == "vrm.humanoid.human_bones.hips.node"
            && matches!(
                &property.value,
                sansavrm_core::PropertyValue::String(value) if value == "Hips"
            )
    }));

    assert!(model.properties.iter().any(|property| {
        property.key == "vrm.humanoid.human_bones.head.node"
            && matches!(
                &property.value,
                sansavrm_core::PropertyValue::String(value) if value == "Head"
            )
    }));

    assert!(!model.properties.iter().any(|property| {
        property.key == "vrm.humanoid.human_bones.customTail.node"
    }));
}

/// VRM0 exportで未知boneを無視できることを検証する。
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p3
#[test]
fn vrm0_tc_006_export_ignore_unknown_bone() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "Hips".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.modules.push(Module {
        module_id: "UnknownPart".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.properties.push(Property::from_typed_value(
        "p_hips",
        "vrm.humanoid.human_bones.hips.node",
        sansavrm_core::PropertyValue::String("Hips".into()),
        PropertyType::Rig,
        PropertyContext::Binding,
    ));

    model.properties.push(Property::from_typed_value(
        "p_custom_tail",
        "vrm.humanoid.human_bones.customTail.node",
        sansavrm_core::PropertyValue::String("UnknownPart".into()),
        PropertyType::Rig,
        PropertyContext::Binding,
    ));

    let result = export_vrm(&model, VrmVersion::V0x, IoOptions::default());

    assert!(result.success);

    let document = result.data.expect("document should be returned");

    assert!(document.contains("\"hips\""));
    assert!(!document.contains("\"customTail\""));
}
