// crates/sansavrm-validator/src/validate.rs

use std::collections::HashSet;

use sansavrm_core::{CoreResult, Model, SansaVrmError};

/// Model の基本検証
///
/// TODO(trace): Validator実装仕様 / 基本検証
pub fn validate_model(model: &Model) -> CoreResult<()> {
    let mut errors = Vec::new();

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
