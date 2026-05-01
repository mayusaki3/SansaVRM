use sansavrm_core::{
    IoOptions, Model, Module, ModuleType, Property, PropertyRole, PropertyType,
    PropertyValueType, VrmVersion,
};
use sansavrm_vrm::{export_vrm, import_vrm};

#[test]
fn vrm1_tc_001_import_minimal_should_create_model() {
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
fn vrm1_tc_002_export_should_create_gltf_json() {
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
fn vrm1_tc_003_import_humanoid_should_create_properties() {
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
fn vrm1_tc_004_export_humanoid_should_create_human_bones() {
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
fn vrm1_tc_005_import_ignore_unknown_bone() {
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
fn vrm1_tc_006_export_ignore_unknown_bone() {
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
