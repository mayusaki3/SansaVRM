use sansavrm_core::{
    Model, Property, PropertyRole, PropertyType, PropertyValueType, VrmVersion,
};
use serde_json::{json, Value};

use crate::common::{
    get_model_property,
    import_array_property,
    import_string_property,
    parse_vrm_humanoid_bone_property_key,
    vrm_humanoid_bone_node_key,
    VRM_HUMANOID_BONES,
};

/// SansaVRM: VRM 1.0 humanoid Import
/// TODO(trace): 変換仕様 / VRM 1.0 Humanoid Import
pub(crate) fn import_vrm1_humanoid(model: &mut Model, document: &str, version: Option<VrmVersion>) {
    if version != Some(VrmVersion::V1_0) {
        return;
    }

    let Ok(value) = serde_json::from_str::<Value>(document) else {
        return;
    };

    let Some(human_bones) = value.pointer("/extensions/VRMC_vrm/humanoid/humanBones") else {
        return;
    };

    let Some(human_bones) = human_bones.as_object() else {
        return;
    };

    for (bone_name, bone_value) in human_bones {
        if !VRM_HUMANOID_BONES.contains(&bone_name.as_str()) {
            continue;
        }

        let Some(node_index) = bone_value.get("node").and_then(Value::as_u64) else {
            continue;
        };

        let Some(module) = model.modules.get(node_index as usize) else {
            continue;
        };

        model.properties.push(vrm_humanoid_bone_property(
            bone_name,
            &module.module_id,
        ));
    }
}

/// SansaVRM: VRM 1.0 humanoid Export
/// TODO(trace): 変換仕様 / VRM 1.0 Humanoid Export
pub(crate) fn apply_vrm1_humanoid(value: &mut Value, model: &Model, version: VrmVersion) {
    if version != VrmVersion::V1_0 {
        return;
    }

    for property in &model.properties {
        let Some(bone_name) = parse_vrm_humanoid_bone_property_key(&property.key) else {
            continue;
        };

        if !VRM_HUMANOID_BONES.contains(&bone_name) {
            continue;
        };

        let Some(node_index) = model
            .modules
            .iter()
            .position(|module| module.module_id == property.value)
        else {
            continue;
        };

        value["extensions"]["VRMC_vrm"]["humanoid"]["humanBones"][bone_name]["node"] =
            json!(node_index);
    }
}

/// SansaVRM: VRM 1.0 meta Import
/// TODO(trace): 変換仕様 / VRM 1.0 Meta Import
pub(crate) fn import_vrm1_meta(model: &mut Model, value: &Value) {
    if let Some(meta) = value.pointer("/extensions/VRMC_vrm/meta") {
        import_string_property(model, meta, "name", "vrm.meta.name");
        import_string_property(model, meta, "version", "vrm.meta.version");
        import_array_property(model, meta, "authors", "vrm.meta.authors");
        import_string_property(model, meta, "licenseUrl", "vrm.meta.license_url");
    }
}

/// SansaVRM: VRM 1.0 meta Export
/// TODO(trace): 変換仕様 / VRM 1.0 Meta Export
pub(crate) fn apply_vrm1_meta(value: &mut Value, model: &Model) {
    value["extensions"]["VRMC_vrm"]["meta"]["name"] =
        json!(get_model_property(model, "vrm.meta.name").unwrap_or_default());

    value["extensions"]["VRMC_vrm"]["meta"]["version"] =
        json!(get_model_property(model, "vrm.meta.version").unwrap_or_default());

    let authors = get_model_property(model, "vrm.meta.authors")
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();

    value["extensions"]["VRMC_vrm"]["meta"]["authors"] = json!(authors);

    value["extensions"]["VRMC_vrm"]["meta"]["licenseUrl"] =
        json!(get_model_property(model, "vrm.meta.license_url").unwrap_or_default());
}

/// Create a SansaVRM Property for VRM 1.0 humanoid humanBone mapping.
///
/// TODO(trace): 変換仕様 / VRM 1.0 Humanoid Property
fn vrm_humanoid_bone_property(bone_name: &str, module_id: &str) -> Property {
    let key = vrm_humanoid_bone_node_key(bone_name);

    Property {
        property_id: format!("property_{}", key.replace('.', "_")),
        key,
        value: module_id.into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    }
}
