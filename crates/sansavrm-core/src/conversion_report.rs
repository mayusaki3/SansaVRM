// crates/sansavrm-core/src/conversion_report.rs

use crate::DiagnosticItem;
use serde::{Deserialize, Serialize};

/// 変換方向。
///
/// 役割:
/// - Import / Export / RoundTrip など、変換処理の方向を表現する。
///
/// 注意点:
/// - 外部フォーマット固有の詳細方向は `source_format` / `target_format` で表現する。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_v8u6x1l1
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p4
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConversionDirection {
    Import,
    Export,
    RoundTrip,
    Custom,
}

/// 変換時に生成された成果物のメタデータ。
///
/// 役割:
/// - MJCF、controller_config、diagnostics、conversion_report などの補助成果物を記録する。
///
/// 注意点:
/// - 実ファイル内容ではなく、成果物の種類・パス・説明を保持する。
/// - Adapter 固有の詳細仕様は Adapter 側仕様で定義する。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p4
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedArtifactMetadata {
    pub artifact_type: String,
    pub path: Option<String>,
    pub description: Option<String>,
}

/// 非可逆変換情報。
///
/// 役割:
/// - 変換時に完全保持できなかった情報、fallback、保持のみの情報を記録する。
///
/// 注意点:
/// - 変換処理そのものを失敗扱いにするかどうかは diagnostics や呼び出し側の policy が決定する。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p4
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonReversibleConversionInfo {
    pub source_path: Option<String>,
    pub reason: String,
    pub fallback: Option<String>,
}

/// 変換レポート。
///
/// 役割:
/// - 変換処理の入力形式、出力形式、diagnostics、非可逆変換情報、生成成果物メタデータをまとめて保持する。
/// - Adapter 側補助成果物への分離結果を記録する。
///
/// 注意点:
/// - 本構造体は SansaVRM Core 側の共通保持形式であり、Adapter 固有の詳細レポート形式ではない。
/// - MJCF 生成アルゴリズムや controller_config の詳細は保持しない。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_v8u6x1l1
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p4
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversionReport {
    pub direction: ConversionDirection,
    pub source_format: String,
    pub target_format: String,
    pub diagnostics: Vec<DiagnosticItem>,
    pub non_reversible: Vec<NonReversibleConversionInfo>,
    pub generated_artifacts: Vec<GeneratedArtifactMetadata>,
}

impl ConversionReport {
    /// 新しい変換レポートを生成する。
    ///
    /// 役割:
    /// - 変換方向、入力形式、出力形式を指定して空のレポートを作成する。
    ///
    /// 引数:
    /// - `direction`: 変換方向。
    /// - `source_format`: 入力形式名。
    /// - `target_format`: 出力形式名。
    ///
    /// 戻り値:
    /// - `ConversionReport`: 空の diagnostics / non_reversible / generated_artifacts を持つレポート。
    pub fn new(
        direction: ConversionDirection,
        source_format: impl Into<String>,
        target_format: impl Into<String>,
    ) -> Self {
        Self {
            direction,
            source_format: source_format.into(),
            target_format: target_format.into(),
            diagnostics: Vec::new(),
            non_reversible: Vec::new(),
            generated_artifacts: Vec::new(),
        }
    }

    /// diagnostics を追加する。
    ///
    /// 役割:
    /// - 変換中に発生した診断情報をレポートへ追加する。
    ///
    /// 引数:
    /// - `diagnostic`: 追加する診断情報。
    pub fn add_diagnostic(&mut self, diagnostic: DiagnosticItem) {
        self.diagnostics.push(diagnostic);
    }

    /// 非可逆変換情報を追加する。
    ///
    /// 役割:
    /// - 完全保持できなかった情報や fallback 結果をレポートへ追加する。
    ///
    /// 引数:
    /// - `info`: 追加する非可逆変換情報。
    pub fn add_non_reversible(&mut self, info: NonReversibleConversionInfo) {
        self.non_reversible.push(info);
    }

    /// 生成成果物メタデータを追加する。
    ///
    /// 役割:
    /// - MJCF や Adapter 側補助成果物などのメタデータをレポートへ追加する。
    ///
    /// 引数:
    /// - `artifact`: 追加する成果物メタデータ。
    pub fn add_generated_artifact(&mut self, artifact: GeneratedArtifactMetadata) {
        self.generated_artifacts.push(artifact);
    }
}
