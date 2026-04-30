// crates/sansavrm-gltf/tests/roundtrip.rs

use sansavrm_gltf::{export_gltf, import_gltf};

#[test]
fn gltf_roundtrip_001_import_export_import_should_keep_nodes() {
    let input = r#"
{
  "asset": {
    "version": "2.0"
  },
  "nodes": [
    { "name": "Root" },
    { "name": "Arm" }
  ]
}
"#;

    let imported = import_gltf(input.into());
    assert!(imported.success);

    let model = imported.data.expect("model should be returned");
    assert_eq!(model.modules.len(), 2);

    let exported = export_gltf(&model);
    assert!(exported.success);

    let exported_document = exported.data.expect("document should be returned");

    let reimported = import_gltf(exported_document);
    assert!(reimported.success);

    let reimported_model = reimported.data.expect("model should be returned");

    assert_eq!(reimported_model.modules.len(), 2);
    assert_eq!(reimported_model.modules[0].module_id, "Root");
    assert_eq!(reimported_model.modules[1].module_id, "Arm");
}
