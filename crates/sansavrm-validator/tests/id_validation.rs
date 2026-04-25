// crates/sansavrm-validator/tests/id_validation.rs

use sansavrm_core::{Model, Module};
use sansavrm_validator::validate_model;

#[test]
fn validator_id_001_duplicate_module_id_should_fail() {
    let mut model = Model::new();

    // 故意に重複
    model.modules.push(Module {
        module_id: model.model_id.clone(),
        module_type: sansavrm_core::ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    let result = validate_model(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn validator_id_002_duplicate_module_id_should_fail() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "dup".into(),
        module_type: sansavrm_core::ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.modules.push(Module {
        module_id: "dup".into(),
        module_type: sansavrm_core::ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    let result = validate_model(&model);

    assert!(!result.success);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn validator_id_003_unique_ids_should_pass() {
    let mut model = Model::new();

    model.modules.push(Module {
        module_id: "m1".into(),
        module_type: sansavrm_core::ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    model.modules.push(Module {
        module_id: "m2".into(),
        module_type: sansavrm_core::ModuleType::Module,
        slots: vec![],
        properties: vec![],
    });

    let result = validate_model(&model);

    assert!(result.success);
}
