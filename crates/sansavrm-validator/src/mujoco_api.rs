// crates/sansavrm-validator/src/mujoco_api.rs

//! MuJoCo 変換前提検証 API。
//!
//! 役割:
//! - SansaVRM Model が MuJoCo / MJCF 連携に利用可能な前提を満たすか検証する。
//! - MuJoCo export 前に、Connection type などの構造制約を検出する。
//! - 将来の custom parameter schema 検証、io_scope 検証、mjcf_mapping 検証、adapter_artifact 検証の入口を提供する。
//!
//! 注意点:
//! - 汎用 Validator とは別に、MuJoCo 連携用の前提検証として扱う。
//! - SansaVRM 本体は MuJoCo 実行ランタイムに依存しない。
//! - MJCF 直接入出力可否は実装側の推測ではなく、登録スキーマに基づいて判定する。

use sansavrm_core::{
    ConnectionType, DiagnosticCode, DiagnosticSeverity, Model, ValidationDiagnostic,
};

use crate::ValidatorResult;

/// MuJoCo 変換前提を検証する。
///
/// 役割:
/// - MuJoCo / MJCF 変換に入る前に、SansaVRM Model が MuJoCo 変換可能な前提を満たすか検証する。
/// - MuJoCo Adapter が参照する前提条件を検証する。
/// - 将来の custom parameter schema では io_scope / mjcf_mapping / adapter_artifact の整合性検証を追加する。
///
/// 引数:
/// - `model`: 検証対象の SansaVRM Model。
///
/// 戻り値:
/// - `ValidatorResult`: 検証成功可否と diagnostics 一覧。
///
/// 注意点:
/// - 汎用 validate には含めない。
/// - MuJoCo は body がツリー構造であるため、Connection 制約は変換前検証として分離する。
/// - MJCF 直接入出力可否は、実装側の推測ではなく登録スキーマで判定する。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_w7v5y0m1
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_w7v5y0m2
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_w7v5y0m3
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_w7v5y0m4
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
/// 役割:
/// - MuJoCo export 対象として許容される Connection type のみで構成されているか確認する。
/// - Joint 以外の Connection が含まれる場合、diagnostics にエラーを追加する。
///
/// 引数:
/// - `model`: 検証対象の SansaVRM Model。
/// - `diagnostics`: 検証結果を追加する diagnostics バッファ。
///
/// 注意点:
/// - 現段階では `ConnectionType::Joint` のみを MuJoCo export 対象とする。
/// - 本関数は diagnostics を追加する副作用を持つ。
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
/// 役割:
/// - diagnostics の順序を deterministic にし、テスト結果と出力結果を安定化する。
///
/// 引数:
/// - `diagnostics`: 並び替え対象の diagnostics スライス。
///
/// 注意点:
/// - 本関数は引数のスライスを直接並び替える。
///
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_p3t0w5x9
fn sort_diagnostics(diagnostics: &mut [ValidationDiagnostic]) {
    diagnostics.sort_by(|a, b| {
        diagnostic_sort_key(a).cmp(&diagnostic_sort_key(b))
    });
}

/// diagnostics ソートキーを生成する。
///
/// 役割:
/// - severity / code / target / message を連結し、安定した比較キーを生成する。
///
/// 引数:
/// - `diagnostic`: ソートキー生成対象の diagnostic。
///
/// 戻り値:
/// - `String`: diagnostics 並び替えに使用する比較キー。
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
