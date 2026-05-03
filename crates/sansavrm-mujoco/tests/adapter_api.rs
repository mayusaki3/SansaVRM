use sansavrm_core::{
    Connection, ConnectionType, Model, Property, PropertyContext, PropertyType, PropertyValue
};
use sansavrm_mujoco::{
    classify_mujoco_property, export_mujoco, import_mujoco, MujocoPropertyTarget,
};

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

#[test]
fn mujoco_adapter_005_physics_property_should_map_to_geom() {
    let property = Property::from_typed_value(
        "property_001",
        "mass",
        PropertyValue::Number(1.0),
        PropertyType::Physics,
        PropertyContext::Simulation,
    );

    let target = classify_mujoco_property(&property);

    assert_eq!(target, MujocoPropertyTarget::Geom);
}

#[test]
fn mujoco_adapter_006_actuator_property_should_map_to_actuator() {
    let property = Property::from_typed_value(
        "property_001",
        "torque",
        PropertyValue::Number(1.0),
        PropertyType::Actuator,
        PropertyContext::Execution,
    );

    let target = classify_mujoco_property(&property);

    assert_eq!(target, MujocoPropertyTarget::Actuator);
}

#[test]
fn mujoco_adapter_007_sensor_property_should_map_to_sensor() {
    let property = Property::from_typed_value(
        "property_001",
        "position",
        PropertyValue::String("jointpos".into()),
        PropertyType::Sensor,
        PropertyContext::IO,
    );

    let target = classify_mujoco_property(&property);

    assert_eq!(target, MujocoPropertyTarget::Sensor);
}

#[test]
fn mujoco_adapter_008_metadata_property_should_ignore() {
    let property = Property::from_typed_value(
        "property_001",
        "display_name",
        PropertyValue::String("Body".into()),
        PropertyType::Metadata,
        PropertyContext::Description,
    );

    let target = classify_mujoco_property(&property);

    assert_eq!(target, MujocoPropertyTarget::Ignore);
}
