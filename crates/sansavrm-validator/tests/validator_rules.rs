// Validator検証規則テスト
// @hldocs.ref doc-20260504-000403Z-SV0Q#sec_g2c9d4x7
// @hldocs.ref doc-20260504-000403Z-SV0Q#sec_h1e0f3y8
// @hldocs.ref doc-20260504-000403Z-SV0Q#sec_j9g1h2z9

use sansavrm_core::{
    Connection,
    ConnectionType,
    Model,
    Module,
    ModuleType,
    Slot,
    SlotType,
    State,
    StateAction,
    StateCategory,
};
use sansavrm_validator::validate_model;

/// テスト用Moduleを生成する。
///
/// 役割:
/// - Validatorテストで必要な最小構成のModuleを返す。
///
/// 引数:
/// - module_id: Module ID。
///
/// 戻り値:
/// - Module。
fn module(module_id: &str) -> Module {
    Module {
        module_id: module_id.to_string(),
        module_type: ModuleType::Body,
        slots: Vec::new(),
        properties: Vec::new(),
    }
}

/// テスト用Slotを生成する。
///
/// 役割:
/// - Validatorテストで必要な最小構成のSlotを返す。
///
/// 引数:
/// - slot_id: Slot ID。
/// - owner_module_id: 所有元Module ID。
///
/// 戻り値:
/// - Slot。
fn slot(slot_id: &str, owner_module_id: &str) -> Slot {
    Slot {
        slot_id: slot_id.to_string(),
        slot_type: SlotType::Structure,
        owner_module_id: owner_module_id.to_string(),
        target_slot_types: Vec::new(),
        current_connections: Vec::new(),
        connection_rules: None,
        properties: Vec::new(),
    }
}

#[test]
fn validator_ref_001_unknown_slot_owner_should_fail() {
    let mut model = Model::with_id("model_test");
    model.slots.push(slot("slot_a", "missing_module"));

    let result = validate_model(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn validator_conn_002_unknown_connection_target_should_fail() {
    let mut model = Model::with_id("model_test");
    model.modules.push(module("module_a"));
    model.connections.push(Connection {
        connection_id: "connection_a".to_string(),
        from_id: "module_a".to_string(),
        to_id: "missing_module".to_string(),
        connection_type: ConnectionType::Attach,
        enabled: true,
    });

    let result = validate_model(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn validator_state_003_unknown_state_action_target_should_fail() {
    let mut model = Model::with_id("model_test");
    model.states.push(State {
        state_id: "state_a".to_string(),
        category: StateCategory::Configuration,
        actions: vec![StateAction::ModuleEnable {
            module_id: "missing_module".to_string(),
        }],
        priority: 0,
        enabled: true,
    });

    let result = validate_model(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}
