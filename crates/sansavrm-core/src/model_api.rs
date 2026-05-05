// crates/sansavrm-core/src/model_api.rs

use crate::{CoreResult, Model, SansaVrmError};

/// Model 作成入力。
///
/// 役割:
/// - create_model の入力値を表現する。
///
/// 注意点:
/// - model_id が None の場合は Model 側でIDを自動生成する。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_k8j2m1a0
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CreateModelInput {
    pub model_id: Option<String>,
}

/// Model を生成する。
///
/// 役割:
/// - model_id 指定ありの場合は指定IDで Model を作成する。
/// - model_id 指定なしの場合は自動生成する。
///
/// 引数:
/// - input: Model作成入力。
///
/// 戻り値:
/// - CoreResult<Model>: 作成されたModel。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_k8j2m1a0
pub fn create_model(input: CreateModelInput) -> CoreResult<Model> {
    let model = match input.model_id {
        Some(model_id) => Model::with_id(model_id),
        None => Model::new(),
    };

    CoreResult::ok(model)
}

/// JSON 文書から Model を読み込む。
///
/// 役割:
/// - JSON 文字列を Model に deserialize する。
///
/// 注意点:
/// - JSON Schema 検証および Validator 統合は後続ステップで追加する。
///
/// 引数:
/// - document: Model JSON文書。
///
/// 戻り値:
/// - CoreResult<Model>: 読み込まれたModel、または解析エラー。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3d2
pub fn load_model(document: impl AsRef<str>) -> CoreResult<Model> {
    match serde_json::from_str::<Model>(document.as_ref()) {
        Ok(model) => CoreResult::ok(model),
        Err(error) => CoreResult::fail(SansaVrmError::InvalidInput(format!(
            "Failed to load model JSON: {}",
            error
        ))),
    }
}

/// Model を JSON 文書へ出力する。
///
/// 役割:
/// - Model を pretty JSON として serialize する。
///
/// 注意点:
/// - Schema準拠保証は後続ステップで追加する。
///
/// 引数:
/// - model: 出力対象Model。
///
/// 戻り値:
/// - CoreResult<String>: JSON文字列、または出力エラー。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3d3
pub fn export_model(model: &Model) -> CoreResult<String> {
    match serde_json::to_string_pretty(model) {
        Ok(document) => CoreResult::ok(document),
        Err(error) => CoreResult::fail(SansaVrmError::InvalidInput(format!(
            "Failed to export model JSON: {}",
            error
        ))),
    }
}
