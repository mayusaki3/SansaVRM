// crates/sansavrm-core/src/diagnostics.rs

use serde::{Deserialize, Serialize};

/// diagnostics の重大度。
///
/// TODO(trace): Validator実装仕様 / diagnostics生成
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// diagnostics の種別。
///
/// TODO(trace): Validator実装仕様 / diagnostics生成
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
/// TODO(trace): Validator実装仕様 / diagnostics生成
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
}

/// SansaVRM diagnostics item。
///
/// 役割:
/// - 検証・変換・実行時の問題を記録する。
///
/// TODO(trace): Validator実装仕様 / diagnostics生成
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
/// - 現段階では CoreResult とは未統合。
/// - 後続工程で CoreResult / ValidatorResult へ統合する。
///
/// TODO(trace): Validator実装仕様 / diagnostics生成
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationDiagnostic {
    pub code: DiagnosticCode,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub target: Option<String>,
}
