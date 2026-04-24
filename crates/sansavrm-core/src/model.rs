// crates/sansavrm-core/src/model.rs

use serde::{Deserialize, Serialize};

use crate::{DiagnosticItem, Module, SansaId, Slot, State};

/// Slot 間接続。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Connection {
    pub from_slot_id: String,
    pub to_slot_id: String,
    pub connection_type: String,
}

/// SansaVRM Model。
///
/// 役割:
/// - SansaVRM Core のルートモデルを表現する。
///
/// TODO(trace): メタモデル仕様 / Model に対応
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Model {
    pub model_id: String,
    pub modules: Vec<Module>,
    pub slots: Vec<Slot>,
    pub states: Vec<State>,
    pub connections: Vec<Connection>,
    pub diagnostics: Vec<DiagnosticItem>,
}

impl Model {
    /// 空の Model を作成する。
    ///
    /// 戻り値:
    /// - `Model`: 初期化済みの空モデル
    ///
    /// TODO(trace): CoreAPI仕様 / create_model に対応
    pub fn new() -> Self {
        Self {
            model_id: SansaId::new("model").0,
            modules: Vec::new(),
            slots: Vec::new(),
            states: Vec::new(),
            connections: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    /// 指定IDで空の Model を作成する。
    ///
    /// 引数:
    /// - `model_id`: 使用する Model ID
    ///
    /// 戻り値:
    /// - `Model`
    pub fn with_id(model_id: impl Into<String>) -> Self {
        Self {
            model_id: model_id.into(),
            modules: Vec::new(),
            slots: Vec::new(),
            states: Vec::new(),
            connections: Vec::new(),
            diagnostics: Vec::new(),
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Model;

    #[test]
    fn model_new_creates_empty_model() {
        let model = Model::new();

        assert!(model.model_id.starts_with("model_"));
        assert!(model.modules.is_empty());
        assert!(model.slots.is_empty());
        assert!(model.states.is_empty());
        assert!(model.connections.is_empty());
        assert!(model.diagnostics.is_empty());
    }

    #[test]
    fn model_with_id_uses_given_id() {
        let model = Model::with_id("model_test");

        assert_eq!(model.model_id, "model_test");
    }
}
