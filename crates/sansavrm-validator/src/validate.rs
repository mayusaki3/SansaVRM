// crates/sansavrm-validator/src/validate.rs

use std::collections::HashSet;

use sansavrm_core::{
    CoreResult, DiagnosticCode, DiagnosticSeverity, Model, Property, PropertyContext, PropertyType,
    PropertyValue, SansaVrmError, ValidationDiagnostic,
};

use crate::vrm_validation::validate_vrm_humanoid;
use crate::ValidatorResult;

/// Model の基本検証。
pub fn validate_model(model: &Model) -> CoreResult<()> {
    let mut errors = Vec::new();
    validate_unique_ids(model, &mut errors);
    validate_slot_owner_refs(model, &mut errors);
    validate_connections(model, &mut errors);
    validate_connection_rules(model, &mut errors);
    validate_state_actions(model, &mut errors);
    validate_properties(model, &mut errors);
    validate_compatibility_properties(model, &mut errors);
    validate_rights_revenue_properties(model, &mut errors);
    validate_gltf_binding_properties(model, &mut errors);
    validate_vrm_humanoid(model, &mut errors);

    if errors.is_empty() {
        CoreResult::ok(())
    } else {
        CoreResult { success: false, data: None, errors, warnings: vec![], infos: vec![] }
    }
}

/// Model の diagnostics 付き基本検証。
pub fn validate_model_with_diagnostics(model: &Model) -> ValidatorResult {
    let mut diagnostics = Vec::new();
    validate_unique_ids_with_diagnostics(model, &mut diagnostics);
    validate_slot_owner_refs_with_diagnostics(model, &mut diagnostics);
    validate_connections_with_diagnostics(model, &mut diagnostics);
    validate_properties_with_diagnostics(model, &mut diagnostics);
    sort_diagnostics(&mut diagnostics);
    ValidatorResult { success: diagnostics.is_empty(), diagnostics }
}

/// ID 一意性検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_f7a2d9m4
fn validate_unique_ids(model: &Model, errors: &mut Vec<SansaVrmError>) {
    let mut ids = HashSet::new();
    for id in collect_ids(model) {
        if !ids.insert(id.clone()) {
            errors.push(SansaVrmError::DuplicateId(id));
        }
    }
}

/// ID 一意性 diagnostics 検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_f7a2d9m4
fn validate_unique_ids_with_diagnostics(model: &Model, diagnostics: &mut Vec<ValidationDiagnostic>) {
    let mut ids = HashSet::new();
    for id in collect_ids(model) {
        if !ids.insert(id.clone()) {
            diagnostics.push(ValidationDiagnostic {
                code: DiagnosticCode::DuplicateId,
                severity: DiagnosticSeverity::Error,
                message: format!("Duplicate ID: {}", id),
                target: Some(id),
            });
        }
    }
}

/// Slot owner参照整合性検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_g2c9d4x7
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_k8m4q2r7
fn validate_slot_owner_refs(model: &Model, errors: &mut Vec<SansaVrmError>) {
    for slot in &model.slots {
        if !model.modules.iter().any(|m| m.module_id == slot.owner_module_id) {
            errors.push(SansaVrmError::InvalidInput(format!(
                "Slot {} references unknown module {}",
                slot.slot_id, slot.owner_module_id
            )));
        }
    }
}

/// Slot owner参照 diagnostics 検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_g2c9d4x7
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_k8m4q2r7
fn validate_slot_owner_refs_with_diagnostics(model: &Model, diagnostics: &mut Vec<ValidationDiagnostic>) {
    for slot in &model.slots {
        if !model.modules.iter().any(|m| m.module_id == slot.owner_module_id) {
            diagnostics.push(ValidationDiagnostic {
                code: DiagnosticCode::RefNotFound,
                severity: DiagnosticSeverity::Error,
                message: format!("Slot {} references unknown module {}", slot.slot_id, slot.owner_module_id),
                target: Some(slot.slot_id.clone()),
            });
        }
    }
}

/// Connection参照整合性検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_h1e0f3y8
fn validate_connections(model: &Model, errors: &mut Vec<SansaVrmError>) {
    for c in &model.connections {
        let from_exists = id_exists(model, &c.from_id);
        let to_exists = id_exists(model, &c.to_id);
        if !from_exists {
            errors.push(SansaVrmError::InvalidInput(format!("Connection {} references unknown from_id {}", c.connection_id, c.from_id)));
        }
        if !to_exists {
            errors.push(SansaVrmError::InvalidInput(format!("Connection {} references unknown to_id {}", c.connection_id, c.to_id)));
        }
    }
}

