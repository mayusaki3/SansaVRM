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
}
