// crates/sansavrm-validator/tests/validate_api.rs

use sansavrm_core::{Model, Slot, SlotType};
use sansavrm_validator::{validate, ValidateOptions};

#[test]
fn validator_api_001_validate_empty_model_should_pass() {
    let model = Model::new();

    let result = validate(&model, ValidateOptions::default());

    assert!(result.success);
}

#[test]
fn validator_api_002_validate_invalid_slot_owner_should_fail() {
    let mut model = Model::new();

    model.slots.push(Slot {
        slot_id: "slot_001".into(),
        slot_type: SlotType::Structure,
        owner_module_id: "unknown_module".into(),
        target_slot_types: vec![],
        current_connections: vec![],
        connection_rules: None,
        properties: vec![],
    });

    let result = validate(&model, ValidateOptions::default());

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn validator_api_003_validate_options_default_should_be_strict() {
    let options = ValidateOptions::default();

    assert!(options.strict);
}
