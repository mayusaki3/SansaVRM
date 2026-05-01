use sansavrm_core::{
    IoOptions, Model, Module, ModuleType, Property, PropertyRole, PropertyType,
    PropertyValueType, VrmVersion,
};
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
      "specVersion": "1.0",
      "meta": {
        "name": "Test VRM",
        "version": "1.0.0",
        "authors": ["SansaVRM"],
        "licenseUrl": "https://example.com/license"
      }
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
    assert_eq!(model.vrm_version, Some(VrmVersion::V1_0));

    assert!(model
        .properties
        .iter()
        .any(|property| property.key == "vrm.meta.name" && property.value == "Test VRM"));

    assert!(model
        .properties
        .iter()
        .any(|property| property.key == "vrm.meta.version" && property.value == "1.0.0"));

    assert!(model
        .properties
        .iter()
        .any(|property| property.key == "vrm.meta.authors" && property.value == "SansaVRM"));

    assert!(model
        .properties
        .iter()
        .any(|property| {
            property.key == "vrm.meta.license_url"
                && property.value == "https://example.com/license"
        }));
}

#[test]
fn vrm_adapter_002_export_vrm_1_0_should_create_gltf_json() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "Root".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.properties.push(Property {
        property_id: "property_vrm_meta_name".into(),
        key: "vrm.meta.name".into(),
        value: "Exported VRM".into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    let result = export_vrm(&model, VrmVersion::V1_0, IoOptions::default());

    assert!(result.success);

    let document = result.data.expect("document should be returned");
    assert!(document.contains("\"version\": \"2.0\""));
    assert!(document.contains("\"name\": \"Root\""));
    assert!(document.contains("\"VRMC_vrm\""));
    assert!(document.contains("\"specVersion\": \"1.0\""));
    assert!(document.contains("\"VRMC_vrm\""));
    assert!(document.contains("\"meta\""));
    assert!(document.contains("\"name\": \"Exported VRM\""));
}

#[test]
fn vrm_adapter_003_import_invalid_json_should_fail() {
    let result = import_vrm("{ invalid json".into());

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn vrm_adapter_004_export_vrm_0x_should_create_gltf_json() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "Root".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.properties.push(Property {
        property_id: "property_vrm_meta_name".into(),
        key: "vrm.meta.name".into(),
        value: "Exported VRM".into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    let result = export_vrm(&model, VrmVersion::V0x, IoOptions::default());

    assert!(result.success);

    let document = result.data.expect("document should be returned");
    assert!(document.contains("\"version\": \"2.0\""));
    assert!(document.contains("\"name\": \"Root\""));
    assert!(document.contains("\"VRM\""));
    assert!(document.contains("\"specVersion\": \"0.0\""));
    assert!(document.contains("\"title\": \"Exported VRM\""));
}

#[test]
fn vrm_adapter_005_import_vrm_0x_should_set_version() {
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
        .any(|property| property.key == "vrm.meta.name" && property.value == "Old VRM"));

    assert!(model
        .properties
        .iter()
        .any(|property| property.key == "vrm.meta.version" && property.value == "0.99"));

    assert!(model
        .properties
        .iter()
        .any(|property| property.key == "vrm.meta.authors" && property.value == "SansaVRM"));

    assert!(model
        .properties
        .iter()
        .any(|property| {
            property.key == "vrm.meta.license_url"
                && property.value == "Redistribution_Prohibited"
        }));
}

#[test]
fn vrm_adapter_006_import_vrm_1_0_humanoid_bones_should_create_properties() {
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
      "specVersion": "1.0",
      "humanoid": {
        "humanBones": {
          "head": {
            "node": 1
          }
        }
      }
    }
  }
}
"#;

    let result = import_vrm(document.into());

    assert!(result.success);

    let model = result.data.expect("model should be returned");

    assert!(model.properties.iter().any(|property| {
        property.key == "vrm.humanoid.human_bones.head.node"
            && property.value == "Head"
    }));
}

#[test]
fn vrm_adapter_007_export_vrm_1_0_humanoid_bones_should_create_human_bones() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "Root".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.modules.push(Module {
        module_id: "Head".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.properties.push(Property {
        property_id: "property_vrm_humanoid_human_bones_head_node".into(),
        key: "vrm.humanoid.human_bones.head.node".into(),
        value: "Head".into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    let result = export_vrm(&model, VrmVersion::V1_0, IoOptions::default());

    assert!(result.success);

    let document = result.data.expect("document should be returned");

    assert!(document.contains("\"humanoid\""));
    assert!(document.contains("\"humanBones\""));
    assert!(document.contains("\"head\""));
    assert!(document.contains("\"node\": 1"));
}

#[test]
fn vrm_adapter_008_import_vrm_1_0_humanoid_should_ignore_unknown_bone() {
    let document = r#"
{
  "asset": {
    "version": "2.0"
  },
  "nodes": [
    { "name": "Root" },
    { "name": "Head" },
    { "name": "UnknownPart" }
  ],
  "extensions": {
    "VRMC_vrm": {
      "specVersion": "1.0",
      "humanoid": {
        "humanBones": {
          "head": {
            "node": 1
          },
          "customTail": {
            "node": 2
          }
        }
      }
    }
  }
}
"#;

    let result = import_vrm(document.into());

    assert!(result.success);

    let model = result.data.expect("model should be returned");

    assert!(model.properties.iter().any(|property| {
        property.key == "vrm.humanoid.human_bones.head.node"
            && property.value == "Head"
    }));

    assert!(!model.properties.iter().any(|property| {
        property.key == "vrm.humanoid.human_bones.customTail.node"
    }));
}

#[test]
fn vrm_adapter_009_export_vrm_1_0_humanoid_should_ignore_unknown_bone_property() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "Head".into(),
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

    model.properties.push(Property {
        property_id: "p_head".into(),
        key: "vrm.humanoid.human_bones.head.node".into(),
        value: "Head".into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    model.properties.push(Property {
        property_id: "p_custom_tail".into(),
        key: "vrm.humanoid.human_bones.customTail.node".into(),
        value: "UnknownPart".into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    let result = export_vrm(&model, VrmVersion::V1_0, IoOptions::default());

    assert!(result.success);

    let document = result.data.expect("document should be returned");

    assert!(document.contains("\"head\""));
    assert!(!document.contains("\"customTail\""));
}

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
