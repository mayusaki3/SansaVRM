use sansavrm_vrm::{import_vrm};

#[test]
fn vrm_adapter_003_import_invalid_json_should_fail() {
    let result = import_vrm("{ invalid json".into());

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}
