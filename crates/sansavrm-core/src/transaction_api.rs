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
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3g3
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3g4
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_p4n6r7e4
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub original: Model,
    pub working: Model,
    pub active: bool,
}

/// Transaction を開始する。
///
/// 役割:
/// - 開始時点のModelをoriginalとして保持し、workingへ操作対象を格納する。
///
/// 引数:
/// - model: Transaction開始対象Model。
///
/// 戻り値:
/// - CoreResult<Transaction>: 開始済みTransaction。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3g3
pub fn begin(model: Model) -> CoreResult<Transaction> {
    CoreResult::ok(Transaction {
        original: model.clone(),
        working: model,
        active: true,
    })
}

/// Transaction を確定する。
///
/// 役割:
/// - working Model を確定結果として返す。
///
/// 引数:
/// - transaction: 確定対象Transaction。
///
/// 戻り値:
/// - CoreResult<Model>: 確定後Model。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3g4
pub fn commit(transaction: Transaction) -> CoreResult<Model> {
    CoreResult::ok(transaction.working)
}

/// Transaction を破棄し、開始時の Model を返す。
///
/// 役割:
/// - original Model を復元結果として返す。
///
/// 引数:
/// - transaction: 破棄対象Transaction。
///
/// 戻り値:
/// - CoreResult<Model>: Transaction開始時点のModel。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_p4n6r7e4
pub fn rollback(transaction: Transaction) -> CoreResult<Model> {
    CoreResult::ok(transaction.original)
}
