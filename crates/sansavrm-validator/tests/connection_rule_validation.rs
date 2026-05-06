// crates/sansavrm-validator/tests/connection_rule_validation.rs

use sansavrm_core::{
    Connection, ConnectionRule, ConnectionType, Model, Module, ModuleType, Slot, SlotType,
};
use sansavrm_validator::validate_model;

fn build_model_with_rule(max_connections: usize, exclusive: bool) -> Model {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "module_001".into(),
        module_type: ModuleType::Module,
        slots: vec!["slot_001".into(), "slot_002".into(), "slot_003".into()],
        properties: vec![],
    });

    model.slots.push(Slot {
        slot_id: "slot_001".into(),
        slot_type: SlotType::Structure,
        owner_module_id: "module_001".into(),
        target_slot_types: vec![],
        current_connections: vec![],
        connection_rules: Some(ConnectionRule {
            min_connections: 0,
            max_connections,
            exclusive,
            replace_mode: "replace".into(),
            priority: 0,
        }),
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

    model.slots.push(Slot {
        slot_id: "slot_003".into(),
        slot_type: SlotType::Structure,
        owner_module_id: "module_001".into(),
        target_slot_types: vec![],
        current_connections: vec![],
        connection_rules: None,
        properties: vec![],
    });

    model
}

/// ConnectionRuleの最大接続数内であれば成功することを検証する。
/// @hldocs.ref doc-20260504-000403Z-SV0Q#sec_d0c8f3u8
#[test]
fn validator_connrule_001_connection_within_max_should_pass() {
    let mut model = build_model_with_rule(1, false);

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

/// ConnectionRuleの最大接続数超過を検出することを検証する。
/// @hldocs.ref doc-20260504-000403Z-SV0Q#sec_d0c8f3u8
#[test]
fn validator_connrule_002_max_connections_exceeded_should_fail() {
    let mut model = build_model_with_rule(1, false);

    model.connections.push(Connection {
        connection_id: "connection_001".into(),
        from_id: "slot_001".into(),
        to_id: "slot_002".into(),
        connection_type: ConnectionType::Attach,
        enabled: true,
    });

    model.connections.push(Connection {
        connection_id: "connection_002".into(),
        from_id: "slot_001".into(),
        to_id: "slot_003".into(),
        connection_type: ConnectionType::Attach,
        enabled: true,
    });

    let result = validate_model(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

/// exclusive指定Slotへの複数接続を検出することを検証する。
/// @hldocs.ref doc-20260504-000403Z-SV0Q#sec_d0c8f3u8
#[test]
fn validator_connrule_003_exclusive_multiple_connections_should_fail() {
    let mut model = build_model_with_rule(10, true);

    model.connections.push(Connection {
        connection_id: "connection_001".into(),
        from_id: "slot_001".into(),
        to_id: "slot_002".into(),
        connection_type: ConnectionType::Attach,
        enabled: true,
    });

    model.connections.push(Connection {
        connection_id: "connection_002".into(),
        from_id: "slot_001".into(),
        to_id: "slot_003".into(),
        connection_type: ConnectionType::Attach,
        enabled: true,
    });

    let result = validate_model(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}
