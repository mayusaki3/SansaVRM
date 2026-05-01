//! SansaVRM VRM adapter.

use sansavrm_core::{CoreResult, IoOptions, Model, VrmDocument, VrmVersion};
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

    let result = sansavrm_gltf::import_gltf(document);

    if !result.success {
        return result;
    }

    let mut model = result.data.expect("model should be returned");
    model.vrm_version = version;

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

    apply_vrm_extension(&mut value, version);

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
