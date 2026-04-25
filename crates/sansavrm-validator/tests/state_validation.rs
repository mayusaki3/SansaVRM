// crates/sansavrm-validator/tests/state_validation.rs

use sansavrm_core::{
    Model, Module, ModuleType, Slot, SlotType, State, StateAction, StateCategory,
};
use sansavrm_validator::validate_model;

fn base_model() -> Model {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "module_001".into(),
        module_type: ModuleType::Module,
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

    model
}

#[test]
fn validator_state_001_valid_module_reference_should_pass() {
    let mut model = base_model();

    model.states.push(State {
        state_id: "state_001".into(),
        category: StateCategory::Expression,
        actions: vec![StateAction::ModuleEnable {
            module_id: "module_001".into(),
        }],
        priority: 0,
        enabled: true,
    });

    let result = validate_model(&model);
    assert!(result.success);
}

#[test]
fn validator_state_002_unknown_module_should_fail() {
    let mut model = base_model();

    model.states.push(State {
        state_id: "state_001".into(),
        category: StateCategory::Expression,
        actions: vec![StateAction::ModuleEnable {
            module_id: "unknown".into(),
        }],
        priority: 0,
        enabled: true,
    });

    let result = validate_model(&model);
    assert!(!result.success);
}

#[test]
fn validator_state_003_unknown_slot_should_fail() {
    let mut model = base_model();

    model.states.push(State {
        state_id: "state_001".into(),
        category: StateCategory::Expression,
        actions: vec![StateAction::SlotBind {
            slot_id: "slot_001".into(),
            target_slot_id: "unknown".into(),
        }],
        priority: 0,
        enabled: true,
    });

    let result = validate_model(&model);
    assert!(!result.success);
}

#[test]
fn validator_state_004_visibility_unknown_target_should_fail() {
    let mut model = base_model();

    model.states.push(State {
        state_id: "state_001".into(),
        category: StateCategory::Expression,
        actions: vec![StateAction::VisibilityChange {
            target_id: "unknown".into(),
            visible: true,
        }],
        priority: 0,
        enabled: true,
    });

    let result = validate_model(&model);
    assert!(!result.success);
}
