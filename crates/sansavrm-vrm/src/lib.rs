//! SansaVRM VRM adapter.

use sansavrm_core::{CoreResult, IoOptions, Model, VrmDocument, VrmVersion};

/// VRM を SansaVRM Model へ import する。
///
/// 注意:
/// - 初期実装では VRM を glTF JSON として読み込み、glTF import に委譲する。
/// - VRM 0.x / 1.0 固有メタデータの解釈は後続実装。
///
/// TODO(trace): 変換仕様 / VRM Import
pub fn import_vrm(document: VrmDocument) -> CoreResult<Model> {
    sansavrm_gltf::import_gltf(document)
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
