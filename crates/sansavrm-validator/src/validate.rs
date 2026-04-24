// validate.rs

use sansavrm_core::{Model, SansaVrmError, CoreResult};

/// Model の基本検証
///
/// TODO(trace): Validator実装仕様 / 基本検証
pub fn validate_model(model: &Model) -> CoreResult<()> {
    // 現段階では常にOK
    CoreResult::ok(())
}
