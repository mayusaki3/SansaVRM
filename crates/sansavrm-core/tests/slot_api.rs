// crates/sansavrm-core/tests/slot_api.rs

use sansavrm_core::{
    add_module, add_slot, remove_slot, update_slot, Connection, ConnectionType, Model, Module,
    ModuleType, Slot, SlotType,
};

fn module(module_id: &str) -> Module {
    Module {
        module_id: module_id.into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    }
}

fn slot(slot_id: &str, owner_module_id: &str, slot_type: SlotType) -> Slot {
    Slot {
        slot_id: slot_id.into(),
        slot_type,
        owner_module_id: owner_module_id.into(),
        target_slot_types: vec![],
        current_connections: vec![],
        connection_rules: None,
        properties: vec![],
    }
}

fn base_model() -> Model {
    let result = add_module(Model::new(), module("module_001"));
    result.data.expect("model should be returned")
}

#[test]
fn core_slot_api_001_add_slot_should_add_slot() {
    let model = base_model();

    let result = add_slot(model, slot("slot_001", "module_001", SlotType::Structure));

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.slots.len(), 1);
    assert_eq!(model.slots[0].slot_id, "slot_001");
    assert_eq!(model.modules[0].slots, vec!["slot_001"]);
}

#[test]
fn core_slot_api_002_add_duplicate_slot_should_fail() {
    let model = base_model();

    let result = add_slot(model, slot("slot_001", "module_001", SlotType::Structure));
    let model = result.data.expect("model should be returned");

    let result = add_slot(model, slot("slot_001", "module_001", SlotType::Equipment));

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn core_slot_api_003_add_slot_unknown_owner_should_fail() {
    let model = base_model();

    let result = add_slot(model, slot("slot_001", "unknown_module", SlotType::Structure));

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn core_slot_api_004_update_slot_should_replace_slot() {
    let model = base_model();

    let result = add_slot(model, slot("slot_001", "module_001", SlotType::Structure));
    let model = result.data.expect("model should be returned");

    let result = update_slot(
        model,
        "slot_001",
        slot("ignored_id", "module_001", SlotType::Equipment),
    );

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.slots[0].slot_id, "slot_001");
    assert_eq!(model.slots[0].slot_type, SlotType::Equipment);
}

#[test]
fn core_slot_api_005_remove_slot_should_remove_slot() {
    let model = base_model();

    let result = add_slot(model, slot("slot_001", "module_001", SlotType::Structure));
    let model = result.data.expect("model should be returned");

    let result = remove_slot(model, "slot_001");

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert!(model.slots.is_empty());
    assert!(model.modules[0].slots.is_empty());
}

#[test]
fn core_slot_api_006_remove_referenced_slot_should_fail() {
    let model = base_model();

    let result = add_slot(model, slot("slot_001", "module_001", SlotType::Structure));
    let mut model = result.data.expect("model should be returned");

    model.connections.push(Connection {
        connection_id: "connection_001".into(),
        from_id: "slot_001".into(),
        to_id: "module_001".into(),
        connection_type: ConnectionType::Attach,
        enabled: true,
    });

    let result = remove_slot(model, "slot_001");

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}
