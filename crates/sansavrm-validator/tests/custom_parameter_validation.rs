use sansavrm_core::{
    CustomParameterFallback,
    CustomParameterFallbackBehavior,
    CustomParameterIoScope,
    CustomParameterSchema,
    CustomParameterValueType,
    PropertyValue,
};

use sansavrm_validator::validate_custom_parameter_schema;

#[test]
fn validator_custom_parameter_001_missing_mjcf_mapping_should_fail() {
    let schema = create_schema(CustomParameterIoScope::Mjcf);

    let result = validate_custom_parameter_schema(&schema);

    assert!(!result.success);
    assert_eq!(result.diagnostics.len(), 1);
}

#[test]
fn validator_custom_parameter_002_unsupported_scope_should_warn() {
    let schema = create_schema(CustomParameterIoScope::Unsupported);

    let result = validate_custom_parameter_schema(&schema);

    assert!(result.success);
    assert_eq!(result.diagnostics.len(), 1);
}

#[test]
fn validator_custom_parameter_003_preserve_only_should_report_info() {
    let schema = create_schema(CustomParameterIoScope::PreserveOnly);

    let result = validate_custom_parameter_schema(&schema);

    assert!(result.success);
    assert_eq!(result.diagnostics.len(), 1);
}

fn create_schema(scope: CustomParameterIoScope) -> CustomParameterSchema {
    CustomParameterSchema {
        namespace: "mujoco".into(),
        name: "armature".into(),
        target_type: "joint".into(),
        value_type: CustomParameterValueType::Number,
        unit: Some("kg*m^2".into()),
        required: false,
        default: Some(PropertyValue::Number(0.0)),
        min: Some(0.0),
        max: None,
        allowed_values: None,
        description: Some("test".into()),
        adapter_support: vec!["sansa-vrm-mujoco-adapter".into()],
        fallback: CustomParameterFallback {
            behavior: CustomParameterFallbackBehavior::UseDefault,
            value: Some(PropertyValue::Number(0.0)),
        },
        io_scope: scope,
        mjcf_mapping: None,
        adapter_artifact: None,
        mujoco_version: None,
        supported_since: None,
        deprecated_since: None,
    }
}
