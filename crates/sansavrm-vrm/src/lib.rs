//! SansaVRM VRM adapter.
//!
//! @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p3

mod common;
mod vrm1;
mod vrm0;

use sansavrm_core::{
    CoreResult, IoOptions, Model, VrmDocument, VrmVersion,
};
use serde_json::{json, Value};
use vrm1::{apply_vrm1_humanoid, apply_vrm1_meta, import_vrm1_humanoid, import_vrm1_meta};
use vrm0::{apply_vrm0_humanoid, apply_vrm0_meta, import_vrm0_humanoid, import_vrm0_meta};
use common::detect_vrm_version;

/// VRM を SansaVRM Model へ import する。
///
/// 注意:
/// - 初期実装では VRM を glTF JSON として読み込み、glTF import に委譲する。
/// - VRM 0.x / 1.0 固有メタデータの解釈は後続実装。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p3
pub fn import_vrm(document: VrmDocument) -> CoreResult<Model> {
    let version = detect_vrm_version(&document);

    let result = sansavrm_gltf::import_gltf(document.clone());
    
    if !result.success {
        return result;
    }

    let mut model = result.data.expect("model should be returned");
    model.vrm_version = version.clone();

    let parsed_value = serde_json::from_str::<Value>(&document).ok();

    // --- VRM meta import ---
    if let Some(value) = &parsed_value {
        match version {
            Some(VrmVersion::V1_0) => import_vrm1_meta(&mut model, value),
            Some(VrmVersion::V0x) => import_vrm0_meta(&mut model, value),
            None => {}
        }
    }

    // --- VRM humanoid import ---
    import_vrm1_humanoid(&mut model, &document, version.clone());
    import_vrm0_humanoid(&mut model, &document, version);

    CoreResult::ok(model)
}

/// SansaVRM Model を VRM へ export する。
///
/// 注意:
/// - 初期実装では glTF export 結果に VRM version extension を追加する。
/// - `version` は出力対象の VRM 系列を明示する。
/// - `options` は後続実装で使用する。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p3
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

    // --- VRM extension export ---
    apply_vrm_extension(&mut value, version.clone());

    // --- VRM meta export ---
    match version {
        VrmVersion::V1_0 => apply_vrm1_meta(&mut value, model),
        VrmVersion::V0x => apply_vrm0_meta(&mut value, model),
    }
    
    // --- VRM humanoid export ---
    apply_vrm1_humanoid(&mut value, model, version.clone());
    apply_vrm0_humanoid(&mut value, model, version);

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
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p3
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
