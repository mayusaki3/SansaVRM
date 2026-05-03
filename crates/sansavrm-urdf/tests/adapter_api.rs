use sansavrm_core::{
    Model, Module, ModuleType, Property, PropertyContext, PropertyType, PropertyValue
};
use sansavrm_urdf::{
    classify_urdf_property, export_urdf, import_urdf, UrdfPropertyTarget,
};

#[test]
fn urdf_adapter_001_import_minimal_urdf_should_create_model() {
    let document = r#"
<robot name="test_robot">
    <link name="base_link"/>
    <link name="arm_link"/>
</robot>
"#;

    let result = import_urdf(document.into());

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.model_id, "test_robot");
    assert_eq!(model.modules.len(), 2);
    assert_eq!(model.modules[0].module_id, "base_link");
    assert_eq!(model.modules[1].module_id, "arm_link");
}

#[test]
fn urdf_adapter_002_export_model_should_create_urdf_xml() {
    let mut model = Model::with_id("test_robot");

    model.modules.push(Module {
        module_id: "base_link".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.modules.push(Module {
        module_id: "arm_link".into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    let result = export_urdf(&model);

    assert!(result.success);

    let document = result.data.expect("document should be returned");
    assert!(document.contains(r#"<robot name="test_robot">"#));
    assert!(document.contains(r#"<link name="base_link"/>"#));
    assert!(document.contains(r#"<link name="arm_link"/>"#));
}

#[test]
fn urdf_adapter_003_import_invalid_xml_should_fail() {
    let result = import_urdf("<robot>".into());

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn urdf_adapter_004_import_without_name_should_generate_model_id() {
    let document = r#"
<robot>
    <link name="base_link"/>
</robot>
"#;

    let result = import_urdf(document.into());

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert!(!model.model_id.is_empty());
    assert_eq!(model.modules.len(), 1);
}

#[test]
fn urdf_adapter_005_export_empty_model_should_create_empty_robot() {
    let model = Model::with_id("empty_robot");

    let result = export_urdf(&model);

    assert!(result.success);

    let document = result.data.expect("document should be returned");
    assert!(document.contains(r#"<robot name="empty_robot">"#));
    assert!(document.contains("</robot>"));
}

#[test]
fn urdf_adapter_006_physics_property_should_map_to_inertial() {
    let property = Property::from_typed_value(
        "property_001",
        "mass",
        PropertyValue::Number(1.0),
        PropertyType::Physics,
        PropertyContext::Simulation,
    );

    let target = classify_urdf_property(&property);

    assert_eq!(target, UrdfPropertyTarget::Inertial);
}

#[test]
fn urdf_adapter_007_geometry_rendering_property_should_map_to_visual() {
    let property = Property::from_typed_value(
        "property_001",
        "mesh",
        PropertyValue::String("mesh_001".into()),
        PropertyType::Geometry,
        PropertyContext::Rendering,
    );

    let target = classify_urdf_property(&property);

    assert_eq!(target, UrdfPropertyTarget::Visual);
}

#[test]
fn urdf_adapter_008_geometry_simulation_property_should_map_to_collision() {
    let property = Property::from_typed_value(
        "property_001",
        "collision_shape",
        PropertyValue::String("box".into()),
        PropertyType::Geometry,
        PropertyContext::Simulation,
    );

    let target = classify_urdf_property(&property);

    assert_eq!(target, UrdfPropertyTarget::Collision);
}

#[test]
fn urdf_adapter_009_metadata_property_should_ignore() {
    let property = Property::from_typed_value(
        "property_001",
        "display_name",
        PropertyValue::String("Body".into()),
        PropertyType::Metadata,
        PropertyContext::Description,
    );

    let target = classify_urdf_property(&property);

    assert_eq!(target, UrdfPropertyTarget::Ignore);
}
