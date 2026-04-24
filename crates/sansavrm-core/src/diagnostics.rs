// crates/sansavrm-core/src/diagnostics.rs

use serde::{Deserialize, Serialize};

/// diagnostics の重大度。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Info,
    Normal,
    Warning,
    Critical,
}

/// diagnostics の種別。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticType {
    ValidationError,
    StructureError,
    CompatibilityError,
    RightsWarning,
    NonReversibleConversion,
    RuntimeWarning,
    Custom,
}

/// SansaVRM diagnostics item。
///
/// 役割:
/// - 検証・変換・実行時の問題を記録する。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticItem {
    pub diagnostic_type: DiagnosticType,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: String,
}
