// crates/sansavrm-core/tests/module_api.rs

use sansavrm_core::{
    add_module, remove_module, update_module, Model, Module, ModuleType, Slot, SlotType,
};

fn module(module_id: &str, module_type: ModuleType) -> Module {
    Module {
        module_id: module_id.into(),
        module_type,
        slots: vec![],
        properties: vec![],
    }
}

#[test]
fn core_module_api_001_add_module_should_add_module() {
    let model = Model::new();

    let result = add_module(model, module("module_001", ModuleType::Module));

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.modules.len(), 1);
    assert_eq!(model.modules[0].module_id, "module_001");
}

#[test]
fn core_module_api_002_add_duplicate_module_should_fail() {
    let model = Model::new();

    let result = add_module(model, module("module_001", ModuleType::Module));
    let model = result.data.expect("model should be returned");

    let result = add_module(model, module("module_001", ModuleType::Accessory));

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn core_module_api_003_update_module_should_replace_module() {
    let model = Model::new();

    let result = add_module(model, module("module_001", ModuleType::Module));
    let model = result.data.expect("model should be returned");

    let result = update_module(
        model,
        "module_001",
        module("ignored_id", ModuleType::Accessory),
    );

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.modules[0].module_id, "module_001");
    assert_eq!(model.modules[0].module_type, ModuleType::Accessory);
}

#[test]
fn core_module_api_004_update_unknown_module_should_fail() {
    let model = Model::new();

    let result = update_module(
        model,
        "unknown_module",
        module("ignored_id", ModuleType::Accessory),
    );

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn core_module_api_005_remove_module_should_remove_module() {
    let model = Model::new();

    let result = add_module(model, module("module_001", ModuleType::Module));
    let model = result.data.expect("model should be returned");

    let result = remove_module(model, "module_001");

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert!(model.modules.is_empty());
}

#[test]
fn core_module_api_006_remove_referenced_module_should_fail() {
    let model = Model::new();

    let result = add_module(model, module("module_001", ModuleType::Module));
    let mut model = result.data.expect("model should be returned");

    model.slots.push(Slot {
        slot_id: "slot_001".into(),
        slot_type: SlotType::Structure,
        owner_module_id: "module_001".into(),
        target_slot_types: vec![],
        current_connections: vec![],
        connection_rules: None,
        properties: vec![],
    });

    let result = remove_module(model, "module_001");

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}
