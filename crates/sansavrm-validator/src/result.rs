use sansavrm_core::ValidationDiagnostic;

/// Validator の実行結果。
///
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_p3t0w5x9
/// @hldocs.ref doc-20260504-000403Z-SV0Q#sec_b2a0d5s6
#[derive(Debug, Clone)]
pub struct ValidatorResult {
    pub success: bool,
    pub diagnostics: Vec<ValidationDiagnostic>,
}
