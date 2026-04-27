// crates/sansavrm-core/tests/diagnostics.rs

use sansavrm_core::{
    DiagnosticCode, DiagnosticSeverity, ValidationDiagnostic,
};

#[test]
fn core_diagnostics_001_validation_diagnostic_can_be_created() {
    let diagnostic = ValidationDiagnostic {
        code: DiagnosticCode::DuplicateId,
        severity: DiagnosticSeverity::Error,
        message: "duplicate id".into(),
        target: Some("module_001".into()),
    };

    assert_eq!(diagnostic.code, DiagnosticCode::DuplicateId);
    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert_eq!(diagnostic.message, "duplicate id");
    assert_eq!(diagnostic.target, Some("module_001".into()));
}
