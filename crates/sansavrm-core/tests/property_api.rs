// crates/sansavrm-core/tests/property_api.rs

use sansavrm_core::{
    add_property, list_properties, remove_property, update_property, Model, Module, ModuleType,
    Property, PropertyRole, PropertyType, PropertyValueType, Slot, SlotType,
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
    Property {
        property_id: property_id.into(),
        key: key.into(),
        value: value.into(),
        value_type: PropertyValueType::String,
        property_type: PropertyType::Metadata,
        role: PropertyRole::Module,
    }
}

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

#[test]
fn core_property_api_003_add_property_unknown_owner_should_fail() {
    let model = base_model();
    let property = metadata_property("property_001", "name", "SansaVRM");

    let result = add_property(model, "unknown_owner", property);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

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
    assert_eq!(model.modules[0].properties[0].value, "new");
}

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
