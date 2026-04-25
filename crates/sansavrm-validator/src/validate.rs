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

    // TODO(trace): Validator実装仕様 / 接続制約検証
    // --- ConnectionRule 最小制約チェック ---
    for slot in &model.slots {
        if let Some(rule) = &slot.connection_rules {
            let connection_count = model
                .connections
                .iter()
                .filter(|connection| {
                    connection.from_slot_id == slot.slot_id || connection.to_slot_id == slot.slot_id
                })
                .count();

            if connection_count < rule.min_connections {
                errors.push(SansaVrmError::InvalidInput(format!(
                    "Slot {} has fewer connections than min_connections {}",
                    slot.slot_id, rule.min_connections
                )));
            }

            if connection_count > rule.max_connections {
                errors.push(SansaVrmError::InvalidInput(format!(
                    "Slot {} exceeds max_connections {}",
                    slot.slot_id, rule.max_connections
                )));
            }

            if rule.exclusive && connection_count > 1 {
                errors.push(SansaVrmError::InvalidInput(format!(
                    "Slot {} is exclusive but has multiple connections",
                    slot.slot_id
                )));
            }
        }
    }

    // TODO(trace): Validator実装仕様 / State参照整合検証
    // --- State Action 参照チェック ---
    for state in &model.states {
        for action in &state.actions {
            match action {
                sansavrm_core::StateAction::ModuleEnable { module_id }
                | sansavrm_core::StateAction::ModuleDisable { module_id } => {
                    if !model.modules.iter().any(|m| &m.module_id == module_id) {
                        errors.push(SansaVrmError::InvalidInput(format!(
                            "State {} references unknown module {}",
                            state.state_id, module_id
                        )));
                    }
                }

                sansavrm_core::StateAction::SlotBind { slot_id, target_slot_id }
                | sansavrm_core::StateAction::SlotUnbind { slot_id, target_slot_id } => {
                    let slot_exists = model.slots.iter().any(|s| &s.slot_id == slot_id);
                    let target_exists = model.slots.iter().any(|s| &s.slot_id == target_slot_id);

                    if !slot_exists {
                        errors.push(SansaVrmError::InvalidInput(format!(
                            "State {} references unknown slot {}",
                            state.state_id, slot_id
                        )));
                    }

                    if !target_exists {
                        errors.push(SansaVrmError::InvalidInput(format!(
                            "State {} references unknown target_slot {}",
                            state.state_id, target_slot_id
                        )));
                    }
                }

                sansavrm_core::StateAction::PropertyOverride { property_id: _, value: _ } => {
                    // 今は未検証（後で property 実装時に追加）
                }

                sansavrm_core::StateAction::VisibilityChange { target_id, .. } => {
                    let exists =
                        model.modules.iter().any(|m| &m.module_id == target_id)
                            || model.slots.iter().any(|s| &s.slot_id == target_id);

                    if !exists {
                        errors.push(SansaVrmError::InvalidInput(format!(
                            "State {} references unknown visibility target {}",
                            state.state_id, target_id
                        )));
                    }
                }
            }
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
