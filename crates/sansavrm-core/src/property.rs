// crates/sansavrm-core/src/property.rs

use serde::{Deserialize, Serialize};

/// SansaVRM Property の型付き値。
///
/// 役割:
/// - 既存の String value / value_type 構成から、型安全な PropertyValue へ段階移行するための中間表現。
///
/// 注意:
/// - 現段階では Property 本体の value:String は維持する。
/// - 完全移行前に adapter / validator / tests を順次対応する。
///
/// TODO(trace): CoreAPI仕様 / Property typed value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Bool(bool),
}

impl PropertyValue {
    /// SansaVRM PropertyValue を既存 Property.value 用の String に変換する。
    ///
    /// 戻り値:
    /// - `String`: 既存 Property.value に格納可能な文字列表現
    pub fn to_legacy_string(&self) -> String {
        match self {
            PropertyValue::String(value) => value.clone(),
            PropertyValue::Number(value) => value.to_string(),
            PropertyValue::Bool(value) => value.to_string(),
        }
    }

    /// SansaVRM PropertyValue を文字列として参照する。
    ///
    /// 戻り値:
    /// - `Some(&str)`: String値の場合
    /// - `None`: Number / Bool の場合
    pub fn as_string(&self) -> Option<&str> {
        match self {
            PropertyValue::String(value) => Some(value.as_str()),
            _ => None,
        }
    }
}

impl Property {
    /// SansaVRM Property を型付き値から作成する。
    ///
    /// 役割:
    /// - 既存の `value: String` / `value_type: PropertyValueType` を維持しながら、
    ///   呼び出し側では `PropertyValue` を使えるようにする。
    ///
    /// 引数:
    /// - `property_id`: Property ID
    /// - `key`: Property key
    /// - `value`: 型付き Property value
    /// - `property_type`: Property分類
    /// - `role`: Property役割
    ///
    /// 戻り値:
    /// - `Property`: 既存構造互換の Property
    ///
    /// TODO(trace): CoreAPI仕様 / Property typed value
    pub fn from_typed_value(
        property_id: impl Into<String>,
        key: impl Into<String>,
        value: PropertyValue,
        property_type: PropertyType,
        role: PropertyRole,
    ) -> Self {
        Self {
            property_id: property_id.into(),
            key: key.into(),
            value,
            property_type,
            role,
        }
    }
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property {
    pub property_id: String,
    pub key: String,
    pub value: PropertyValue,
    pub property_type: PropertyType,
    pub role: PropertyRole,
}
