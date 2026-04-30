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
/// TODO(trace): MuJoCo連携仕様 / Connectionタイプ制約
/// TODO(trace): Validator実装仕様 / MuJoCo変換前提整合性
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
fn sort_diagnostics(diagnostics: &mut [ValidationDiagnostic]) {
    diagnostics.sort_by(|a, b| {
        diagnostic_sort_key(a).cmp(&diagnostic_sort_key(b))
    });
}

/// diagnostics ソートキーを生成する。
fn diagnostic_sort_key(diagnostic: &ValidationDiagnostic) -> String {
    format!(
        "{:?}|{:?}|{}|{}",
        diagnostic.severity,
        diagnostic.code,
        diagnostic.target.clone().unwrap_or_default(),
        diagnostic.message
    )
}
