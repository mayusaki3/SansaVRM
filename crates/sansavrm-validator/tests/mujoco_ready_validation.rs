// crates/sansavrm-validator/tests/mujoco_ready_validation.rs

use sansavrm_core::{
    Connection, ConnectionType, DiagnosticCode, Model,
};
use sansavrm_validator::validate_mujoco_ready;

#[test]
fn validator_mujoco_001_joint_connection_should_pass() {
    let mut model = Model::new();

    model.connections.push(Connection {
        connection_id: "connection_001".into(),
        from_id: "module_001".into(),
        to_id: "module_002".into(),
        connection_type: ConnectionType::Joint,
        enabled: true,
    });

    let result = validate_mujoco_ready(&model);

    assert!(result.success);
    assert!(result.diagnostics.is_empty());
}

#[test]
fn validator_mujoco_002_attach_connection_should_fail() {
    let mut model = Model::new();

    model.connections.push(Connection {
        connection_id: "connection_001".into(),
        from_id: "module_001".into(),
        to_id: "module_002".into(),
        connection_type: ConnectionType::Attach,
        enabled: true,
    });

    let result = validate_mujoco_ready(&model);

    assert!(!result.success);
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(
        result.diagnostics[0].code,
        DiagnosticCode::MujocoConstraintViolation
    );
}

#[test]
fn validator_mujoco_003_diagnostics_order_should_be_stable() {
    let mut model = Model::new();

    model.connections.push(Connection {
        connection_id: "connection_b".into(),
        from_id: "module_001".into(),
        to_id: "module_002".into(),
        connection_type: ConnectionType::Attach,
        enabled: true,
    });

    model.connections.push(Connection {
        connection_id: "connection_a".into(),
        from_id: "module_003".into(),
        to_id: "module_004".into(),
        connection_type: ConnectionType::Logical,
        enabled: true,
    });

    let result = validate_mujoco_ready(&model);

    assert!(!result.success);
    assert_eq!(result.diagnostics.len(), 2);
    assert_eq!(result.diagnostics[0].target.as_deref(), Some("connection_a"));
    assert_eq!(result.diagnostics[1].target.as_deref(), Some("connection_b"));
}
