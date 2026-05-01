//! SansaVRM VRM adapter.

use sansavrm_core::{
    CoreResult, IoOptions, Model, Property, PropertyRole, PropertyType, PropertyValueType,
    VrmDocument, VrmVersion,
};
use serde_json::{json, Value};

/// VRM を SansaVRM Model へ import する。
///
/// 注意:
/// - 初期実装では VRM を glTF JSON として読み込み、glTF import に委譲する。
/// - VRM 0.x / 1.0 固有メタデータの解釈は後続実装。
///
/// TODO(trace): 変換仕様 / VRM Import
pub fn import_vrm(document: VrmDocument) -> CoreResult<Model> {
    let version = detect_vrm_version(&document);

    let result = sansavrm_gltf::import_gltf(document.clone());
    
    if !result.success {
        return result;
    }

    let mut model = result.data.expect("model should be returned");
    model.vrm_version = version.clone();

    import_vrm_meta(&mut model, &document, version);

    CoreResult::ok(model)
}

/// SansaVRM Model を VRM へ export する。
///
/// 注意:
/// - 初期実装では glTF export 結果に VRM version extension を追加する。
/// - `version` は出力対象の VRM 系列を明示する。
/// - `options` は後続実装で使用する。
///
/// TODO(trace): 変換仕様 / VRM Export
pub fn export_vrm(
    model: &Model,
    version: VrmVersion,
    _options: IoOptions,
) -> CoreResult<VrmDocument> {
    let result = sansavrm_gltf::export_gltf(model);

    if !result.success {
        return result;
    }

    let document = result.data.expect("document should be returned");
    let mut value = match serde_json::from_str::<Value>(&document) {
        Ok(value) => value,
        Err(error) => {
            return CoreResult::fail(sansavrm_core::SansaVrmError::InvalidInput(format!(
                "Failed to parse exported glTF JSON: {}",
                error
            )));
        }
    };

    apply_vrm_extension(&mut value, version.clone());
    apply_vrm_meta(&mut value, model, version);

    match serde_json::to_string_pretty(&value) {
        Ok(document) => CoreResult::ok(document),
        Err(error) => CoreResult::fail(sansavrm_core::SansaVrmError::InvalidInput(format!(
            "Failed to export VRM JSON: {}",
            error
        ))),
    }
}

/// Apply VRM version extension to glTF JSON document.
///
/// SansaVRM exports:
/// - VRM 1.0 as "extensions.VRMC_vrm"
/// - VRM 0.x as "extensions.VRM"
///
/// Note:
/// - This function writes only the minimal version marker.
/// - Full VRM metadata, humanoid, expressions, and constraints are added in later steps.
fn apply_vrm_extension(value: &mut Value, version: VrmVersion) {
    if value.get("extensions").is_none() {
        value["extensions"] = json!({});
    }

    match version {
        VrmVersion::V1_0 => {
            value["extensions"]["VRMC_vrm"] = json!({
                "specVersion": "1.0"
            });
        }
        VrmVersion::V0x => {
            value["extensions"]["VRM"] = json!({
                "specVersion": "0.0"
            });
        }
    }
}

/// Import VRM meta information into SansaVRM Model properties.
///
/// SansaVRM stores VRM meta as Model-level properties:
/// - "vrm.meta.name"
/// - "vrm.meta.version"
/// - "vrm.meta.authors"
/// - "vrm.meta.license_url"
///
/// Note:
/// - VRM 1.0 meta is read from "extensions.VRMC_vrm.meta".
/// - VRM 0.x meta is read from "extensions.VRM.meta".
/// - Only string and string-array-compatible values are handled in this step.
fn import_vrm_meta(model: &mut Model, document: &str, version: Option<VrmVersion>) {
    let Ok(value) = serde_json::from_str::<Value>(document) else {
        return;
    };

    match version {
        Some(VrmVersion::V1_0) => {
            if let Some(meta) = value.pointer("/extensions/VRMC_vrm/meta") {
                import_string_property(model, meta, "name", "vrm.meta.name");
                import_string_property(model, meta, "version", "vrm.meta.version");
                import_array_property(model, meta, "authors", "vrm.meta.authors");
                import_string_property(model, meta, "licenseUrl", "vrm.meta.license_url");
            }
        }
        Some(VrmVersion::V0x) => {
            if let Some(meta) = value.pointer("/extensions/VRM/meta") {
                import_string_property(model, meta, "title", "vrm.meta.name");
                import_string_property(model, meta, "version", "vrm.meta.version");
                import_string_property(model, meta, "author", "vrm.meta.authors");
                import_string_property(model, meta, "licenseName", "vrm.meta.license_url");
            }
        }
        None => {}
    }
}

/// Import a string value from VRM meta into SansaVRM Model properties.
fn import_string_property(model: &mut Model, meta: &Value, source_key: &str, property_key: &str) {
    if let Some(value) = meta.get(source_key).and_then(Value::as_str) {
        model.properties.push(vrm_meta_property(property_key, value));
    }
}

/// Import an array value from VRM meta into SansaVRM Model properties.
///
/// Note:
/// - SansaVRM stores the array as a comma-separated string in this initial implementation.
fn import_array_property(model: &mut Model, meta: &Value, source_key: &str, property_key: &str) {
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
fn vrm_meta_property(key: &str, value: &str) -> Property {
    Property {
        property_id: format!("property_{}", key.replace('.', "_")),
        key: key.into(),
        value: value.into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    }
}

/// Apply SansaVRM Model VRM meta properties to glTF JSON document.
///
/// SansaVRM exports:
/// - VRM 1.0 meta to "extensions.VRMC_vrm.meta"
/// - VRM 0.x meta to "extensions.VRM.meta"
///
/// Note:
/// - "vrm.meta.authors" is split by comma for VRM 1.0.
/// - VRM 0.x stores authors as a single "author" string.
fn apply_vrm_meta(value: &mut Value, model: &Model, version: VrmVersion) {
    match version {
        VrmVersion::V1_0 => {
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
        VrmVersion::V0x => {
            value["extensions"]["VRM"]["meta"]["title"] =
                json!(get_model_property(model, "vrm.meta.name").unwrap_or_default());
            value["extensions"]["VRM"]["meta"]["version"] =
                json!(get_model_property(model, "vrm.meta.version").unwrap_or_default());
            value["extensions"]["VRM"]["meta"]["author"] =
                json!(get_model_property(model, "vrm.meta.authors").unwrap_or_default());
            value["extensions"]["VRM"]["meta"]["licenseName"] =
                json!(get_model_property(model, "vrm.meta.license_url").unwrap_or_default());
        }
    }
}

/// Get a SansaVRM Model-level property value by key.
fn get_model_property<'a>(model: &'a Model, key: &str) -> Option<&'a str> {
    model
        .properties
        .iter()
        .find(|property| property.key == key)
        .map(|property| property.value.as_str())
}

/// Detect VRM version from glTF JSON document.
///
/// SansaVRM assumes:
/// - VRM 1.0: presence of "extensions.VRMC_vrm"
/// - VRM 0.x: presence of "extensions.VRM"
///
/// If neither extension exists, returns None.
///
/// Note:
/// - This is a heuristic based on extension keys only.
/// - Does not validate specVersion or schema correctness.
/// - Future VRM versions may require additional detection logic.
fn detect_vrm_version(document: &str) -> Option<VrmVersion> {
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
