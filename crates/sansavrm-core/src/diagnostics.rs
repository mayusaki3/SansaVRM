// crates/sansavrm-core/src/diagnostics.rs

use serde::{Deserialize, Serialize};

/// diagnostics の重大度。
///
/// 役割:
/// - Validatorおよび変換処理で発生した診断情報の重大度を表現する。
///
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_p3t0w5x9
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// diagnostics の種別。
///
/// 役割:
/// - diagnostics item の分類を表現する。
///
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_p3t0w5x9
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticType {
    Validation,
    Structure,
    Compatibility,
    Rights,
    NonReversibleConversion,
    Runtime,
    Custom,
}

/// Validator 診断コード。
///
/// 役割:
/// - Validatorが検出した問題種別を機械処理可能に表現する。
///
/// 注意:
/// - custom parameter 関連の診断は、汎用 Property 診断と区別する。
///
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_f7a2d9m4
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_g2c9d4x7
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_h1e0f3y8
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_j9g1h2z9
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_n4s1u6v0
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m2
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m3
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m4
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticCode {
    DuplicateId,
    RefNotFound,
    ConnectionInvalid,
    ConnectionRuleViolation,
    StateActionInvalid,
    PropertyTypeMismatch,
    PropertyValueInvalid,
    PropertyClassificationMismatch,
    MujocoConstraintViolation,
    CustomParameterMappingInvalid,
    CustomParameterUnsupported,
    CustomParameterPreserved,
    CustomParameterValueInvalid,
    CustomParameterVersionInvalid,
    CustomParameterFallbackInvalid,
}

/// SansaVRM diagnostics item。
///
/// 役割:
/// - 検証・変換・実行時の問題を記録する。
///
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_p3t0w5x9
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticItem {
    pub diagnostic_type: DiagnosticType,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: String,
}

/// Validator 診断情報。
///
/// 役割:
/// - Validator が検出したエラー・警告を機械処理可能な形式で表現する。
///
/// 注意:
/// - CoreResultとは分離し、ValidatorResult側で利用する。
///
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_f7a2d9m4
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_g2c9d4x7
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_h1e0f3y8
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_j9g1h2z9
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_n4s1u6v0
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationDiagnostic {
    pub code: DiagnosticCode,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub target: Option<String>,
}
