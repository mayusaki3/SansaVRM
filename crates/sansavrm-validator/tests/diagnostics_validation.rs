// crates/sansavrm-validator/tests/diagnostics_validation.rs

use sansavrm_core::{DiagnosticCode, DiagnosticSeverity, Model, Module, ModuleType, Slot, SlotType};
use sansavrm_validator::validate_model_with_diagnostics;

#[test]
fn validator_diag_001_duplicate_id_should_return_diagnostic() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: model.model_id.clone(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    let result = validate_model_with_diagnostics(&model);

    assert!(!result.success);
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(result.diagnostics[0].code, DiagnosticCode::DuplicateId);
    assert_eq!(result.diagnostics[0].severity, DiagnosticSeverity::Error);
    assert_eq!(result.diagnostics[0].target, Some(model.model_id.clone()));
}

#[test]
fn validator_diag_002_unique_ids_should_return_no_diagnostics() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "module_001".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    let result = validate_model_with_diagnostics(&model);

    assert!(result.success);
    assert!(result.diagnostics.is_empty());
}

#[test]
fn validator_diag_003_unknown_slot_owner_should_return_ref_not_found() {
    let mut model = Model::new();

    model.slots.push(Slot {
        slot_id: "slot_001".into(),
        slot_type: SlotType::Structure,
        owner_module_id: "unknown_module".into(),
        target_slot_types: vec![],
        current_connections: vec![],
        connection_rules: None,
        properties: vec![],
    });

    let result = validate_model_with_diagnostics(&model);

    assert!(!result.success);
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(result.diagnostics[0].code, DiagnosticCode::RefNotFound);
    assert_eq!(result.diagnostics[0].severity, DiagnosticSeverity::Error);
    assert_eq!(result.diagnostics[0].target, Some("slot_001".into()));
}
