// crates/sansavrm-core/tests/connection_api.rs

use sansavrm_core::{
    connect, disable_connection, disconnect, enable_connection, list_connections, ConnectionType,
    Model, Module, ModuleType, Slot, SlotType,
};

fn base_model() -> Model {
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
fn core_conn_api_001_connect_should_add_connection() {
    let model = base_model();

    let result = connect(model, "slot_001", "slot_002", ConnectionType::Attach);

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.connections.len(), 1);
    assert_eq!(model.connections[0].from_id, "slot_001");
    assert_eq!(model.connections[0].to_id, "slot_002");
    assert_eq!(model.connections[0].connection_type, ConnectionType::Attach);
    assert!(model.connections[0].enabled);
}

#[test]
fn core_conn_api_002_connect_unknown_from_id_should_fail() {
    let model = base_model();

    let result = connect(model, "unknown", "slot_002", ConnectionType::Attach);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn core_conn_api_003_disable_connection_should_set_enabled_false() {
    let model = base_model();
    let result = connect(model, "slot_001", "slot_002", ConnectionType::Attach);
    let model = result.data.expect("model should be returned");
    let connection_id = model.connections[0].connection_id.clone();

    let result = disable_connection(model, &connection_id);

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert!(!model.connections[0].enabled);
}

#[test]
fn core_conn_api_004_enable_connection_should_set_enabled_true() {
    let model = base_model();
    let result = connect(model, "slot_001", "slot_002", ConnectionType::Attach);
    let model = result.data.expect("model should be returned");
    let connection_id = model.connections[0].connection_id.clone();

    let result = disable_connection(model, &connection_id);
    let model = result.data.expect("model should be returned");

    let result = enable_connection(model, &connection_id);

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert!(model.connections[0].enabled);
}

#[test]
fn core_conn_api_005_disconnect_should_remove_connection() {
    let model = base_model();
    let result = connect(model, "slot_001", "slot_002", ConnectionType::Attach);
    let model = result.data.expect("model should be returned");
    let connection_id = model.connections[0].connection_id.clone();

    let result = disconnect(model, &connection_id);

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert!(model.connections.is_empty());
}

#[test]
fn core_conn_api_006_list_connections_should_return_connections() {
    let model = base_model();
    let result = connect(model, "slot_001", "slot_002", ConnectionType::Attach);
    let model = result.data.expect("model should be returned");

    let connections = list_connections(&model);

    assert_eq!(connections.len(), 1);
}
