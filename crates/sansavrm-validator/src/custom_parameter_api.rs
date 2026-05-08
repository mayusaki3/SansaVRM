// crates/sansavrm-validator/src/custom_parameter_api.rs

use sansavrm_core::{
    CustomParameterIoScope, CustomParameterSchema, DiagnosticCode,
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
