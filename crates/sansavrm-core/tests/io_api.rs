// crates/sansavrm-core/tests/io_api.rs

use sansavrm_core::{
    export_gltf, export_mujoco, export_urdf, export_vrm, import_gltf, import_mujoco, import_urdf,
    import_vrm, IoOptions, Model, VrmVersion,
};

#[test]
fn core_io_api_001_import_gltf_returns_not_implemented() {
    let result = import_gltf("{}".into());

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn core_io_api_002_export_gltf_returns_not_implemented() {
    let model = Model::new();

    let result = export_gltf(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn core_io_api_003_import_vrm_returns_not_implemented() {
    let result = import_vrm("{}".into());

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn core_io_api_004_export_vrm_1_0_returns_not_implemented() {
    let model = Model::new();

    let result = export_vrm(&model, VrmVersion::V1_0, IoOptions::default());

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn core_io_api_005_export_vrm_0x_returns_not_implemented() {
    let model = Model::new();

    let result = export_vrm(&model, VrmVersion::V0x, IoOptions::default());

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn core_io_api_006_import_urdf_returns_not_implemented() {
    let result = import_urdf("<robot />".into());

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn core_io_api_007_export_urdf_returns_not_implemented() {
    let model = Model::new();

    let result = export_urdf(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn core_io_api_008_import_mujoco_returns_not_implemented() {
    let result = import_mujoco("<mujoco />".into());

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn core_io_api_009_export_mujoco_returns_not_implemented() {
    let model = Model::new();

    let result = export_mujoco(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}
