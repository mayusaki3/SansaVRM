// crates/sansavrm-mujoco/tests/roundtrip.rs

use sansavrm_core::{Connection, ConnectionType, Model};
use sansavrm_mujoco::export_mujoco;

#[test]
fn mujoco_roundtrip_001_export_structure_should_be_valid_mjcf() {
    let mut model = Model::with_id("test_model");

    model.connections.push(Connection {
        connection_id: "joint_001".into(),
        from_id: "module_001".into(),
        to_id: "module_002".into(),
        connection_type: ConnectionType::Joint,
        enabled: true,
    });

    let result = export_mujoco(&model);
    assert!(result.success);

    let document = result.data.expect("document should be returned");

    // 構造検証（RoundTrip代替）
    assert!(document.contains("<mujoco"));
    assert!(document.contains("<worldbody>"));
    assert!(document.contains("<joint name=\"joint_001\"/>"));
    assert!(document.contains("</mujoco>"));
}
