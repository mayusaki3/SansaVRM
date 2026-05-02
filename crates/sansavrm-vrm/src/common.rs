use sansavrm_core::{
    Model, Property, PropertyContext, PropertyType, PropertyValue, VrmVersion,
};
use serde_json::Value;

/// SansaVRM: VRM humanoid humanBones Property key prefix.
/// TODO(trace): 変換仕様 / VRM Humanoid Property
pub(crate) const VRM_HUMANOID_BONE_PREFIX: &str = "vrm.humanoid.human_bones.";


/// SansaVRM: VRM humanoid humanBones Property key suffix.
/// TODO(trace): 変換仕様 / VRM Humanoid Property
pub(crate) const VRM_HUMANOID_BONE_NODE_SUFFIX: &str = ".node";

/// SansaVRM: VRM 1.0 supported humanoid bones.
/// TODO(trace): 変換仕様 / VRM Humanoid Property
pub(crate) const VRM_HUMANOID_BONES: &[&str] = &[
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
];

/// SansaVRM: VRM humanoid humanBone key を Property key に変換する。
/// TODO(trace): 変換仕様 / VRM Humanoid Property
pub(crate) fn vrm_humanoid_bone_node_key(bone_name: &str) -> String {
    format!(
        "{}{}{}",
        VRM_HUMANOID_BONE_PREFIX, bone_name, VRM_HUMANOID_BONE_NODE_SUFFIX
    )
}

/// SansaVRM: VRM humanoid humanBone Property key から bone名を取得する。
/// TODO(trace): 変換仕様 / VRM Humanoid Property
pub(crate) fn parse_vrm_humanoid_bone_property_key(key: &str) -> Option<&str> {
    key.strip_prefix(VRM_HUMANOID_BONE_PREFIX)?
        .strip_suffix(VRM_HUMANOID_BONE_NODE_SUFFIX)
        .filter(|bone_name| !bone_name.is_empty())
}

/// Get a SansaVRM Model-level property value by key.
pub(crate) fn get_model_property<'a>(model: &'a Model, key: &str) -> Option<&'a str> {
    model
        .properties
        .iter()
        .find(|property| property.key == key)
        .and_then(|property| property.value.as_string())
}

/// Import a string value from VRM meta into SansaVRM Model properties.
pub(crate) fn import_string_property(
    model: &mut Model,
    meta: &Value,
    source_key: &str,
    property_key: &str,
) {
    if let Some(value) = meta.get(source_key).and_then(Value::as_str) {
        model.properties.push(vrm_meta_property(property_key, value));
    }
}

/// Import an array value from VRM meta into SansaVRM Model properties.
///
/// Note:
/// - SansaVRM stores the array as a comma-separated string in this initial implementation.
pub(crate) fn import_array_property(
    model: &mut Model,
    meta: &Value,
    source_key: &str,
    property_key: &str,
) {
    if let Some(values) = meta.get(source_key).and_then(Value::as_array) {
        let joined = values
            .iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>()
            .join(", ");

        if !joined.is_empty() {
            model.properties.push(vrm_meta_property(property_key, &joined));
        }
    }
}

/// Create a SansaVRM Property for VRM meta.
pub(crate) fn vrm_meta_property(key: &str, value: &str) -> Property {
    Property::from_typed_value(
        format!("property_{}", key.replace('.', "_")),
        key,
        PropertyValue::String(value.into()),
        PropertyType::Metadata,
        PropertyContext::Description,
    )
}

/// Detect VRM version from glTF JSON document.
///
/// SansaVRM assumes:
/// - VRM 1.0: presence of "extensions.VRMC_vrm"
/// - VRM 0.x: presence of "extensions.VRM"
///
/// TODO(trace): 変換仕様 / VRM Version Detection
pub(crate) fn detect_vrm_version(document: &str) -> Option<VrmVersion> {
    let value = serde_json::from_str::<Value>(document).ok()?;
    let extensions = value.get("extensions")?;

    if extensions.get("VRMC_vrm").is_some() {
        return Some(VrmVersion::V1_0);
    }

    if extensions.get("VRM").is_some() {
        return Some(VrmVersion::V0x);
    }

    None
}
