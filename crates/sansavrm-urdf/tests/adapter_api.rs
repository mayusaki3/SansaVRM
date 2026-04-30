use sansavrm_core::Model;
use sansavrm_urdf::{export_urdf, import_urdf};

#[test]
fn urdf_adapter_001_import_returns_not_implemented() {
    let result = import_urdf("<robot />".into());
    assert!(!result.success);
}

#[test]
fn urdf_adapter_002_export_returns_not_implemented() {
    let model = Model::new();
    let result = export_urdf(&model);
    assert!(!result.success);
}
