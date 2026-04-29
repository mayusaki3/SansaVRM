// crates/sansavrm-validator/tests/diagnostics_order_validation.rs

use sansavrm_core::{
    Connection, ConnectionType, Model, Module, ModuleType, Property, PropertyRole, PropertyType,
    PropertyValueType,
};
use sansavrm_validator::{validate_diagnostics, ValidateOptions};

#[test]
fn validator_diag_order_001_diagnostics_order_should_be_stable() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "module_001".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![Property {
            property_id: "property_001".into(),
            key: "weight".into(),
            value: "not_number".into(),
            value_type: PropertyValueType::Number,
            property_type: PropertyType::Metadata,
            role: PropertyRole::Module,
        }],
    });

    model.connections.push(Connection {
        connection_id: "connection_001".into(),
        from_id: "unknown_from".into(),
        to_id: "unknown_to".into(),
        connection_type: ConnectionType::Attach,
        enabled: true,
    });

    let first = validate_diagnostics(&model, ValidateOptions::default());
    let second = validate_diagnostics(&model, ValidateOptions::default());

    assert!(!first.success);
    assert!(!second.success);
    assert_eq!(first.diagnostics, second.diagnostics);
    assert_eq!(first.diagnostics.len(), 3);
}
