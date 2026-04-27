// crates/sansavrm-validator/tests/diagnostics_validation.rs

use sansavrm_core::{DiagnosticCode, DiagnosticSeverity, Model, Module, ModuleType};
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
