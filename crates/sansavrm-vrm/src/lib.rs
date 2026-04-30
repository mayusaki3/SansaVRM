//! SansaVRM VRM adapter.

use sansavrm_core::{CoreResult, IoOptions, Model, SansaVrmError, VrmDocument, VrmVersion};

/// VRM を SansaVRM Model へ import する。
///
/// TODO(trace): 変換仕様 / VRM Import
pub fn import_vrm(_document: VrmDocument) -> CoreResult<Model> {
    CoreResult::fail(SansaVrmError::InvalidInput(
        "vrm import is not implemented yet".into(),
    ))
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
