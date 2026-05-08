// crates/sansavrm-core/tests/conversion_report.rs

use sansavrm_core::{
    ConversionDirection, ConversionReport, DiagnosticItem, DiagnosticSeverity,
    DiagnosticType, GeneratedArtifactMetadata, NonReversibleConversionInfo,
};

/// conversion report を生成できることを確認する。
///
/// テスト内容:
/// - direction / source_format / target_format が保持されること。
/// - diagnostics / non_reversible / generated_artifacts が初期空状態であること。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p4
#[test]
fn core_conversion_report_001_create_report_should_initialize_empty_lists() {
    let report = ConversionReport::new(
        ConversionDirection::Export,
        "sansavrm",
        "mjcf",
    );

    assert_eq!(report.direction, ConversionDirection::Export);
    assert_eq!(report.source_format, "sansavrm");
    assert_eq!(report.target_format, "mjcf");

    assert!(report.diagnostics.is_empty());
    assert!(report.non_reversible.is_empty());
    assert!(report.generated_artifacts.is_empty());
}

/// diagnostics を追加できることを確認する。
///
/// テスト内容:
/// - add_diagnostic により diagnostics が追加されること。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p4
#[test]
fn core_conversion_report_002_add_diagnostic_should_append_item() {
    let mut report = ConversionReport::new(
        ConversionDirection::Export,
        "sansavrm",
        "mjcf",
    );

    report.add_diagnostic(DiagnosticItem {
        diagnostic_type: DiagnosticType::NonReversibleConversion,
        severity: DiagnosticSeverity::Warning,
        message: "fallback applied".into(),
        source: "adapter".into(),
    });

    assert_eq!(report.diagnostics.len(), 1);
    assert_eq!(report.diagnostics[0].message, "fallback applied");
}

/// 非可逆変換情報を追加できることを確認する。
///
/// テスト内容:
/// - add_non_reversible により non_reversible が追加されること。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p4
#[test]
fn core_conversion_report_003_add_non_reversible_should_append_item() {
    let mut report = ConversionReport::new(
        ConversionDirection::Export,
        "sansavrm",
        "mjcf",
    );

    report.add_non_reversible(NonReversibleConversionInfo {
        source_path: Some("joint.armature".into()),
        reason: "unsupported by target runtime".into(),
        fallback: Some("use_default".into()),
    });

    assert_eq!(report.non_reversible.len(), 1);
    assert_eq!(
        report.non_reversible[0].reason,
        "unsupported by target runtime"
    );
}

/// 生成成果物メタデータを追加できることを確認する。
///
/// テスト内容:
/// - add_generated_artifact により generated_artifacts が追加されること。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p4
#[test]
fn core_conversion_report_004_add_generated_artifact_should_append_item() {
    let mut report = ConversionReport::new(
        ConversionDirection::Export,
        "sansavrm",
        "mjcf",
    );

    report.add_generated_artifact(GeneratedArtifactMetadata {
        artifact_type: "controller_config".into(),
        path: Some("controller/config.json".into()),
        description: Some("adapter runtime config".into()),
    });

    assert_eq!(report.generated_artifacts.len(), 1);
    assert_eq!(
        report.generated_artifacts[0].artifact_type,
        "controller_config"
    );
}
