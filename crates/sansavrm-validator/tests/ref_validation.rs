// crates/sansavrm-validator/tests/ref_validation.rs

use sansavrm_core::{Model, Slot, SlotType};
use sansavrm_validator::validate_model;

#[test]
fn validator_ref_001_slot_with_unknown_module_should_fail() {
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

    let result = validate_model(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn validator_ref_002_slot_with_existing_module_should_pass() {
    let mut model = Model::new();

    model.modules.push(sansavrm_core::Module {
        module_id: "module_001".into(),
        module_type: sansavrm_core::ModuleType::Module,
        slots: vec!["slot_001".into()],
        properties: vec![],
    });

    model.slots.push(Slot {
        slot_id: "slot_001".into(),
        slot_type: SlotType::Structure,
        owner_module_id: "module_001".into(),
        target_slot_types: vec![],
        current_connections: vec![],
        connection_rules: None,
        properties: vec![],
    });

    let result = validate_model(&model);

    assert!(result.success);
}
