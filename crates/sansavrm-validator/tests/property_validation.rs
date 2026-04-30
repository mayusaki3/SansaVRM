// crates/sansavrm-validator/tests/property_validation.rs

use sansavrm_core::{
    Model, Module, ModuleType, Property, PropertyRole, PropertyType, PropertyValueType,
};
use sansavrm_validator::validate_model;

fn model_with_module_property(property: Property) -> Model {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "module_001".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![property],
    });

    model
}

#[test]
fn validator_property_001_string_value_should_pass() {
    let model = model_with_module_property(Property {
        property_id: "property_001".into(),
        key: "name".into(),
        value: "SansaVRM".into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    let result = validate_model(&model);

    assert!(result.success);
}

#[test]
fn validator_property_002_number_value_should_pass() {
    let model = model_with_module_property(Property {
        property_id: "property_001".into(),
        key: "weight".into(),
        value: "12.5".into(),
        value_type: PropertyValueType::Number,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    let result = validate_model(&model);

    assert!(result.success);
}

#[test]
fn validator_property_003_invalid_number_should_fail() {
    let model = model_with_module_property(Property {
        property_id: "property_001".into(),
        key: "weight".into(),
        value: "not_number".into(),
        value_type: PropertyValueType::Number,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    let result = validate_model(&model);

    assert!(!result.success);
}

#[test]
fn validator_property_004_boolean_value_should_pass() {
    let model = model_with_module_property(Property {
        property_id: "property_001".into(),
        key: "enabled".into(),
        value: "true".into(),
        value_type: PropertyValueType::Boolean,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    let result = validate_model(&model);

    assert!(result.success);
}

#[test]
fn validator_property_005_invalid_boolean_should_fail() {
    let model = model_with_module_property(Property {
        property_id: "property_001".into(),
        key: "enabled".into(),
        value: "yes".into(),
        value_type: PropertyValueType::Boolean,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    let result = validate_model(&model);

    assert!(!result.success);
}

#[test]
fn validator_property_006_physics_property_with_physics_role_should_pass() {
    let model = model_with_module_property(Property {
        property_id: "property_001".into(),
        key: "mass".into(),
        value: "12.5".into(),
        value_type: PropertyValueType::Number,
        property_type: PropertyType::Physics,
        role: PropertyRole::Physics,
    });

    let result = validate_model(&model);

    assert!(result.success);
}

#[test]
fn validator_property_007_sensor_property_with_module_role_should_fail() {
    let model = model_with_module_property(Property {
        property_id: "property_001".into(),
        key: "position".into(),
        value: "0.0".into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Sensor,
        role: PropertyRole::Module,
    });

    let result = validate_model(&model);

    assert!(!result.success);
}
