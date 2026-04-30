//! SansaVRM VRM adapter.

use sansavrm_core::{CoreResult, IoOptions, Model, SansaVrmError, VrmDocument, VrmVersion};

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
/// TODO(trace): 変換仕様 / VRM Export
pub fn export_vrm(
    _model: &Model,
    _version: VrmVersion,
    _options: IoOptions,
) -> CoreResult<VrmDocument> {
    CoreResult::fail(SansaVrmError::InvalidInput(
        "vrm export is not implemented yet".into(),
    ))
}
