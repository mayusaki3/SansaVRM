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

#[test]
fn validator_property_008_model_level_invalid_number_should_fail() {
    let mut model = Model::new();

    model.properties.push(Property {
        property_id: "property_model_invalid_number".into(),
        key: "vrm.humanoid.human_bones.head.node".into(),
        value: "Head".into(),
        value_type: PropertyValueType::Number,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    let result = validate_model(&model);

    assert!(!result.success);
}

#[test]
fn validator_property_009_model_level_vrm_humanoid_property_without_module_should_fail() {
    let mut model = Model::new();

    model.properties.push(Property {
        property_id: "property_vrm_humanoid_human_bones_head_node".into(),
        key: "vrm.humanoid.human_bones.head.node".into(),
        value: "Head".into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    let result = validate_model(&model);

    assert!(!result.success);
}

#[test]
fn validator_property_010_vrm_humanoid_missing_head_should_fail() {
    let mut model = Model::new();

    model.properties.push(Property {
        property_id: "property_vrm_humanoid_human_bones_left_upper_arm_node".into(),
        key: "vrm.humanoid.human_bones.leftUpperArm.node".into(),
        value: "LeftUpperArm".into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    let result = validate_model(&model);

    assert!(!result.success);
}

#[test]
fn validator_property_011_vrm_humanoid_invalid_reference_should_fail() {
    let mut model = Model::new();

    model.properties.push(Property {
        property_id: "p1".into(),
        key: "vrm.humanoid.human_bones.head.node".into(),
        value: "Unknown".into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    let result = validate_model(&model);

    assert!(!result.success);
}

#[test]
fn validator_property_012_vrm_humanoid_valid_should_pass() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "Hips".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.properties.push(Property {
        property_id: "p_hips".into(),
        key: "vrm.humanoid.human_bones.hips.node".into(),
        value: "Hips".into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    model.modules.push(Module {
        module_id: "Head".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.properties.push(Property {
        property_id: "p_head".into(),
        key: "vrm.humanoid.human_bones.head.node".into(),
        value: "Head".into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    });

    let result = validate_model(&model);

    assert!(result.success);
}
