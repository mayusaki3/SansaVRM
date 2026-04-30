//! SansaVRM MuJoCo adapter.

use sansavrm_core::{CoreResult, MjcfDocument, Model, SansaVrmError};
use sansavrm_validator::validate_mujoco_ready;

/// MuJoCo / MJCF を SansaVRM Model へ import する。
///
/// TODO(trace): MuJoCo連携仕様 / MuJoCo Import
pub fn import_mujoco(_document: MjcfDocument) -> CoreResult<Model> {
    CoreResult::fail(SansaVrmError::InvalidInput(
        "mujoco import is not implemented yet".into(),
    ))
}

/// SansaVRM Model を MuJoCo / MJCF へ export する。
///
/// 注意点:
/// - 初期実装では MuJoCo 変換前提検証のみ実行する。
/// - 実際の MJCF 生成は後続実装。
///
/// TODO(trace): MuJoCo連携仕様 / MuJoCo Export
pub fn export_mujoco(model: &Model) -> CoreResult<MjcfDocument> {
    let validation = validate_mujoco_ready(model);

    if !validation.success {
        return CoreResult::fail(SansaVrmError::InvalidInput(
            "model is not ready for MuJoCo export".into(),
        ));
    }

    CoreResult::fail(SansaVrmError::InvalidInput(
        "mujoco export is not implemented yet".into(),
    ))
}
