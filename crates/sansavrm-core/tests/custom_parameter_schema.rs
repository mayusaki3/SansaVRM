// crates/sansavrm-core/tests/custom_parameter_schema.rs

use sansavrm_core::{
    AdapterArtifactMapping, CustomParameterFallback,
    CustomParameterFallbackBehavior, CustomParameterIoScope,
    CustomParameterSchema, CustomParameterValueType, MjcfMapping,
    PropertyValue, VersionRange,
};

/// io_scope = mjcf の場合に mjcf_mapping が必要であることを確認する。
///
/// テスト内容:
/// - mjcf_mapping 未設定時は false を返すこと。
/// - mjcf_mapping 設定時は true を返すこと。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m2
#[test]
fn core_custom_parameter_001_mjcf_scope_should_require_mjcf_mapping() {
    let mut schema = create_base_schema(CustomParameterIoScope::Mjcf);

    assert!(!schema.has_required_mappings());

    schema.mjcf_mapping = Some(MjcfMapping {
        element: "joint".into(),
        attribute: "armature".into(),
        path: "joint.@armature".into(),
        direction: "import_export".into(),
        value_conversion: None,
        required_mujoco_version: VersionRange {
            min: Some("2.3.0".into()),
            max: None,
        },
    });

    assert!(schema.has_required_mappings());
}

/// io_scope = adapter_artifact の場合に adapter_artifact が必要であることを確認する。
///
/// テスト内容:
/// - adapter_artifact 未設定時は false を返すこと。
/// - adapter_artifact 設定時は true を返すこと。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m3
#[test]
fn core_custom_parameter_002_adapter_scope_should_require_artifact_mapping() {
    let mut schema = create_base_schema(CustomParameterIoScope::AdapterArtifact);

    assert!(!schema.has_required_mappings());

    schema.adapter_artifact = Some(AdapterArtifactMapping {
        artifact_type: "controller_config".into(),
        path: "actuators[].delay".into(),
        direction: "export".into(),
        value_conversion: None,
        required_adapter_version: VersionRange {
            min: Some("0.1.0".into()),
            max: None,
        },
    });

    assert!(schema.has_required_mappings());
}

/// io_scope = both の場合に両 mapping が必要であることを確認する。
///
/// テスト内容:
/// - 片方不足時は false を返すこと。
/// - 両方設定時は true を返すこと。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m3
#[test]
fn core_custom_parameter_003_both_scope_should_require_both_mappings() {
    let mut schema = create_base_schema(CustomParameterIoScope::Both);

    assert!(!schema.has_required_mappings());

    schema.mjcf_mapping = Some(MjcfMapping {
        element: "actuator".into(),
        attribute: "forcerange".into(),
        path: "actuator.@forcerange".into(),
        direction: "export".into(),
        value_conversion: None,
        required_mujoco_version: VersionRange {
            min: Some("2.3.0".into()),
            max: None,
        },
    });

    assert!(!schema.has_required_mappings());

    schema.adapter_artifact = Some(AdapterArtifactMapping {
        artifact_type: "controller_config".into(),
        path: "actuators[].torque_limit".into(),
        direction: "export".into(),
        value_conversion: None,
        required_adapter_version: VersionRange {
            min: Some("0.1.0".into()),
            max: None,
        },
    });

    assert!(schema.has_required_mappings());
}

fn create_base_schema(scope: CustomParameterIoScope) -> CustomParameterSchema {
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
