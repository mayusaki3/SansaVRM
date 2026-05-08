use sansavrm_core::custom_parameter::{
    CustomParameterFallback,
    CustomParameterFallbackBehavior,
    CustomParameterIoScope,
    CustomParameterSchema,
    CustomParameterValueType,
};
use sansavrm_core::custom_parameter_registry::CustomParameterRegistry;

/// registry が登録済み schema を解決できることを確認する。
#[test]
fn core_custom_registry_001_schema_lookup_should_find_registered_schema() {
    let mut registry = CustomParameterRegistry::new();

    registry.register(CustomParameterSchema {
        namespace: "mujoco".to_string(),
        name: "armature".to_string(),
        target_type: "joint".to_string(),
        value_type: CustomParameterValueType::Number,
        unit: Some("kg*m^2".to_string()),
        required: false,
        default: None,
        min: None,
        max: None,
        enum_values: vec![],
        description: "joint armature".to_string(),
        adapter_support: Default::default(),
        fallback: CustomParameterFallback {
            behavior: CustomParameterFallbackBehavior::UseDefault,
            value: None,
        },
        io_scope: CustomParameterIoScope::Mjcf,
        mjcf_mapping: None,
        adapter_artifact: None,
        mujoco_version: None,
        supported_since: None,
        deprecated_since: None,
    });

    let schema = registry.resolve_custom_parameter_schema(
        "mujoco",
        "armature",
        "joint",
    );

    assert!(schema.is_some());
}

/// 未登録 schema が None を返すことを確認する。
#[test]
fn core_custom_registry_002_unknown_schema_should_return_none() {
    let registry = CustomParameterRegistry::new();

    let schema = registry.resolve_custom_parameter_schema(
        "mujoco",
        "unknown",
        "joint",
    );

    assert!(schema.is_none());
}
