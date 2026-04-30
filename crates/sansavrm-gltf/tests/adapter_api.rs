use sansavrm_core::{Model, Module, ModuleType};
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
fn gltf_adapter_002_export_model_should_create_gltf_json() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "Root".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.modules.push(Module {
        module_id: "Arm".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    let result = export_gltf(&model);

    assert!(result.success);

    let document = result.data.expect("document should be returned");
    assert!(document.contains("\"version\": \"2.0\""));
    assert!(document.contains("\"name\": \"Root\""));
    assert!(document.contains("\"name\": \"Arm\""));
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

#[test]
fn gltf_adapter_005_export_empty_model_should_create_empty_nodes() {
    let model = Model::new();

    let result = export_gltf(&model);

    assert!(result.success);

    let document = result.data.expect("document should be returned");
    assert!(document.contains("\"version\": \"2.0\""));
    assert!(document.contains("\"nodes\": []"));
}
