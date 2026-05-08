// crates/sansavrm-core/src/custom_parameter_registry.rs

//! custom parameter schema registry。
//!
//! 役割:
//! - 登録済み custom parameter schema を保持する。
//! - namespace / name / target_type による schema 解決 API を提供する。
//! - Adapter や Validator が schema-driven に入出力範囲や mapping を判定するための入口を提供する。
//!
//! 注意点:
//! - 本 registry は in-memory の最小実装であり、永続化や外部配布 schema の読み込みは後続実装とする。
//! - 重複登録の禁止、version 別解決、Adapter capability による絞り込みは後続実装とする。

use crate::custom_parameter::CustomParameterSchema;

/// 登録済み custom parameter schema を保持する registry。
///
/// 役割:
/// - `CustomParameterSchema` の一覧を保持する。
/// - namespace / name / target_type に一致する schema を検索する。
///
/// 注意点:
/// - 登録順を保持する。
/// - 同一キーの重複登録は現段階では禁止しない。
#[derive(Debug, Default)]
pub struct CustomParameterRegistry {
    /// 登録済み custom parameter schema 一覧。
    schemas: Vec<CustomParameterSchema>,
}

impl CustomParameterRegistry {
    /// 空の registry を生成する。
    ///
    /// 戻り値:
    /// - `CustomParameterRegistry`: schema 未登録の registry。
    pub fn new() -> Self {
        Self {
            schemas: Vec::new(),
        }
    }

    /// schema を登録する。
    ///
    /// 役割:
    /// - 指定された custom parameter schema を registry に追加する。
    ///
    /// 引数:
    /// - `schema`: 登録対象の custom parameter schema。
    ///
    /// 注意点:
    /// - 現段階では重複チェックを行わない。
    pub fn register(&mut self, schema: CustomParameterSchema) {
        self.schemas.push(schema);
    }

    /// namespace / name / target_type から schema を検索する。
    ///
    /// 役割:
    /// - 登録済み schema から、完全一致する schema を返す。
    ///
    /// 引数:
    /// - `namespace`: 検索対象 namespace。
    /// - `name`: 検索対象 parameter 名。
    /// - `target_type`: 検索対象 target_type。
    ///
    /// 戻り値:
    /// - `Some(&CustomParameterSchema)`: 一致する schema が存在する場合。
    /// - `None`: 一致する schema が存在しない場合。
    pub fn resolve_custom_parameter_schema(
        &self,
        namespace: &str,
        name: &str,
        target_type: &str,
    ) -> Option<&CustomParameterSchema> {
        self.schemas.iter().find(|schema| {
            schema.namespace == namespace
                && schema.name == name
                && schema.target_type == target_type
        })
    }

    /// 登録済み schema 一覧を返す。
    ///
    /// 戻り値:
    /// - `&[CustomParameterSchema]`: 登録済み schema の読み取り専用スライス。
    pub fn schemas(&self) -> &[CustomParameterSchema] {
        &self.schemas
    }
}
