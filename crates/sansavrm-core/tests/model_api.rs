// crates/sansavrm-core/tests/model_api.rs

use sansavrm_core::{create_model, export_model, load_model, CreateModelInput};

#[test]
fn core_model_api_001_create_model_without_id_should_generate_id() {
    let result = create_model(CreateModelInput::default());

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert!(model.model_id.starts_with("model_"));
    assert!(model.modules.is_empty());
    assert!(model.slots.is_empty());
    assert!(model.states.is_empty());
    assert!(model.connections.is_empty());
}

#[test]
fn core_model_api_002_create_model_with_id_should_use_given_id() {
    let result = create_model(CreateModelInput {
        model_id: Some("model_test".into()),
    });

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.model_id, "model_test");
}

#[test]
fn core_model_api_003_export_model_should_return_json() {
    let result = create_model(CreateModelInput {
        model_id: Some("model_test".into()),
    });
    let model = result.data.expect("model should be returned");

    let result = export_model(&model);

    assert!(result.success);

    let document = result.data.expect("document should be returned");
    assert!(document.contains("\"model_id\": \"model_test\""));
}

#[test]
fn core_model_api_004_load_model_should_parse_json() {
    let document = r#"
{
  "model_id": "model_test",
  "modules": [],
  "slots": [],
  "states": [],
  "connections": [],
  "diagnostics": []
}
"#;

    let result = load_model(document);

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.model_id, "model_test");
}

#[test]
fn core_model_api_005_load_model_invalid_json_should_fail() {
    let result = load_model("{ invalid json");

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}
