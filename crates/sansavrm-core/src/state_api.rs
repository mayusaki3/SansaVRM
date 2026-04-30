// crates/sansavrm-core/src/state_api.rs

use crate::{CoreResult, Model, SansaVrmError, State, StateAction};

/// Evaluate 結果。
///
/// 役割:
/// - CoreAPI仕様の EvaluationResult に対応する初期実装。
///
/// 注意点:
/// - compatibility_results は後続実装で追加する。
///
/// TODO(trace): CoreAPI仕様 / EvaluationResult
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationResult {
    pub active_states: Vec<State>,
    pub applied_actions: Vec<StateAction>,
    pub connection_status: Vec<ConnectionStatus>,
}

/// Connection 状態。
///
/// TODO(trace): CoreAPI仕様 / EvaluationResult.connection_status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionStatus {
    pub connection_id: String,
    pub enabled: bool,
}

/// State を追加する。
///
/// TODO(trace): CoreAPI仕様 / add_state
pub fn add_state(mut model: Model, state: State) -> CoreResult<Model> {
    if model.states.iter().any(|s| s.state_id == state.state_id) {
        return CoreResult::fail(SansaVrmError::DuplicateId(state.state_id));
    }

    model.states.push(state);
    CoreResult::ok(model)
}

/// State を削除する。
///
/// TODO(trace): CoreAPI仕様 / remove_state
pub fn remove_state(mut model: Model, state_id: impl AsRef<str>) -> CoreResult<Model> {
    let state_id = state_id.as_ref();

    let before_len = model.states.len();
    model.states.retain(|state| state.state_id != state_id);

    if model.states.len() == before_len {
        return CoreResult::fail(SansaVrmError::IdNotFound(state_id.to_string()));
    }

    CoreResult::ok(model)
}

/// State を評価する。
///
/// 役割:
/// - 初期実装では `enabled = true` の State を active とする。
///
/// TODO(trace): CoreAPI仕様 / evaluate_state
pub fn evaluate_state(model: &Model) -> CoreResult<Vec<State>> {
    let active_states = model
        .states
        .iter()
        .filter(|state| state.enabled)
        .cloned()
        .collect();

    CoreResult::ok(active_states)
}

/// State を適用する。
///
/// 役割:
/// - 指定 State の Action を Model に反映する。
///
/// 注意点:
/// - 初期実装では ConnectionEnable / ConnectionDisable のみ実処理する。
/// - ModuleEnable / ModuleDisable / SlotBind / SlotUnbind / PropertyOverride / VisibilityChange は後続実装。
///
/// TODO(trace): CoreAPI仕様 / apply_state
pub fn apply_state(mut model: Model, state_id: impl AsRef<str>) -> CoreResult<Model> {
    let state_id = state_id.as_ref();

    let state = match model.states.iter().find(|state| state.state_id == state_id) {
        Some(state) => state.clone(),
        None => return CoreResult::fail(SansaVrmError::IdNotFound(state_id.to_string())),
    };

    for action in state.actions {
        match action {
            StateAction::ConnectionEnable { connection_id } => {
                if let Err(error) = set_connection_enabled(&mut model, &connection_id, true) {
                    return CoreResult::fail(error);
                }
            }

            StateAction::ConnectionDisable { connection_id } => {
                if let Err(error) = set_connection_enabled(&mut model, &connection_id, false) {
                    return CoreResult::fail(error);
                }
            }

            _ => {
                // 後続実装対象。
            }
        }
    }

    CoreResult::ok(model)
}

/// Model を評価する。
///
/// 役割:
/// - active_states / applied_actions / connection_status を返す。
///
/// 注意点:
/// - 初期実装では State の適用は行わず、評価結果のみ生成する。
///
/// TODO(trace): CoreAPI仕様 / evaluate
pub fn evaluate(model: &Model) -> CoreResult<EvaluationResult> {
    let active_states: Vec<State> = model
        .states
        .iter()
        .filter(|state| state.enabled)
        .cloned()
        .collect();

    let applied_actions = active_states
        .iter()
        .flat_map(|state| state.actions.clone())
        .collect();

    let connection_status = model
        .connections
        .iter()
        .map(|connection| ConnectionStatus {
            connection_id: connection.connection_id.clone(),
            enabled: connection.enabled,
        })
        .collect();

    CoreResult::ok(EvaluationResult {
        active_states,
        applied_actions,
        connection_status,
    })
}

/// Connection enabled を変更する。
fn set_connection_enabled(
    model: &mut Model,
    connection_id: &str,
    enabled: bool,
) -> Result<(), SansaVrmError> {
    let connection = model
        .connections
        .iter_mut()
        .find(|connection| connection.connection_id == connection_id)
        .ok_or_else(|| SansaVrmError::IdNotFound(connection_id.to_string()))?;

    connection.enabled = enabled;
    Ok(())
}
