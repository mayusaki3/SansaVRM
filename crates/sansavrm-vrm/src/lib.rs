//! SansaVRM VRM adapter.

use sansavrm_core::{CoreResult, IoOptions, Model, VrmDocument, VrmVersion};
use serde_json::Value;

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
/// - 初期実装では VRM を glTF JSON として出力し、glTF export に委譲する。
/// - `version` は API として受け取るが、VRM 0.x / 1.0 固有 extension の出力は後続実装。
/// - `options` は後続実装で使用する。
///
/// TODO(trace): 変換仕様 / VRM Export
pub fn export_vrm(
    model: &Model,
    _version: VrmVersion,
    _options: IoOptions,
) -> CoreResult<VrmDocument> {
    sansavrm_gltf::export_gltf(model)
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
