// crates/sansavrm-core/src/connection_api.rs

use crate::{Connection, ConnectionType, CoreResult, Model, SansaId, SansaVrmError};

/// Connection を追加する。
///
/// 役割:
/// - `from_id` / `to_id` / `connection_type` を元に Connection を生成し、Model に追加する。
///
/// 注意点:
/// - `from_id` / `to_id` は Module ID または Slot ID を参照する。
/// - 参照存在チェックは本関数で行う。
/// - 詳細な接続制約検証は Validator 側で行う。
///
/// 引数:
/// - `model`: 更新対象 Model
/// - `from_id`: 接続元 ID
/// - `to_id`: 接続先 ID
/// - `connection_type`: Connection 種別
///
/// 戻り値:
/// - 成功時: Connection 追加後の Model
/// - 失敗時: 参照不明エラー
///
/// TODO(trace): CoreAPI仕様 / connect
pub fn connect(
    mut model: Model,
    from_id: impl Into<String>,
    to_id: impl Into<String>,
    connection_type: ConnectionType,
) -> CoreResult<Model> {
    let from_id = from_id.into();
    let to_id = to_id.into();

    if !id_exists(&model, &from_id) {
        return CoreResult::fail(SansaVrmError::IdNotFound(from_id));
    }

    if !id_exists(&model, &to_id) {
        return CoreResult::fail(SansaVrmError::IdNotFound(to_id));
    }

    model.connections.push(Connection {
        connection_id: SansaId::new("connection").0,
        from_id,
        to_id,
        connection_type,
        enabled: true,
    });

    CoreResult::ok(model)
}

/// Connection を削除する。
///
/// 役割:
/// - `connection_id` に一致する Connection を Model から削除する。
///
/// 引数:
/// - `model`: 更新対象 Model
/// - `connection_id`: 削除対象 Connection ID
///
/// 戻り値:
/// - 成功時: Connection 削除後の Model
/// - 失敗時: ID不明エラー
///
/// TODO(trace): CoreAPI仕様 / disconnect
pub fn disconnect(mut model: Model, connection_id: impl AsRef<str>) -> CoreResult<Model> {
    let connection_id = connection_id.as_ref();

    let before_len = model.connections.len();
    model
        .connections
        .retain(|connection| connection.connection_id != connection_id);

    if model.connections.len() == before_len {
        return CoreResult::fail(SansaVrmError::IdNotFound(connection_id.to_string()));
    }

    CoreResult::ok(model)
}

/// Connection 一覧を取得する。
///
/// 役割:
/// - Model に含まれる Connection を返す。
///
/// 引数:
/// - `model`: 参照対象 Model
///
/// 戻り値:
/// - Connection 一覧
///
/// TODO(trace): CoreAPI仕様 / list_connections
pub fn list_connections(model: &Model) -> Vec<Connection> {
    model.connections.clone()
}

/// Connection を有効化する。
///
/// 役割:
/// - `connection_id` に一致する Connection の `enabled` を true にする。
///
/// TODO(trace): CoreAPI仕様 / enable_connection
pub fn enable_connection(mut model: Model, connection_id: impl AsRef<str>) -> CoreResult<Model> {
    set_connection_enabled(&mut model, connection_id.as_ref(), true)
}

/// Connection を無効化する。
///
/// 役割:
/// - `connection_id` に一致する Connection の `enabled` を false にする。
///
/// TODO(trace): CoreAPI仕様 / disable_connection
pub fn disable_connection(mut model: Model, connection_id: impl AsRef<str>) -> CoreResult<Model> {
    set_connection_enabled(&mut model, connection_id.as_ref(), false)
}

/// Connection の enabled を変更する。
///
/// 役割:
/// - enable / disable の共通処理を提供する。
fn set_connection_enabled(
    model: &mut Model,
    connection_id: &str,
    enabled: bool,
) -> CoreResult<Model> {
    if let Some(connection) = model
        .connections
        .iter_mut()
        .find(|connection| connection.connection_id == connection_id)
    {
        connection.enabled = enabled;
        return CoreResult::ok(model.clone());
    }

    CoreResult::fail(SansaVrmError::IdNotFound(connection_id.to_string()))
}

/// ID が Model 内に存在するか確認する。
///
/// 役割:
/// - Module ID / Slot ID を接続対象として許可する。
fn id_exists(model: &Model, id: &str) -> bool {
    model.modules.iter().any(|module| module.module_id == id)
        || model.slots.iter().any(|slot| slot.slot_id == id)
}
