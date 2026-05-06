// crates/sansavrm-core/tests/state_api.rs

use sansavrm_core::{
    add_state, apply_state, evaluate, evaluate_state, remove_state, Connection, ConnectionType,
    Model, Module, ModuleType, Slot, SlotType, State, StateAction, StateCategory,
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

    model.connections.push(Connection {
        connection_id: "connection_001".into(),
        from_id: "slot_001".into(),
        to_id: "slot_002".into(),
        connection_type: ConnectionType::Attach,
        enabled: true,
    });

    model
}

fn connection_disable_state(state_id: &str, connection_id: &str, enabled: bool) -> State {
    State {
        state_id: state_id.into(),
        category: StateCategory::Configuration,
        actions: vec![StateAction::ConnectionDisable {
            connection_id: connection_id.into(),
        }],
        priority: 0,
        enabled,
    }
}

/// Stateを追加できることを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_n5m5q8d3
#[test]
fn core_state_api_001_add_state_should_add_state() {
    let model = base_model();
    let state = connection_disable_state("state_001", "connection_001", true);

    let result = add_state(model, state);

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.states.len(), 1);
}

/// 重複State IDを追加できないことを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_n5m5q8d3
#[test]
fn core_state_api_002_add_duplicate_state_should_fail() {
    let model = base_model();
    let state = connection_disable_state("state_001", "connection_001", true);

    let result = add_state(model, state.clone());
    let model = result.data.expect("model should be returned");

    let result = add_state(model, state);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

/// Stateを削除できることを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_n5m5q8d3
#[test]
fn core_state_api_003_remove_state_should_remove_state() {
    let model = base_model();
    let state = connection_disable_state("state_001", "connection_001", true);

    let result = add_state(model, state);
    let model = result.data.expect("model should be returned");

    let result = remove_state(model, "state_001");

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert!(model.states.is_empty());
}

/// 有効なStateのみを評価結果として返すことを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_t0s8v3j8
#[test]
fn core_state_api_004_evaluate_state_should_return_enabled_states() {
    let model = base_model();

    let result = add_state(
        model,
        connection_disable_state("state_001", "connection_001", true),
    );
    let model = result.data.expect("model should be returned");

    let result = add_state(
        model,
        connection_disable_state("state_002", "connection_001", false),
    );
    let model = result.data.expect("model should be returned");

    let result = evaluate_state(&model);

    assert!(result.success);

    let states = result.data.expect("states should be returned");
    assert_eq!(states.len(), 1);
    assert_eq!(states[0].state_id, "state_001");
}

/// State適用でConnectionを無効化できることを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_n5m5q8d3
#[test]
fn core_state_api_005_apply_state_should_disable_connection() {
    let model = base_model();

    let result = add_state(
        model,
        connection_disable_state("state_001", "connection_001", true),
    );
    let model = result.data.expect("model should be returned");

    let result = apply_state(model, "state_001");

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert!(!model.connections[0].enabled);
}

/// 存在しないStateを適用できないことを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_n5m5q8d3
#[test]
fn core_state_api_006_apply_unknown_state_should_fail() {
    let model = base_model();

    let result = apply_state(model, "unknown_state");

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

/// Model評価結果にactive_states/applied_actions/connection_statusが含まれることを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_t0s8v3j8
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_u9t7w2k9
#[test]
fn core_state_api_007_evaluate_should_return_evaluation_result() {
    let model = base_model();

    let result = add_state(
        model,
        connection_disable_state("state_001", "connection_001", true),
    );
    let model = result.data.expect("model should be returned");

    let result = evaluate(&model);

    assert!(result.success);

    let evaluation = result.data.expect("evaluation should be returned");
    assert_eq!(evaluation.active_states.len(), 1);
    assert_eq!(evaluation.applied_actions.len(), 1);
    assert_eq!(evaluation.connection_status.len(), 1);
}
