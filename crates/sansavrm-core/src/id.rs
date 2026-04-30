// crates/sansavrm-core/src/id.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// SansaVRM の汎用 ID。
///
/// 役割:
/// - Model / Module / Slot / State などの識別子として使用する。
///
/// 注意:
/// - 現段階では UUID 由来の安定しない ID 生成を提供する。
/// - 変換時の安定 ID 生成は Adapter 側で実装する。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SansaId(pub String);

impl SansaId {
    /// 新しいランダム ID を生成する。
    ///
    /// 戻り値:
    /// - `SansaId`: `prefix_uuid` 形式の ID
    pub fn new(prefix: &str) -> Self {
        Self(format!("{}_{}", prefix, Uuid::new_v4()))
    }

    /// 既存文字列から ID を作成する。
    ///
    /// 引数:
    /// - `value`: ID文字列
    ///
    /// 戻り値:
    /// - `SansaId`
    pub fn from_string(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}
