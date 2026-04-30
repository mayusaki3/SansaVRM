use sansavrm_core::{Connection, ConnectionType, Model};
use sansavrm_mujoco::{export_mujoco, import_mujoco};

#[test]
fn mujoco_adapter_001_import_returns_not_implemented() {
    let result = import_mujoco("<mujoco />".into());

    assert!(!result.success);
}

#[test]
fn mujoco_adapter_002_export_joint_model_returns_not_implemented() {
    let mut model = Model::new();

    model.connections.push(Connection {
        connection_id: "connection_001".into(),
        from_id: "module_001".into(),
        to_id: "module_002".into(),
        connection_type: ConnectionType::Joint,
        enabled: true,
    });

    let result = export_mujoco(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
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
