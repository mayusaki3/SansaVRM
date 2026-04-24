// crates/sansavrm-core/src/state.rs

use serde::{Deserialize, Serialize};

/// State 種別。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateCategory {
    Expression,
    Configuration,
    Equipment,
    Visibility,
}

/// State Action。
///
/// 役割:
/// - 状態適用時に発生する操作を表現する。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateAction {
    ModuleEnable { module_id: String },
    ModuleDisable { module_id: String },
    SlotBind { slot_id: String, target_slot_id: String },
    SlotUnbind { slot_id: String, target_slot_id: String },
    PropertyOverride { property_id: String, value: String },
    VisibilityChange { target_id: String, visible: bool },
}

/// SansaVRM State。
///
/// 役割:
/// - 条件と Action による状態制御を表現する。
///
/// 注意:
/// - conditions は後続実装で構造化する。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub state_id: String,
    pub category: StateCategory,
    pub actions: Vec<StateAction>,
    pub priority: i32,
    pub enabled: bool,
}
