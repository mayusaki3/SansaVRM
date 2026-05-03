use sansavrm_core::{
    Model, Property, PropertyContext, PropertyType, PropertyValue, VrmVersion,
};
use serde_json::{json, Value};

use crate::common::{
    get_model_property,
    import_string_property,
    vrm_humanoid_bone_node_key,
    VRM_HUMANOID_BONES,
};

/// SansaVRM: VRM 0.x humanoid Import
/// TODO(trace): 変換仕様 / VRM 0.x Humanoid Import
pub(crate) fn import_vrm0_humanoid(model: &mut Model, document: &str, version: Option<VrmVersion>) {
    if version != Some(VrmVersion::V0x) {
        return;
    }

    let Ok(value) = serde_json::from_str::<Value>(document) else {
        return;
    };

    let Some(human_bones) = value.pointer("/extensions/VRM/humanoid/humanBones") else {
        return;
    };

    let Some(human_bones) = human_bones.as_array() else {
        return;
    };

    for bone in human_bones {
        let Some(bone_name) = bone.get("bone").and_then(Value::as_str) else {
            continue;
        };

        if !VRM_HUMANOID_BONES.contains(&bone_name) {
            continue;
        }

        let Some(node_index) = bone.get("node").and_then(Value::as_u64) else {
            continue;
        };

        let Some(module) = model.modules.get(node_index as usize) else {
            continue;
        };

        model.properties.push(Property::from_typed_value(
            format!("p_{}", bone_name),
            vrm_humanoid_bone_node_key(bone_name),
            PropertyValue::String(module.module_id.clone()),
            PropertyType::Rig,
            PropertyContext::Binding,
        ));
    }
}

/// SansaVRM: VRM 0.x humanoid Export
/// TODO(trace): 変換仕様 / VRM 0.x Humanoid Export
pub(crate) fn apply_vrm0_humanoid(value: &mut Value, model: &Model, version: VrmVersion) {
    if version != VrmVersion::V0x {
        return;
    }

    let mut bones = Vec::new();

    for property in &model.properties {
        if let Some(bone_name) = property
            .key
            .strip_prefix("vrm.humanoid.human_bones.")
            .and_then(|s| s.strip_suffix(".node"))
        {
            if !VRM_HUMANOID_BONES.contains(&bone_name) {
                continue;
            }

            let Some(module_id) = property.value.as_string() else {
                continue;
            };

            if let Some(node_index) = model
                .modules
                .iter()
                .position(|module| module.module_id == module_id)
            {
                bones.push(json!({
                    "bone": bone_name,
                    "node": node_index
                }));
            }
        }
    }

    value["extensions"]["VRM"]["humanoid"]["humanBones"] = json!(bones);
}

/// SansaVRM: VRM 0.x meta Import
/// TODO(trace): 変換仕様 / VRM 0.x Meta Import
pub(crate) fn import_vrm0_meta(model: &mut Model, value: &Value) {
    if let Some(meta) = value.pointer("/extensions/VRM/meta") {
        import_string_property(model, meta, "title", "vrm.meta.name");
        import_string_property(model, meta, "version", "vrm.meta.version");
        import_string_property(model, meta, "author", "vrm.meta.authors");
        import_string_property(model, meta, "licenseName", "vrm.meta.license_url");
    }
}

/// SansaVRM: VRM 0.x meta Export
/// TODO(trace): 変換仕様 / VRM 0.x Meta Export
pub(crate) fn apply_vrm0_meta(value: &mut Value, model: &Model) {
    value["extensions"]["VRM"]["meta"]["title"] =
        json!(get_model_property(model, "vrm.meta.name").unwrap_or_default());

    value["extensions"]["VRM"]["meta"]["version"] =
        json!(get_model_property(model, "vrm.meta.version").unwrap_or_default());

    value["extensions"]["VRM"]["meta"]["author"] =
        json!(get_model_property(model, "vrm.meta.authors").unwrap_or_default());

    value["extensions"]["VRM"]["meta"]["licenseName"] =
        json!(get_model_property(model, "vrm.meta.license_url").unwrap_or_default());
}
