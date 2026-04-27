use sansavrm_core::ValidationDiagnostic;

/// Validator の実行結果。
///
/// TODO(trace): Validator実装仕様 / diagnostics出力
#[derive(Debug, Clone)]
pub struct ValidatorResult {
    pub success: bool,
    pub diagnostics: Vec<ValidationDiagnostic>,
}
