// crates/sansavrm-validator/tests/basic.rs

use sansavrm_core::Model;
use sansavrm_validator::validate_model;

#[test]
fn empty_model_is_valid() {
    let model = Model::new();

    let result = validate_model(&model);

    assert!(result.success);
}
