// crates/sansavrm-validator/tests/connection_validation.rs

use sansavrm_core::{Connection, ConnectionType, Model, Module, ModuleType, Slot, SlotType};
use sansavrm_validator::validate_model;

fn build_model_with_two_slots() -> Model {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "module_001".into(),
        module_type: ModuleType::Module,
        slots: vec!["slot_001".into(), "slot_002".into()],
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

    model.slots.push(Slot {
        slot_id: "slot_002".into(),
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
fn validator_conn_001_connection_with_existing_slots_should_pass() {
    let mut model = build_model_with_two_slots();

    model.connections.push(Connection {
        connection_id: "connection_001".into(),
        from_id: "slot_001".into(),
        to_id: "slot_002".into(),
        connection_type: ConnectionType::Attach,
        enabled: true,
    });

    let result = validate_model(&model);

    assert!(result.success);
}

#[test]
fn validator_conn_002_connection_with_unknown_from_id_should_fail() {
    let mut model = build_model_with_two_slots();

    model.connections.push(Connection {
        connection_id: "connection_001".into(),
        from_id: "unknown_slot".into(),
        to_id: "slot_002".into(),
        connection_type: ConnectionType::Attach,
        enabled: true,
    });

    let result = validate_model(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn validator_conn_003_connection_with_unknown_to_id_should_fail() {
    let mut model = build_model_with_two_slots();

    model.connections.push(Connection {
        connection_id: "connection_001".into(),
        from_id: "slot_001".into(),
        to_id: "unknown_slot".into(),
        connection_type: ConnectionType::Attach,
        enabled: true,
    });

    let result = validate_model(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}
