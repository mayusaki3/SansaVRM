// crates/sansavrm-core/tests/transaction_api.rs

use sansavrm_core::{add_module, begin, commit, rollback, Model, Module, ModuleType};

fn module(module_id: &str) -> Module {
    Module {
        module_id: module_id.into(),
        module_type: ModuleType::Module,
        slots: vec![],
        properties: vec![],
    }
}

/// Transactionを開始できることを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_p4n6r7e4
#[test]
fn core_tx_api_001_begin_should_create_active_transaction() {
    let model = Model::with_id("model_001");

    let result = begin(model);

    assert!(result.success);

    let transaction = result.data.expect("transaction should be returned");
    assert!(transaction.active);
    assert_eq!(transaction.original.model_id, "model_001");
    assert_eq!(transaction.working.model_id, "model_001");
}

/// Transactionをcommitできることを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_p4n6r7e4
#[test]
fn core_tx_api_002_commit_should_return_working_model() {
    let model = Model::with_id("model_001");

    let result = begin(model);
    let mut transaction = result.data.expect("transaction should be returned");

    let result = add_module(transaction.working, module("module_001"));
    transaction.working = result.data.expect("model should be returned");

    let result = commit(transaction);

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert_eq!(model.modules.len(), 1);
    assert_eq!(model.modules[0].module_id, "module_001");
}

/// Transactionをrollbackできることを検証する。
/// @hldocs.ref doc-20260504-000404Z-SV0R#sec_p4n6r7e4
#[test]
fn core_tx_api_003_rollback_should_return_original_model() {
    let model = Model::with_id("model_001");

    let result = begin(model);
    let mut transaction = result.data.expect("transaction should be returned");

    let result = add_module(transaction.working, module("module_001"));
    transaction.working = result.data.expect("model should be returned");

    let result = rollback(transaction);

    assert!(result.success);

    let model = result.data.expect("model should be returned");
    assert!(model.modules.is_empty());
}
