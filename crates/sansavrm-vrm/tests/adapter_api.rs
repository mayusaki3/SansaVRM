use sansavrm_core::{IoOptions, Model, VrmVersion};
use sansavrm_vrm::{export_vrm, import_vrm};

#[test]
fn vrm_adapter_001_import_returns_not_implemented() {
    let result = import_vrm("{}".into());
    assert!(!result.success);
}

#[test]
fn vrm_adapter_002_export_vrm_1_0_returns_not_implemented() {
    let model = Model::new();
    let result = export_vrm(&model, VrmVersion::V1_0, IoOptions::default());
    assert!(!result.success);
}
