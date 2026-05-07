// crates/sansavrm-validator/src/mujoco_api.rs

use sansavrm_core::{
    ConnectionType, DiagnosticCode, DiagnosticSeverity, Model, ValidationDiagnostic,
};

use crate::ValidatorResult;

/// MuJoCo 変換前提を検証する。
///
/// 役割:
/// - MuJoCo / MJCF 変換に入る前に、SansaVRM Model が MuJoCo 変換可能な前提を満たすか検証する。
///
/// 注意点:
/// - 汎用 validate には含めない。
/// - MuJoCo は body がツリー構造であるため、Connection 制約は変換前検証として分離する。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_w7v5y0m1
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_n4s1u6v0
pub fn validate_mujoco_ready(model: &Model) -> ValidatorResult {
    let mut diagnostics = Vec::new();

    validate_mujoco_connection_types(model, &mut diagnostics);
    sort_diagnostics(&mut diagnostics);

    ValidatorResult {
        success: diagnostics.is_empty(),
        diagnostics,
    }
}

/// MuJoCo対象 Connection type を検証する。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_w7v5y0m1
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_n4s1u6v0
fn validate_mujoco_connection_types(
    model: &Model,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    for connection in &model.connections {
        if connection.connection_type != ConnectionType::Joint {
            diagnostics.push(ValidationDiagnostic {
                code: DiagnosticCode::MujocoConstraintViolation,
                severity: DiagnosticSeverity::Error,
                message: format!(
                    "MuJoCo export supports only Joint connection, but {} is {:?}",
                    connection.connection_id,
                    connection.connection_type
                ),
                target: Some(connection.connection_id.clone()),
            });
        }
    }
}

/// diagnostics の出力順を安定化する。
///
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_p3t0w5x9
fn sort_diagnostics(diagnostics: &mut [ValidationDiagnostic]) {
    diagnostics.sort_by(|a, b| {
        diagnostic_sort_key(a).cmp(&diagnostic_sort_key(b))
    });
}

/// diagnostics ソートキーを生成する。
///
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_p3t0w5x9
fn diagnostic_sort_key(diagnostic: &ValidationDiagnostic) -> String {
    format!(
        "{:?}|{:?}|{}|{}",
        diagnostic.severity,
        diagnostic.code,
        diagnostic.target.clone().unwrap_or_default(),
        diagnostic.message
    )
}
