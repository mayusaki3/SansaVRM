//! SansaVRM MuJoCo adapter.
//!
//! @hldocs.ref doc-20260504-000405Z-SV0S#sec_w7v5y0m1
//! @hldocs.ref doc-20260504-000203Z-SV0D#sec_c6t5v8s3

use sansavrm_core::{
    ConnectionType, CoreResult, MjcfDocument, Model, Property, PropertyContext, PropertyType,
    SansaVrmError,
};
use sansavrm_validator::validate_mujoco_ready;

/// MuJoCo export 時に Property をどの MJCF 要素へ反映するかを表す。
///
/// 役割:
/// - SansaVRM Property の property_type / context を MuJoCo の出力対象へ分類する。
///
/// 注意点:
/// - この分類は export 前段の判定であり、実際の MJCF 生成は別処理で行う。
/// - property_type を優先し、context は補助判定として扱う。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_w7v5y0m1
/// @hldocs.ref doc-20260504-000203Z-SV0D#sec_c6t5v8s3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MujocoPropertyTarget {
    Geom,
    Actuator,
    Sensor,
    Ignore,
}

/// Property を MuJoCo 出力対象へ分類する。
///
/// 役割:
/// - Physics / Geometry / Material / Texture 系 Property を geom 生成対象に分類する。
/// - Actuator 系 Property を actuator 生成対象に分類する。
/// - Sensor 系 Property を sensor 生成対象に分類する。
/// - MuJoCo 出力対象外の Property を Ignore に分類する。
///
/// 引数:
/// - `property`: 分類対象の Property
///
/// 戻り値:
/// - `MujocoPropertyTarget`: MuJoCo 出力対象分類
///
/// 注意点:
/// - property_type を最優先する。
/// - context は property_type が補助判定を必要とする場合のみ使用する。
/// - Metadata / Rights / Revenue / Compatibility 等は現段階では Ignore とする。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_w7v5y0m1
/// @hldocs.ref doc-20260504-000203Z-SV0D#sec_c6t5v8s3
pub fn classify_mujoco_property(property: &Property) -> MujocoPropertyTarget {
    match property.property_type {
        PropertyType::Physics
        | PropertyType::Geometry
        | PropertyType::Material
        | PropertyType::Texture => MujocoPropertyTarget::Geom,

        PropertyType::Actuator => MujocoPropertyTarget::Actuator,

        PropertyType::Sensor => MujocoPropertyTarget::Sensor,

        PropertyType::Control => match property.context {
            PropertyContext::Execution | PropertyContext::Simulation => {
                MujocoPropertyTarget::Actuator
            }
            PropertyContext::IO => MujocoPropertyTarget::Sensor,
            _ => MujocoPropertyTarget::Ignore,
        },

        _ => MujocoPropertyTarget::Ignore,
    }
}

/// MuJoCo / MJCF を SansaVRM Model へ import する。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_w7v5y0m1
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
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_w7v5y0m1
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