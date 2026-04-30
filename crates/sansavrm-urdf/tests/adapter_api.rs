use sansavrm_core::Model;
use sansavrm_urdf::{export_urdf, import_urdf};

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
fn urdf_adapter_002_export_returns_not_implemented() {
    let model = Model::new();
    let result = export_urdf(&model);
    assert!(!result.success);
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
