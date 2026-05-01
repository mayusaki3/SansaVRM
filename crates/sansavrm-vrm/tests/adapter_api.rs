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
