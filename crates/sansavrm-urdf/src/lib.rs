//! SansaVRM URDF adapter.

use sansavrm_core::{CoreResult, Model, SansaVrmError, UrdfDocument};

/// URDF を SansaVRM Model へ import する。
///
/// TODO(trace): 変換仕様 / URDF Import
pub fn import_urdf(_document: UrdfDocument) -> CoreResult<Model> {
    CoreResult::fail(SansaVrmError::InvalidInput(
        "urdf import is not implemented yet".into(),
    ))
}

/// SansaVRM Model を URDF へ export する。
///
/// TODO(trace): 変換仕様 / URDF Export
pub fn export_urdf(_model: &Model) -> CoreResult<UrdfDocument> {
    CoreResult::fail(SansaVrmError::InvalidInput(
        "urdf export is not implemented yet".into(),
    ))
}
