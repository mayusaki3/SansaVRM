//! SansaVRM MuJoCo adapter.

use sansavrm_core::{ConnectionType, CoreResult, MjcfDocument, Model, SansaVrmError};
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
/// - 初期実装では Joint Connection を MJCF joint として出力する。
/// - body 階層、geom、actuator、sensor 生成は後続実装。
///
/// TODO(trace): MuJoCo連携仕様 / MuJoCo Export
pub fn export_mujoco(model: &Model) -> CoreResult<MjcfDocument> {
    let validation = validate_mujoco_ready(model);

    if !validation.success {
        return CoreResult::fail(SansaVrmError::InvalidInput(
            "model is not ready for MuJoCo export".into(),
        ));
    }

    let mut document = format!(r#"<mujoco model="{}">"#, model.model_id);
    document.push_str("\n    <worldbody>");

    for connection in &model.connections {
        if connection.connection_type == ConnectionType::Joint {
            document.push_str(&format!(
                r#"
        <joint name="{}"/>"#,
                connection.connection_id
            ));
        }
    }

    document.push_str("\n    </worldbody>");
    document.push_str("\n</mujoco>");

    CoreResult::ok(document)
}