/// Connection参照 diagnostics 検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_h1e0f3y8
fn validate_connections_with_diagnostics(model: &Model, diagnostics: &mut Vec<ValidationDiagnostic>) {
    for c in &model.connections {
        if !id_exists(model, &c.from_id) {
            diagnostics.push(ValidationDiagnostic {
                code: DiagnosticCode::RefNotFound,
                severity: DiagnosticSeverity::Error,
                message: format!("Connection {} references unknown from_id {}", c.connection_id, c.from_id),
                target: Some(c.connection_id.clone()),
            });
        }

        if !id_exists(model, &c.to_id) {
            diagnostics.push(ValidationDiagnostic {
                code: DiagnosticCode::RefNotFound,
                severity: DiagnosticSeverity::Error,
                message: format!("Connection {} references unknown to_id {}", c.connection_id, c.to_id),
                target: Some(c.connection_id.clone()),
            });
        }
    }
}

/// ConnectionRule整合性検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_h1e0f3y8
fn validate_connection_rules(model: &Model, errors: &mut Vec<SansaVrmError>) {
    for slot in &model.slots {
        if let Some(rule) = &slot.connection_rules {
            let count = model.connections.iter().filter(|c| c.from_id == slot.slot_id || c.to_id == slot.slot_id).count();
            if count < rule.min_connections || count > rule.max_connections || (rule.exclusive && count > 1) {
                errors.push(SansaVrmError::InvalidInput(format!("Slot {} violates connection rule", slot.slot_id)));
            }
        }
    }
}

/// StateAction参照整合性検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_j9g1h2z9
fn validate_state_actions(model: &Model, errors: &mut Vec<SansaVrmError>) {
    for state in &model.states {
        for action in &state.actions {
            match action {
                sansavrm_core::StateAction::ModuleEnable { module_id }
                | sansavrm_core::StateAction::ModuleDisable { module_id } => {
                    if !model.modules.iter().any(|m| &m.module_id == module_id) {
                        errors.push(SansaVrmError::InvalidInput(format!("State {} references unknown module {}", state.state_id, module_id)));
                    }
                }
                sansavrm_core::StateAction::SlotBind { slot_id, target_slot_id }
                | sansavrm_core::StateAction::SlotUnbind { slot_id, target_slot_id } => {
                    if !model.slots.iter().any(|s| &s.slot_id == slot_id) || !model.slots.iter().any(|s| &s.slot_id == target_slot_id) {
                        errors.push(SansaVrmError::InvalidInput(format!("State {} references unknown slot", state.state_id)));
                    }
                }
                sansavrm_core::StateAction::ConnectionEnable { connection_id }
                | sansavrm_core::StateAction::ConnectionDisable { connection_id } => {
                    if !model.connections.iter().any(|c| &c.connection_id == connection_id) {
                        errors.push(SansaVrmError::InvalidInput(format!("State {} references unknown connection {}", state.state_id, connection_id)));
                    }
                }
                sansavrm_core::StateAction::PropertyOverride { .. } => {}
                sansavrm_core::StateAction::VisibilityChange { target_id, .. } => {
                    if !id_exists(model, target_id) {
                        errors.push(SansaVrmError::InvalidInput(format!("State {} references unknown visibility target {}", state.state_id, target_id)));
                    }
                }
            }
        }
    }
}

/// Property分類整合性検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_n4s1u6v0
fn validate_properties(model: &Model, errors: &mut Vec<SansaVrmError>) {
    for p in all_properties(model) {
        validate_property_value(p, errors);
    }
}

/// Property分類 diagnostics 検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_n4s1u6v0
fn validate_properties_with_diagnostics(model: &Model, diagnostics: &mut Vec<ValidationDiagnostic>) {
    for p in all_properties(model) {
        validate_property_value_with_diagnostics(p, diagnostics);
    }
}

/// Compatibility整合性検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_l6n3p8s2
fn validate_compatibility_properties(model: &Model, errors: &mut Vec<SansaVrmError>) {
    for p in all_properties(model) {
        if p.property_type == PropertyType::Compatibility
            && !matches!(p.context, PropertyContext::Validation | PropertyContext::Conversion)
        {
            errors.push(SansaVrmError::InvalidInput(format!("Compatibility property {} has invalid context", p.property_id)));
        }
    }
}

