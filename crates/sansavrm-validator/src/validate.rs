// crates/sansavrm-validator/src/validate.rs

use std::collections::HashSet;

use sansavrm_core::{
    CoreResult, DiagnosticCode, DiagnosticSeverity, Model, SansaVrmError, ValidationDiagnostic,
};

use crate::ValidatorResult;

/// Model の基本検証
///
/// TODO(trace): Validator実装仕様 / 基本検証
pub fn validate_model(model: &Model) -> CoreResult<()> {
    let mut errors = Vec::new();

    validate_unique_ids(model, &mut errors);
    validate_slot_owner_refs(model, &mut errors);
    validate_connections(model, &mut errors);
    validate_connection_rules(model, &mut errors);
    validate_state_actions(model, &mut errors);
    validate_properties(model, &mut errors);

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

/// Model の diagnostics 付き基本検証
///
/// TODO(trace): Validator実装仕様 / diagnostics出力
pub fn validate_model_with_diagnostics(model: &Model) -> ValidatorResult {
    let mut diagnostics = Vec::new();

    validate_unique_ids_with_diagnostics(model, &mut diagnostics);
    validate_slot_owner_refs_with_diagnostics(model, &mut diagnostics);

    ValidatorResult {
        success: diagnostics.is_empty(),
        diagnostics,
    }
}

/// ID 一意性検証
///
/// TODO(trace): Validator実装仕様 / ID一意性検証
fn validate_unique_ids(model: &Model, errors: &mut Vec<SansaVrmError>) {
    let mut ids = HashSet::new();

    if !ids.insert(model.model_id.clone()) {
        errors.push(SansaVrmError::DuplicateId(model.model_id.clone()));
    }

    for module in &model.modules {
        if !ids.insert(module.module_id.clone()) {
            errors.push(SansaVrmError::DuplicateId(module.module_id.clone()));
        }
    }

    for slot in &model.slots {
        if !ids.insert(slot.slot_id.clone()) {
            errors.push(SansaVrmError::DuplicateId(slot.slot_id.clone()));
        }
    }

    for state in &model.states {
        if !ids.insert(state.state_id.clone()) {
            errors.push(SansaVrmError::DuplicateId(state.state_id.clone()));
        }
    }

    for connection in &model.connections {
        if !ids.insert(connection.connection_id.clone()) {
            errors.push(SansaVrmError::DuplicateId(connection.connection_id.clone()));
        }
    }
}

/// ID 一意性 diagnostics 検証
///
/// TODO(trace): Validator実装仕様 / diagnostics出力
fn validate_unique_ids_with_diagnostics(
    model: &Model,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    let mut ids = HashSet::new();

    if !ids.insert(model.model_id.clone()) {
        diagnostics.push(ValidationDiagnostic {
            code: DiagnosticCode::DuplicateId,
            severity: DiagnosticSeverity::Error,
            message: format!("Duplicate ID: {}", model.model_id),
            target: Some(model.model_id.clone()),
        });
    }

    for module in &model.modules {
        if !ids.insert(module.module_id.clone()) {
            diagnostics.push(ValidationDiagnostic {
                code: DiagnosticCode::DuplicateId,
                severity: DiagnosticSeverity::Error,
                message: format!("Duplicate ID: {}", module.module_id),
                target: Some(module.module_id.clone()),
            });
        }
    }

    for slot in &model.slots {
        if !ids.insert(slot.slot_id.clone()) {
            diagnostics.push(ValidationDiagnostic {
                code: DiagnosticCode::DuplicateId,
                severity: DiagnosticSeverity::Error,
                message: format!("Duplicate ID: {}", slot.slot_id),
                target: Some(slot.slot_id.clone()),
            });
        }
    }

    for state in &model.states {
        if !ids.insert(state.state_id.clone()) {
            diagnostics.push(ValidationDiagnostic {
                code: DiagnosticCode::DuplicateId,
                severity: DiagnosticSeverity::Error,
                message: format!("Duplicate ID: {}", state.state_id),
                target: Some(state.state_id.clone()),
            });
        }
    }
}

/// Slot の owner_module_id 参照整合性検証
///
/// TODO(trace): Validator実装仕様 / 参照整合性検証
fn validate_slot_owner_refs(model: &Model, errors: &mut Vec<SansaVrmError>) {
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
}

/// Slot の owner_module_id 参照整合性 diagnostics 検証
///
/// TODO(trace): Validator実装仕様 / diagnostics出力
fn validate_slot_owner_refs_with_diagnostics(
    model: &Model,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    for slot in &model.slots {
        if !model
            .modules
            .iter()
            .any(|module| module.module_id == slot.owner_module_id)
        {
            diagnostics.push(ValidationDiagnostic {
                code: DiagnosticCode::RefNotFound,
                severity: DiagnosticSeverity::Error,
                message: format!(
                    "Slot {} references unknown module {}",
                    slot.slot_id, slot.owner_module_id
                ),
                target: Some(slot.slot_id.clone()),
            });
        }
    }
}

/// Connection の参照整合性検証
///
/// TODO(trace): Validator実装仕様 / 接続整合性検証
fn validate_connections(model: &Model, errors: &mut Vec<SansaVrmError>) {
    for connection in &model.connections {
        let from_exists =
            model.modules.iter().any(|module| module.module_id == connection.from_id)
            || model.slots.iter().any(|slot| slot.slot_id == connection.from_id);

        let to_exists =
            model.modules.iter().any(|module| module.module_id == connection.to_id)
            || model.slots.iter().any(|slot| slot.slot_id == connection.to_id);

        if !from_exists {
            errors.push(SansaVrmError::InvalidInput(format!(
                "Connection {} references unknown from_id {}",
                connection.connection_id,
                connection.from_id
            )));
        }

        if !to_exists {
            errors.push(SansaVrmError::InvalidInput(format!(
                "Connection {} references unknown to_id {}",
                connection.connection_id,
                connection.to_id
            )));
        }
    }
}

/// ConnectionRule の最小制約検証
///
/// TODO(trace): Validator実装仕様 / 接続制約検証
fn validate_connection_rules(model: &Model, errors: &mut Vec<SansaVrmError>) {
    for slot in &model.slots {
        if let Some(rule) = &slot.connection_rules {
            let connection_count = model
                .connections
                .iter()
                .filter(|connection| {
                    connection.from_id == slot.slot_id || connection.to_id == slot.slot_id
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
}

/// StateAction の参照整合性検証
///
/// TODO(trace): Validator実装仕様 / State参照整合検証
fn validate_state_actions(model: &Model, errors: &mut Vec<SansaVrmError>) {
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
                    // 今は未検証。Property参照モデル確定後に追加する。
                }

                sansavrm_core::StateAction::VisibilityChange { target_id, .. } => {
                    let exists = model.modules.iter().any(|m| &m.module_id == target_id)
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
}

/// Model 内の Property 検証
///
/// TODO(trace): Validator実装仕様 / Property整合性検証
fn validate_properties(model: &Model, errors: &mut Vec<SansaVrmError>) {
    for module in &model.modules {
        for property in &module.properties {
            validate_property_value(property, errors);
        }
    }

    for slot in &model.slots {
        for property in &slot.properties {
            validate_property_value(property, errors);
        }
    }
}

/// Property の値整合性検証
///
/// TODO(trace): Validator実装仕様 / Property整合性検証
fn validate_property_value(
    property: &sansavrm_core::Property,
    errors: &mut Vec<SansaVrmError>,
) {
    match property.value_type {
        sansavrm_core::PropertyValueType::String => {}

        sansavrm_core::PropertyValueType::Number => {
            if property.value.parse::<f64>().is_err() {
                errors.push(SansaVrmError::InvalidInput(format!(
                    "Property {} expects number but got {}",
                    property.property_id, property.value
                )));
            }
        }

        sansavrm_core::PropertyValueType::Boolean => {
            if property.value.parse::<bool>().is_err() {
                errors.push(SansaVrmError::InvalidInput(format!(
                    "Property {} expects boolean but got {}",
                    property.property_id, property.value
                )));
            }
        }

        sansavrm_core::PropertyValueType::Object
        | sansavrm_core::PropertyValueType::Array => {
            // 現段階では JSON値検証未実装。
            // serde_json 導入後に Object / Array の構造検証を追加する。
        }
    }
}
