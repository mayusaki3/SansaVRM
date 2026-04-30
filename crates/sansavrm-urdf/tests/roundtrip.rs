// crates/sansavrm-urdf/tests/roundtrip.rs

use sansavrm_urdf::{export_urdf, import_urdf};

#[test]
fn urdf_roundtrip_001_import_export_import_should_keep_links() {
    let input = r#"
<robot name="test_robot">
    <link name="base_link"/>
    <link name="arm_link"/>
</robot>
"#;

    let imported = import_urdf(input.into());
    assert!(imported.success);

    let model = imported.data.expect("model should be returned");
    assert_eq!(model.model_id, "test_robot");
    assert_eq!(model.modules.len(), 2);

    let exported = export_urdf(&model);
    assert!(exported.success);

    let exported_document = exported.data.expect("document should be returned");

    let reimported = import_urdf(exported_document);
    assert!(reimported.success);

    let reimported_model = reimported.data.expect("model should be returned");

    assert_eq!(reimported_model.model_id, "test_robot");
    assert_eq!(reimported_model.modules.len(), 2);
    assert_eq!(reimported_model.modules[0].module_id, "base_link");
    assert_eq!(reimported_model.modules[1].module_id, "arm_link");
}
