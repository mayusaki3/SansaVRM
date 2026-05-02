// crates/sansavrm-core/src/transaction_api.rs

use crate::{CoreResult, Model};

/// Transaction。
///
/// 役割:
/// - CoreAPI の begin / commit / rollback に対応する最小実装。
///
/// 注意点:
/// - 初期実装ではインメモリスナップショットとして扱う。
/// - 永続化・差分管理・ネストTransactionは後続実装。
///
/// TODO(trace): CoreAPI仕様 / Transaction API
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub original: Model,
    pub working: Model,
    pub active: bool,
}

/// Transaction を開始する。
///
/// TODO(trace): CoreAPI仕様 / begin
pub fn begin(model: Model) -> CoreResult<Transaction> {
    CoreResult::ok(Transaction {
        original: model.clone(),
        working: model,
        active: true,
    })
}

/// Transaction を確定する。
///
/// TODO(trace): CoreAPI仕様 / commit
pub fn commit(transaction: Transaction) -> CoreResult<Model> {
    CoreResult::ok(transaction.working)
}

/// Transaction を破棄し、開始時の Model を返す。
///
/// TODO(trace): CoreAPI仕様 / rollback
pub fn rollback(transaction: Transaction) -> CoreResult<Model> {
    CoreResult::ok(transaction.original)
}
