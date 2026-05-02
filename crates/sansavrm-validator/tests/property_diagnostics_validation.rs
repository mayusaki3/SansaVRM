// crates/sansavrm-validator/tests/property_diagnostics_validation.rs

use sansavrm_core::{
    DiagnosticCode, Model, Module, ModuleType, Property, PropertyRole, PropertyType,
};
use sansavrm_validator::{validate_diagnostics, ValidateOptions};

#[test]
fn validator_property_diag_001_classification_mismatch_should_return_diagnostic() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "module_001".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![Property {
            property_id: "property_001".into(),
            key: "position".into(),
            value: sansavrm_core::PropertyValue::String("0.0".into()),
            property_type: PropertyType::Sensor,
            role: PropertyRole::Module,
        }],
    });

    let result = validate_diagnostics(&model, ValidateOptions::default());

    assert!(!result.success);
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(
        result.diagnostics[0].code,
        DiagnosticCode::PropertyClassificationMismatch
    );
}
