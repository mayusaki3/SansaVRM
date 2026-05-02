// crates/sansavrm-core/src/property.rs

use serde::{Deserialize, Serialize};

/// SansaVRM Property の型付き値。
///
/// 役割:
/// - SansaVRM Property の値を型安全に表現する。
///
/// 注意:
/// - String / Number / Bool を enum として保持する。
/// - JSON互換形式は別途 serde 方針で定義する。
///
/// TODO(trace): CoreAPI仕様 / Property typed value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Bool(bool),
}

impl PropertyValue {
    /// SansaVRM PropertyValue を表示・ログ・互換処理用の文字列に変換する。
    ///
    /// 戻り値:
    /// - `String`: PropertyValue の文字列表現
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
    /// - `PropertyValue` を用いて Property の値を型安全に保持する。
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
        context: PropertyContext,
    ) -> Self {
        Self {
            property_id: property_id.into(),
            key: key.into(),
            value,
            property_type,
            context,
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
    Metadata,
    Physics,
    Geometry,
    Material,
    Texture,
    Rig,
    Animation,
    Expression,
    Control,
    Sensor,
    Actuator,
    Constraint,
    Compatibility,
    Rights,
    Revenue,
    Custom,
}

/// Property の役割。
///
/// 役割:
/// - Property がどの用途・文脈で使われるかを表現する。
///
/// TODO(trace): JSONスキーマ仕様 / Property.context
/// TODO(trace): 物理・制御メタモデル仕様 / Property の役割
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyContext {
    Description,
    Simulation,
    Rendering,
    IO,
    Validation,
    Conversion,
    Execution,
    Binding,
    Authoring,
    Custom,
}

/// SansaVRM Property。
///
/// 役割:
/// - Module / Slot / Rights / Revenue 等に付与される属性を表現する。
///
/// 注意:
/// - value は PropertyValue として型付き値を保持する。
/// - JSON表現は serde tagged enum 形式を使用する。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property {
    pub property_id: String,
    pub key: String,
    pub value: PropertyValue,
    pub property_type: PropertyType,
    pub context: PropertyContext,
}
