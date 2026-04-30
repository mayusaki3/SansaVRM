// crates/sansavrm-core/src/module.rs

use serde::{Deserialize, Serialize};

use crate::Property;

/// Module 種別。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleType {
    Body,
    Clothing,
    Accessory,
    Equipment,
    Prop,
    Module,
    WorldObject,
    CompositeRoot,
    Custom,
}

/// SansaVRM Module。
///
/// 役割:
/// - モデルを構成する論理的な部品を表現する。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Module {
    pub module_id: String,
    pub module_type: ModuleType,
    pub slots: Vec<String>,
    pub properties: Vec<Property>,
}
