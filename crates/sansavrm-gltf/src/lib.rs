//! SansaVRM glTF adapter.

use sansavrm_core::{CoreResult, GltfDocument, Model, SansaVrmError};

/// glTF を SansaVRM Model へ import する。
///
/// TODO(trace): 変換仕様 / glTF Import
pub fn import_gltf(_document: GltfDocument) -> CoreResult<Model> {
    CoreResult::fail(SansaVrmError::InvalidInput(
        "gltf import is not implemented yet".into(),
    ))
}

/// SansaVRM Model を glTF へ export する。
///
/// TODO(trace): 変換仕様 / glTF Export
pub fn export_gltf(_model: &Model) -> CoreResult<GltfDocument> {
    CoreResult::fail(SansaVrmError::InvalidInput(
        "gltf export is not implemented yet".into(),
    ))
}
