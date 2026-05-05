// crates/sansavrm-core/src/connection_api.rs

use crate::{Connection, ConnectionType, CoreResult, Model, SansaId, SansaVrmError};

/// Connection作成入力。
///
/// 役割:
/// - connect_with_input API の入力値を表現する。
///
/// 注意点:
/// - 既存互換の connect API は from_id / to_id / connection_type を直接受け取る。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_m6l4p9c2
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectInput {
    pub connection_id: Option<String>,
    pub from_id: String,
    pub to_id: String,
    pub connection_type: ConnectionType,
    pub enabled: bool,
}

/// ModuleまたはSlotを接続する。
///
/// 役割:
/// - from_id / to_id の存在を確認し、ConnectionをModelへ追加する。
/// - Slotに紐づく接続では current_connections を同期する。
///
/// 引数:
/// - model: 更新対象Model。
/// - from_id: 接続元ID。
/// - to_id: 接続先ID。
/// - connection_type: Connection種別。
///
/// 戻り値:
/// - CoreResult<Model>: 更新後Model、または入力エラー。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_m6l4p9c2
pub fn connect(
    model: Model,
    from_id: impl Into<String>,
    to_id: impl Into<String>,
    connection_type: ConnectionType,
) -> CoreResult<Model> {
    connect_with_input(
        model,
        ConnectInput {
            connection_id: None,
            from_id: from_id.into(),
            to_id: to_id.into(),
            connection_type,
            enabled: true,
        },
    )
}

/// 入力構造体を使用してModuleまたはSlotを接続する。
///
/// 役割:
/// - connection_id や enabled を明示指定したい場合の補助API。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_m6l4p9c2
pub fn connect_with_input(mut model: Model, input: ConnectInput) -> CoreResult<Model> {
    if !connectable_id_exists(&model, &input.from_id) {
        return CoreResult::fail(SansaVrmError::InvalidInput(format!(
            "from_id not found: {}",
            input.from_id
        )));
    }

    if !connectable_id_exists(&model, &input.to_id) {
        return CoreResult::fail(SansaVrmError::InvalidInput(format!(
            "to_id not found: {}",
            input.to_id
        )));
    }

    let connection_id = input
        .connection_id
        .unwrap_or_else(|| SansaId::new("connection").0);

    if model
        .connections
        .iter()
        .any(|connection| connection.connection_id == connection_id)
    {
        return CoreResult::fail(SansaVrmError::InvalidInput(format!(
            "Duplicate connection_id: {}",
            connection_id
        )));
    }

    let connection = Connection {
        connection_id: connection_id.clone(),
        from_id: input.from_id,
        to_id: input.to_id,
        connection_type: input.connection_type,
        enabled: input.enabled,
    };

    sync_slot_connection(&mut model, &connection.from_id, &connection_id);
    sync_slot_connection(&mut model, &connection.to_id, &connection_id);
    model.connections.push(connection);

    CoreResult::ok(model)
}

/// Connectionを削除する。
///
/// 役割:
/// - connection_id に一致するConnectionをModelから削除する。
/// - Slotの current_connections からも対象IDを削除する。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3e1
pub fn disconnect(mut model: Model, connection_id: impl AsRef<str>) -> CoreResult<Model> {
    let connection_id = connection_id.as_ref();
    let old_len = model.connections.len();

    model
        .connections
        .retain(|connection| connection.connection_id != connection_id);

    if model.connections.len() == old_len {
        return CoreResult::fail(SansaVrmError::InvalidInput(format!(
            "Connection not found: {}",
            connection_id
        )));
    }

    for slot in &mut model.slots {
        slot.current_connections
            .retain(|current_id| current_id != connection_id);
    }

    CoreResult::ok(model)
}

/// Connection一覧を取得する。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3e2
pub fn list_connections(model: &Model) -> Vec<Connection> {
    model.connections.clone()
}

/// Connectionを有効化する。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3e3
pub fn enable_connection(mut model: Model, connection_id: impl AsRef<str>) -> CoreResult<Model> {
    set_connection_enabled(&mut model, connection_id.as_ref(), true)
}

/// Connectionを無効化する。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_s1r9u4h7
pub fn disable_connection(mut model: Model, connection_id: impl AsRef<str>) -> CoreResult<Model> {
    set_connection_enabled(&mut model, connection_id.as_ref(), false)
}

/// Connectionの有効状態を変更する。
fn set_connection_enabled(
    model: &mut Model,
    connection_id: &str,
    enabled: bool,
) -> CoreResult<Model> {
    for connection in &mut model.connections {
        if connection.connection_id == connection_id {
            connection.enabled = enabled;
            return CoreResult::ok(model.clone());
        }
    }

    CoreResult::fail(SansaVrmError::InvalidInput(format!(
        "Connection not found: {}",
        connection_id
    )))
}

/// ModuleまたはSlotとして参照可能なIDが存在するか判定する。
fn connectable_id_exists(model: &Model, id: &str) -> bool {
    model.modules.iter().any(|module| module.module_id == id)
        || model.slots.iter().any(|slot| slot.slot_id == id)
}

/// Slotの current_connections にConnection IDを同期する。
fn sync_slot_connection(model: &mut Model, endpoint_id: &str, connection_id: &str) {
    for slot in &mut model.slots {
        if slot.slot_id == endpoint_id
            && !slot
                .current_connections
                .iter()
                .any(|current_id| current_id == connection_id)
        {
            slot.current_connections.push(connection_id.to_string());
        }
    }
}
