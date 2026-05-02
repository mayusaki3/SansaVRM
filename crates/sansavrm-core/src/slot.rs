// crates/sansavrm-core/src/slot.rs

use serde::{Deserialize, Serialize};

use crate::Property;

/// Slot 種別。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SlotType {
    Structure,
    Equipment,
    State,
    Rights,
    Revenue,
    Physics,
    Control,
    Sensor,
    Actuator,
    Compatibility,
    SemanticTag,
    Morph,
    Animation,
    Custom,
}

/// Slot 接続制約。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionRule {
    pub min_connections: usize,
    pub max_connections: usize,
    pub exclusive: bool,
    pub replace_mode: String,
    pub priority: i32,
}

/// SansaVRM Slot。
///
/// 役割:
/// - Module 間の接続点を表現する。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slot {
    pub slot_id: String,
    pub slot_type: SlotType,
    pub owner_module_id: String,
    pub target_slot_types: Vec<SlotType>,
    pub current_connections: Vec<String>,
    pub connection_rules: Option<ConnectionRule>,
    pub properties: Vec<Property>,
}
