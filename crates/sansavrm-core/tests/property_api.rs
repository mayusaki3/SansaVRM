// crates/sansavrm-core/tests/property_api.rs

use sansavrm_core::{
    add_property, list_properties, remove_property, update_property, Model, Module, ModuleType,
    Property, PropertyContext, PropertyType, Slot, SlotType,
};

fn base_model() -> Model {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "module_001".into(),
        module_type: ModuleType::Module,
        slots: vec!["slot_001".into()],
        properties: vec![],
    });

    model.slots.push(Slot {
        slot_id: "slot_001".into(),
        slot_type: SlotType::Structure,
        owner_module_id: "module_001".into(),
        target_slot_types: vec![],
        current_connections: vec![],
        connection_rules: None,
        properties: vec![],
    });

    model
}

fn metadata_property(property_id: &str, key: &str, value: &str) -> Property {
    Property::from_typed_value(
        property_id,
        key,
        sansavrm_core::PropertyValue::String(value.into()),
        PropertyType::Metadata,
        PropertyContext::Description,
    )
}

/// ModuleへPropertyを追加できることを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_q3p7s6f5
#[test]
fn core_property_api_001_add_property_to_module_should_pass() {
    let model = base_model();
    let property = metadata_property("property_001", "name", "SansaVRM");

    let result = add_property(model, "module_001", property);

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.modules[0].properties.len(), 1);
    assert_eq!(model.modules[0].properties[0].property_id, "property_001");
}

/// SlotへPropertyを追加できることを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_q3p7s6f5
#[test]
fn core_property_api_002_add_property_to_slot_should_pass() {
    let model = base_model();
    let property = metadata_property("property_001", "label", "slot label");

    let result = add_property(model, "slot_001", property);

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.slots[0].properties.len(), 1);
    assert_eq!(model.slots[0].properties[0].property_id, "property_001");
}

/// 存在しないownerへPropertyを追加できないことを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_q3p7s6f5
#[test]
fn core_property_api_003_add_property_unknown_owner_should_fail() {
    let model = base_model();
    let property = metadata_property("property_001", "name", "SansaVRM");

    let result = add_property(model, "unknown_owner", property);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

/// property_idがModel全体で一意であることを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_q3p7s6f5
#[test]
fn core_property_api_004_add_duplicate_property_id_should_fail() {
    let model = base_model();

    let result = add_property(
        model,
        "module_001",
        metadata_property("property_001", "name", "SansaVRM"),
    );
    let model = result.data.expect("model should be returned");

    let result = add_property(
        model,
        "slot_001",
        metadata_property("property_001", "label", "slot label"),
    );

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

/// Propertyを更新でき、既存property_idを維持することを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_q3p7s6f5
#[test]
fn core_property_api_005_update_property_should_replace_value() {
    let model = base_model();

    let result = add_property(
        model,
        "module_001",
        metadata_property("property_001", "name", "old"),
    );
    let model = result.data.expect("model should be returned");

    let result = update_property(
        model,
        "property_001",
        metadata_property("ignored_id", "name", "new"),
    );

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.modules[0].properties[0].property_id, "property_001");
    assert_eq!(
        model.modules[0].properties[0].value,
        sansavrm_core::PropertyValue::String("new".into())
    );
}

/// Propertyを削除できることを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_q3p7s6f5
#[test]
fn core_property_api_006_remove_property_should_remove_property() {
    let model = base_model();

    let result = add_property(
        model,
        "module_001",
        metadata_property("property_001", "name", "SansaVRM"),
    );
    let model = result.data.expect("model should be returned");

    let result = remove_property(model, "property_001");

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert!(model.modules[0].properties.is_empty());
}

/// owner_idに対応するProperty一覧を取得できることを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_q3p7s6f5
#[test]
fn core_property_api_007_list_properties_should_return_owner_properties() {
    let model = base_model();

    let result = add_property(
        model,
        "module_001",
        metadata_property("property_001", "name", "SansaVRM"),
    );
    let model = result.data.expect("model should be returned");

    let result = list_properties(&model, "module_001");

    assert!(result.success);

    let properties = result.data.expect("properties should be returned");
    assert_eq!(properties.len(), 1);
    assert_eq!(properties[0].property_id, "property_001");
}

#[test]
fn core_property_api_008_property_from_typed_string_should_create_legacy_property() {
    let property = Property::from_typed_value(
        "property_001",
        "name",
        sansavrm_core::PropertyValue::String("SansaVRM".into()),
        PropertyType::Metadata,
        PropertyContext::Description,
    );

    assert_eq!(property.property_id, "property_001");
    assert_eq!(property.key, "name");
    assert_eq!(
        property.value,
        sansavrm_core::PropertyValue::String("SansaVRM".into())
    );
    assert_eq!(property.property_type, PropertyType::Metadata);
    assert_eq!(property.context, PropertyContext::Description);
}

#[test]
fn core_property_api_009_property_from_typed_number_should_create_legacy_property() {
    let property = Property::from_typed_value(
        "property_001",
        "weight",
        sansavrm_core::PropertyValue::Number(12.5),
        PropertyType::Metadata,
        PropertyContext::Description,
    );

    assert_eq!(property.value, sansavrm_core::PropertyValue::Number(12.5));
}

#[test]
fn core_property_api_010_property_from_typed_bool_should_create_legacy_property() {
    let property = Property::from_typed_value(
        "property_001",
        "enabled",
        sansavrm_core::PropertyValue::Bool(true),
        PropertyType::Metadata,
        PropertyContext::Description,
    );

    assert_eq!(property.value, sansavrm_core::PropertyValue::Bool(true));
}

#[test]
fn core_property_api_011_property_value_json_should_use_tagged_format() {
    let property = Property::from_typed_value(
        "property_001",
        "mass",
        sansavrm_core::PropertyValue::Number(12.5),
        PropertyType::Metadata,
        PropertyContext::Description,
    );

    let json = serde_json::to_value(&property).expect("property should serialize");

    assert_eq!(json["value"]["type"], "Number");
    assert_eq!(json["value"]["data"], 12.5);
}

#[test]
fn core_property_api_012_property_value_json_should_deserialize_tagged_format() {
    let json = r#"
{
  "property_id": "property_001",
  "key": "mass",
  "value": {
    "type": "Number",
    "data": 12.5
  },
  "property_type": "Metadata",
  "context": "Description"
}
"#;

    let property: Property = serde_json::from_str(json).expect("property should deserialize");

    assert_eq!(property.property_id, "property_001");
    assert_eq!(property.key, "mass");
    assert_eq!(property.value, sansavrm_core::PropertyValue::Number(12.5));
    assert_eq!(property.property_type, PropertyType::Metadata);
    assert_eq!(property.context, PropertyContext::Description);
}
