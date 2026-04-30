use sansavrm_core::{Connection, ConnectionType, Model};
use sansavrm_mujoco::{export_mujoco, import_mujoco};

#[test]
fn mujoco_adapter_001_import_returns_not_implemented() {
    let result = import_mujoco("<mujoco />".into());

    assert!(!result.success);
}

#[test]
fn mujoco_adapter_002_export_joint_model_should_create_mjcf() {
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
    assert!(document.contains(r#"<mujoco model="test_model">"#));
    assert!(document.contains(r#"<worldbody>"#));
    assert!(document.contains(r#"<joint name="joint_001"/>"#));
}

#[test]
fn mujoco_adapter_003_export_non_joint_model_should_fail_before_export() {
    let mut model = Model::new();

    model.connections.push(Connection {
        connection_id: "connection_001".into(),
        from_id: "module_001".into(),
        to_id: "module_002".into(),
        connection_type: ConnectionType::Attach,
        enabled: true,
    });

    let result = export_mujoco(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn mujoco_adapter_004_export_empty_model_should_create_empty_mjcf() {
    let model = Model::with_id("empty_model");

    let result = export_mujoco(&model);

    assert!(result.success);

    let document = result.data.expect("document should be returned");
    assert!(document.contains(r#"<mujoco model="empty_model">"#));
    assert!(document.contains(r#"<worldbody>"#));
    assert!(document.contains(r#"</mujoco>"#));
}
