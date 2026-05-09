// crates/sansavrm-validator/src/custom_parameter_api.rs

//! custom parameter schema 検証 API。
//!
//! 役割:
//! - custom parameter schema の io_scope / mjcf_mapping / adapter_artifact 整合性を検証する。
//! - unsupported / preserve_only / source_raw を diagnostics として表現する。
//! - registry に登録された schema 一覧をまとめて検証する。
//!
//! 注意点:
//! - 本モジュールは schema 構造と registry 整合性の検証を担当する。
//! - 実際の値検証、MuJoCo version 比較、Adapter version 比較は後続実装で扱う。

use sansavrm_core::{
    CustomParameterIoScope, CustomParameterRegistry, CustomParameterSchema, DiagnosticCode,
    DiagnosticSeverity, ValidationDiagnostic,
};

use crate::ValidatorResult;

/// custom parameter schema を検証する。
///
/// 役割:
/// - `io_scope` と `mjcf_mapping` / `adapter_artifact` の整合性を検証する。
/// - unsupported / preserve_only / source_raw の扱いを diagnostics に反映できる入口を提供する。
///
/// 引数:
/// - `schema`: 検証対象の custom parameter schema。
///
/// 戻り値:
/// - `ValidatorResult`: 成功可否と diagnostics。
///
/// 注意点:
/// - ここでは schema 構造の整合性のみを検証する。
/// - 実際の値検証、MuJoCo version 比較、Adapter version 比較は後続実装で扱う。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m2
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m3
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m4
pub fn validate_custom_parameter_schema(schema: &CustomParameterSchema) -> ValidatorResult {
    let mut diagnostics = Vec::new();

    validate_required_mappings(schema, &mut diagnostics);
    validate_non_output_scopes(schema, &mut diagnostics);

    ValidatorResult {
        success: diagnostics
            .iter()
            .all(|diagnostic| diagnostic.severity != DiagnosticSeverity::Error),
        diagnostics,
    }
}

/// custom parameter registry を検証する。
///
/// 役割:
/// - registry に登録された全 schema を検証する。
/// - registry を利用する Adapter / Validator が schema-driven に判定できる状態か確認する。
///
/// 引数:
/// - `registry`: 検証対象の custom parameter registry。
///
/// 戻り値:
/// - `ValidatorResult`: 全 schema の検証成功可否と diagnostics。
///
/// 注意点:
/// - registry が空の場合は成功扱いとする。
/// - 同一キー重複の検出は後続実装とする。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m2
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m3
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m4
pub fn validate_custom_parameter_registry(
    registry: &CustomParameterRegistry,
) -> ValidatorResult {
    let mut diagnostics = Vec::new();

    for schema in registry.schemas() {
        let result = validate_custom_parameter_schema(schema);
        diagnostics.extend(result.diagnostics);
    }

    ValidatorResult {
        success: diagnostics
            .iter()
            .all(|diagnostic| diagnostic.severity != DiagnosticSeverity::Error),
        diagnostics,
    }
}

/// registry から schema を解決できることを検証する。
///
/// 役割:
/// - namespace / name / target_type に対応する schema が registry に登録済みか確認する。
/// - 未登録 parameter を diagnostics として返す。
///
/// 引数:
/// - `registry`: 検索対象 registry。
/// - `namespace`: 検索対象 namespace。
/// - `name`: 検索対象 parameter 名。
/// - `target_type`: 検索対象 target_type。
///
/// 戻り値:
/// - `ValidatorResult`: schema が存在する場合は成功、存在しない場合は diagnostics 付き失敗。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m2
pub fn validate_custom_parameter_registered(
    registry: &CustomParameterRegistry,
    namespace: &str,
    name: &str,
    target_type: &str,
) -> ValidatorResult {
    let target = format!("{}.{}:{}", namespace, name, target_type);

    if registry
        .resolve_custom_parameter_schema(namespace, name, target_type)
        .is_some()
    {
        return ValidatorResult {
            success: true,
            diagnostics: Vec::new(),
        };
    }

    ValidatorResult {
        success: false,
        diagnostics: vec![ValidationDiagnostic {
            code: DiagnosticCode::CustomParameterUnsupported,
            severity: DiagnosticSeverity::Error,
            message: format!(
                "custom parameter schema is not registered: {}",
                target
            ),
            target: Some(target),
        }],
    }
}

/// io_scope に応じた必須 mapping を検証する。
///
/// 役割:
/// - `io_scope = Mjcf` / `Both` の場合に `mjcf_mapping` が存在することを検証する。
/// - `io_scope = AdapterArtifact` / `Both` の場合に `adapter_artifact` が存在することを検証する。
///
/// 引数:
/// - `schema`: 検証対象 schema。
/// - `diagnostics`: 追記先 diagnostics。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m3
fn validate_required_mappings(
    schema: &CustomParameterSchema,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    if schema.io_scope.requires_mjcf_mapping() && schema.mjcf_mapping.is_none() {
        diagnostics.push(ValidationDiagnostic {
            code: DiagnosticCode::CustomParameterMappingInvalid,
            severity: DiagnosticSeverity::Error,
            message: format!(
                "custom parameter {}.{} requires mjcf_mapping",
                schema.namespace, schema.name
            ),
            target: Some(format!("{}.{}", schema.namespace, schema.name)),
        });
    }

    if schema.io_scope.requires_adapter_artifact() && schema.adapter_artifact.is_none() {
        diagnostics.push(ValidationDiagnostic {
            code: DiagnosticCode::CustomParameterMappingInvalid,
            severity: DiagnosticSeverity::Error,
            message: format!(
                "custom parameter {}.{} requires adapter_artifact",
                schema.namespace, schema.name
            ),
            target: Some(format!("{}.{}", schema.namespace, schema.name)),
        });
    }
}

/// 出力対象外 scope の diagnostics を生成する。
///
/// 役割:
/// - `Unsupported` を warning として記録する。
/// - `PreserveOnly` / `SourceRaw` は成功扱いの info として記録する。
///
/// 引数:
/// - `schema`: 検証対象 schema。
/// - `diagnostics`: 追記先 diagnostics。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m4
fn validate_non_output_scopes(
    schema: &CustomParameterSchema,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    match schema.io_scope {
        CustomParameterIoScope::Unsupported => diagnostics.push(ValidationDiagnostic {
            code: DiagnosticCode::CustomParameterUnsupported,
            severity: DiagnosticSeverity::Warning,
            message: format!(
                "custom parameter {}.{} is registered but unsupported",
                schema.namespace, schema.name
            ),
            target: Some(format!("{}.{}", schema.namespace, schema.name)),
        }),
        CustomParameterIoScope::PreserveOnly | CustomParameterIoScope::SourceRaw => {
            diagnostics.push(ValidationDiagnostic {
                code: DiagnosticCode::CustomParameterPreserved,
                severity: DiagnosticSeverity::Info,
                message: format!(
                    "custom parameter {}.{} is preserved without direct output",
                    schema.namespace, schema.name
                ),
                target: Some(format!("{}.{}", schema.namespace, schema.name)),
            });
        }
        _ => {}
    }
}
