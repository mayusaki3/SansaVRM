// crates/sansavrm-validator/src/validate.rs

use std::collections::HashSet;

use sansavrm_core::{CoreResult, Model, SansaVrmError};

/// Model の基本検証
///
/// TODO(trace): Validator実装仕様 / 基本検証
pub fn validate_model(model: &Model) -> CoreResult<()> {
    let mut errors = Vec::new();

    // TODO(trace): Validator実装仕様 / ID一意性検証
    // --- ID一意性チェック ---
    let mut ids = HashSet::new();

    // model_id
    if !ids.insert(model.model_id.clone()) {
        errors.push(SansaVrmError::DuplicateId(model.model_id.clone()));
    }

    // module_id
    for module in &model.modules {
        if !ids.insert(module.module_id.clone()) {
            errors.push(SansaVrmError::DuplicateId(module.module_id.clone()));
        }
    }

    // slot_id
    for slot in &model.slots {
        if !ids.insert(slot.slot_id.clone()) {
            errors.push(SansaVrmError::DuplicateId(slot.slot_id.clone()));
        }
    }

    // state_id
    for state in &model.states {
        if !ids.insert(state.state_id.clone()) {
            errors.push(SansaVrmError::DuplicateId(state.state_id.clone()));
        }
    }

    // TODO(trace): Validator実装仕様 / 参照整合性検証
    // --- 参照整合チェック ---
    for slot in &model.slots {
        if !model
            .modules
            .iter()
            .any(|module| module.module_id == slot.owner_module_id)
        {
            errors.push(SansaVrmError::InvalidInput(format!(
                "Slot {} references unknown module {}",
                slot.slot_id, slot.owner_module_id
            )));
        }
    }

    // TODO(trace): Validator実装仕様 / 接続整合性検証
    // --- Connection 整合チェック ---
    for connection in &model.connections {
        let from_exists = model
            .slots
            .iter()
            .any(|slot| slot.slot_id == connection.from_slot_id);

        let to_exists = model
            .slots
            .iter()
            .any(|slot| slot.slot_id == connection.to_slot_id);

        if !from_exists {
            errors.push(SansaVrmError::InvalidInput(format!(
                "Connection references unknown from_slot_id {}",
                connection.from_slot_id
            )));
        }

        if !to_exists {
            errors.push(SansaVrmError::InvalidInput(format!(
                "Connection references unknown to_slot_id {}",
                connection.to_slot_id
            )));
        }
    }

    if errors.is_empty() {
        CoreResult::ok(())
    } else {
        CoreResult {
            success: false,
            data: None,
            errors,
            warnings: vec![],
            infos: vec![],
        }
    }
}