/// Rights / Revenue整合性検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_m5q2r7t1
fn validate_rights_revenue_properties(model: &Model, errors: &mut Vec<SansaVrmError>) {
    for p in all_properties(model) {
        if matches!(p.property_type, PropertyType::Rights | PropertyType::Revenue) && p.key.trim().is_empty() {
            errors.push(SansaVrmError::InvalidInput(format!("Rights or revenue property {} has empty key", p.property_id)));
        }
    }
}

/// glTF補助整合性検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_p3t0w5x9
fn validate_gltf_binding_properties(model: &Model, errors: &mut Vec<SansaVrmError>) {
    for p in all_properties(model) {
        if p.context == PropertyContext::Binding && p.key.trim().is_empty() {
            errors.push(SansaVrmError::InvalidInput(format!("glTF binding property {} has empty key", p.property_id)));
        }
    }
}

/// Property値検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_n4s1u6v0
fn validate_property_value(p: &Property, errors: &mut Vec<SansaVrmError>) {
    match &p.value {
        PropertyValue::String(_) | PropertyValue::Number(_) | PropertyValue::Bool(_) => {}
    }
    if !property_classification_is_valid(p) {
        errors.push(SansaVrmError::InvalidInput(format!("Property {} has incompatible property_type / context / key", p.property_id)));
    }
}

/// Property値 diagnostics 検証。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_n4s1u6v0
fn validate_property_value_with_diagnostics(p: &Property, diagnostics: &mut Vec<ValidationDiagnostic>) {
    if !property_classification_is_valid(p) {
        diagnostics.push(ValidationDiagnostic {
            code: DiagnosticCode::PropertyClassificationMismatch,
            severity: DiagnosticSeverity::Error,
            message: format!("Property {} has incompatible property_type / context / key", p.property_id),
            target: Some(p.property_id.clone()),
        });
    }
}

/// Property分類の組み合わせを判定する。
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_n4s1u6v0
fn property_classification_is_valid(p: &Property) -> bool {
    match &p.property_type {
        PropertyType::Physics => matches!(p.context, PropertyContext::Simulation | PropertyContext::Execution),
        PropertyType::Geometry | PropertyType::Material | PropertyType::Texture => matches!(p.context, PropertyContext::Rendering | PropertyContext::Conversion),
        PropertyType::Actuator => matches!(p.context, PropertyContext::Execution | PropertyContext::Simulation),
        PropertyType::Sensor => matches!(p.context, PropertyContext::IO | PropertyContext::Execution),
        PropertyType::Constraint => matches!(p.context, PropertyContext::Validation),
        PropertyType::Metadata => matches!(p.context, PropertyContext::Description),
        _ => true,
    }
}

/// Model内の全Propertyを列挙する。
fn all_properties(model: &Model) -> Vec<&Property> {
    let mut result = Vec::new();
    result.extend(model.properties.iter());
    for module in &model.modules { result.extend(module.properties.iter()); }
    for slot in &model.slots { result.extend(slot.properties.iter()); }
    result
}

/// Model内の参照可能IDを列挙する。
fn collect_ids(model: &Model) -> Vec<String> {
    let mut ids = vec![model.model_id.clone()];
    ids.extend(model.modules.iter().map(|m| m.module_id.clone()));
    ids.extend(model.slots.iter().map(|s| s.slot_id.clone()));
    ids.extend(model.states.iter().map(|s| s.state_id.clone()));
    ids.extend(model.connections.iter().map(|c| c.connection_id.clone()));
    ids
}

/// IDがModel内に存在するか判定する。
fn id_exists(model: &Model, id: &str) -> bool {
    model.modules.iter().any(|m| m.module_id == id)
        || model.slots.iter().any(|s| s.slot_id == id)
        || model.connections.iter().any(|c| c.connection_id == id)
}

/// diagnostics の出力順を安定化する。
fn sort_diagnostics(diagnostics: &mut [ValidationDiagnostic]) {
    diagnostics.sort_by(|a, b| diagnostic_sort_key(a).cmp(&diagnostic_sort_key(b)));
}

/// diagnostics ソートキーを生成する。
fn diagnostic_sort_key(d: &ValidationDiagnostic) -> String {
    format!("{:?}|{:?}|{}|{}", d.severity, d.code, d.target.clone().unwrap_or_default(), d.message)
}
