// crates/sansavrm-validator/tests/property_validation.rs

use sansavrm_core::{
    Model, Module, ModuleType, Property, PropertyContext, PropertyType,
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

fn add_humanoid_bone(model: &mut Model, bone_name: &str, module_id: &str) {
    model.modules.push(Module {
        module_id: module_id.into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.properties.push(Property::from_typed_value(
        format!("p_{}", bone_name),
        format!("vrm.humanoid.human_bones.{}.node", bone_name),
        sansavrm_core::PropertyValue::String(module_id.into()),
        PropertyType::Metadata,
        PropertyContext::Description,
    ));
}

#[test]
fn validator_property_001_string_value_should_pass() {
    let model = model_with_module_property(Property::from_typed_value(
        "property_001",
        "name",
        sansavrm_core::PropertyValue::String("SansaVRM".into()),
        PropertyType::Metadata,
        PropertyContext::Description,
    ));

    let result = validate_model(&model);

    assert!(result.success);
}

#[test]
fn validator_property_002_number_value_should_pass() {
    let model = model_with_module_property(Property::from_typed_value(
        "property_001",
        "weight",
        sansavrm_core::PropertyValue::Number(12.5),
        PropertyType::Metadata,
        PropertyContext::Description,
    ));

    let result = validate_model(&model);

    assert!(result.success);
}

#[test]
fn validator_property_003_boolean_value_should_pass() {
    let model = model_with_module_property(Property::from_typed_value(
        "property_001",
        "enabled",
        sansavrm_core::PropertyValue::Bool(true),
        PropertyType::Metadata,
        PropertyContext::Description,
    ));

    let result = validate_model(&model);

    assert!(result.success);
}

#[test]
fn validator_property_004_physics_property_with_physics_role_should_pass() {
    let model = model_with_module_property(Property::from_typed_value(
        "property_001",
        "mass",
        sansavrm_core::PropertyValue::Number(12.5),
        PropertyType::Physics,
        PropertyContext::Simulation,
    ));

    let result = validate_model(&model);

    assert!(result.success);
}

#[test]
fn validator_property_005_sensor_property_with_module_role_should_fail() {
    let model = model_with_module_property(Property::from_typed_value(
        "property_001",
        "position",
        sansavrm_core::PropertyValue::String("0.0".into()),
        PropertyType::Sensor,
        PropertyContext::Description,
    ));

    let result = validate_model(&model);

    assert!(!result.success);
}

#[test]
fn validator_property_006_model_level_invalid_number_should_fail() {
    let mut model = Model::new();

    model.properties.push(Property {
        property_id: "property_model_invalid_number".into(),
        key: "vrm.humanoid.human_bones.head.node".into(),
        value: sansavrm_core::PropertyValue::String("Head".into()),
        property_type: PropertyType::Metadata,
        context: PropertyContext::Description,
    });

    let result = validate_model(&model);

    assert!(!result.success);
}

#[test]
fn validator_property_007_model_level_vrm_humanoid_property_without_module_should_fail() {
    let mut model = Model::new();

    model.properties.push(Property::from_typed_value(
        "property_vrm_humanoid_human_bones_head_node",
        "vrm.humanoid.human_bones.head.node",
        sansavrm_core::PropertyValue::String("Head".into()),
        PropertyType::Metadata,
        PropertyContext::Description,
    ));

    let result = validate_model(&model);

    assert!(!result.success);
}

#[test]
fn validator_property_008_vrm_humanoid_missing_head_should_fail() {
    let mut model = Model::new();

    model.properties.push(Property::from_typed_value(
        "property_vrm_humanoid_human_bones_left_upper_arm_node",
        "vrm.humanoid.human_bones.leftUpperArm.node",
        sansavrm_core::PropertyValue::String("LeftUpperArm".into()),
        PropertyType::Metadata,
        PropertyContext::Description,
    ));

    let result = validate_model(&model);

    assert!(!result.success);
}

#[test]
fn validator_property_009_vrm_humanoid_invalid_reference_should_fail() {
    let mut model = Model::new();

    model.properties.push(Property::from_typed_value(
        "p1",
        "vrm.humanoid.human_bones.head.node",
        sansavrm_core::PropertyValue::String("Unknown".into()),
        PropertyType::Metadata,
        PropertyContext::Description,
    ));

    let result = validate_model(&model);

    assert!(!result.success);
}

#[test]
fn validator_property_010_vrm_humanoid_valid_should_pass() {
    let mut model = Model::new();

    for bone_name in [
        "hips",
        "spine",
        "chest",
        "neck",
        "head",
        "leftUpperLeg",
        "leftLowerLeg",
        "leftFoot",
        "rightUpperLeg",
        "rightLowerLeg",
        "rightFoot",
        "leftUpperArm",
        "leftLowerArm",
        "leftHand",
        "rightUpperArm",
        "rightLowerArm",
        "rightHand",
    ] {
        add_humanoid_bone(&mut model, bone_name, bone_name);
    }

    let result = validate_model(&model);

    assert!(result.success);
}

#[test]
fn validator_property_011_vrm_humanoid_missing_spine_should_fail() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "Hips".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.modules.push(Module {
        module_id: "Head".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.properties.push(Property::from_typed_value(
        "p_hips",
        "vrm.humanoid.human_bones.hips.node",
        sansavrm_core::PropertyValue::String("Hips".into()),
        PropertyType::Metadata,
        PropertyContext::Description,
    ));

    model.properties.push(Property::from_typed_value(
        "p_head",
        "vrm.humanoid.human_bones.head.node",
        sansavrm_core::PropertyValue::String("Head".into()),
        PropertyType::Metadata,
        PropertyContext::Description,
    ));

    let result = validate_model(&model);

    assert!(!result.success);
}

#[test]
fn validator_property_012_typed_number_property_should_pass() {
    let mut model = Model::new();

    model.properties.push(Property::from_typed_value(
        "property_typed_number",
        "mass",
        sansavrm_core::PropertyValue::Number(12.5),
        PropertyType::Metadata,
        PropertyContext::Description,
    ));

    let result = validate_model(&model);

    assert!(result.success);
}

#[test]
fn validator_property_013_typed_bool_property_should_pass() {
    let mut model = Model::new();

    model.properties.push(Property::from_typed_value(
        "property_typed_bool",
        "enabled",
        sansavrm_core::PropertyValue::Bool(true),
        PropertyType::Metadata,
        PropertyContext::Description,
    ));

    let result = validate_model(&model);

    assert!(result.success);
}

#[test]
fn validator_property_014_typed_string_property_should_pass() {
    let mut model = Model::new();

    model.properties.push(Property::from_typed_value(
        "property_typed_string",
        "name",
        sansavrm_core::PropertyValue::String("SansaVRM".into()),
        PropertyType::Metadata,
        PropertyContext::Description,
    ));

    let result = validate_model(&model);

    assert!(result.success);
}
