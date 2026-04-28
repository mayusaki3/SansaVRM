// crates/sansavrm-core/src/property.rs

use serde::{Deserialize, Serialize};

/// Property の値型。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyValueType {
    String,
    Number,
    Boolean,
    Object,
    Array,
}

/// Property の分類。
///
/// 役割:
/// - 変換処理および Validator における Property の用途判定に使用する。
///
/// 注意点:
/// - `property_type -> role -> key` の順で分類判定する。
///
/// TODO(trace): JSONスキーマ仕様 / Property.property_type
/// TODO(trace): MuJoCo連携仕様 / Property分類ルール
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyType {
    Physics,
    Collision,
    Visual,
    Control,
    Actuator,
    Sensor,
    Metadata,
    Custom,
}

/// Property の役割。
///
/// 役割:
/// - Property がどの用途・文脈で使われるかを表現する。
///
/// TODO(trace): JSONスキーマ仕様 / Property.role
/// TODO(trace): 物理・制御メタモデル仕様 / Property の役割
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyRole {
    Module,
    Slot,
    Physics,
    Control,
    Actuator,
    Sensor,
    Interface,
    Constraint,
    Custom,
}

/// SansaVRM Property。
///
/// 役割:
/// - Module / Slot / Rights / Revenue 等に付与される属性を表現する。
///
/// 注意:
/// - value は初期実装では String 表現に留める。
/// - JSON値対応は serde_json 導入時に拡張する。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Property {
    pub property_id: String,
    pub key: String,
    pub value: String,
    pub value_type: PropertyValueType,
    pub property_type: PropertyType,
    pub role: PropertyRole,
}
