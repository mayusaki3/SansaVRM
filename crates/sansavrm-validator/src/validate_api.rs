// crates/sansavrm-validator/src/validate_api.rs

use sansavrm_core::{CoreResult, Model};

use crate::validate_model;
use crate::{validate_model_with_diagnostics, ValidatorResult};

/// Validator 実行オプション。
///
/// 役割:
/// - CoreAPI仕様の validate(model, options) に対応する。
///
/// 注意点:
/// - 初期実装では strict のみ定義する。
/// - JSON Schema 検証や warnings 制御は後続実装で追加する。
///
/// @hldocs.ref doc-20260504-000204Z-SV0E#sec_e9d7g2v9
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateOptions {
    pub strict: bool,
}

impl Default for ValidateOptions {
    fn default() -> Self {
        Self {
            strict: true,
        }
    }
}

/// Model を検証する。
///
/// 役割:
/// - CoreAPI仕様の `validate(model, options)` に対応する公開API。
///
/// 注意点:
/// - 実処理は既存の `validate_model` に委譲する。
/// - options は初期実装では保持のみで、挙動差分は後続実装で追加する。
///
/// @hldocs.ref doc-20260504-000204Z-SV0E#sec_e9d7g2v9
pub fn validate(model: &Model, _options: ValidateOptions) -> CoreResult<()> {
    validate_model(model)
}

/// Model を diagnostics 付きで検証する。
///
/// 役割:
/// - diagnostics を取得するための公開API。
///
/// @hldocs.ref doc-20260504-000205Z-SV0F#sec_p3t0w5x9
/// @hldocs.ref doc-20260504-000403Z-SV0Q#sec_b2a0d5s6
pub fn validate_diagnostics(model: &Model, _options: ValidateOptions) -> ValidatorResult {
    validate_model_with_diagnostics(model)
}
