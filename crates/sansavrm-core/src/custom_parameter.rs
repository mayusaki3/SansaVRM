// crates/sansavrm-core/src/custom_parameter.rs

//! custom parameter 登録・保持用の共通型定義。
//!
//! 役割:
//! - namespace 付き custom parameter の登録スキーマを表現する。
//! - MJCF 直接入出力、Adapter 側補助成果物、保持のみ、未対応、source_raw の分類を表現する。
//! - MuJoCo 固有または将来追加されるパラメータを、完全自由な key-value ではなく検証可能な構造として保持する。
//!
//! 注意点:
//! - 本モジュールは型定義と軽量な整合性判定のみを担当する。
//! - 実際の version 比較、値範囲検証、Adapter 固有解釈は Validator または Adapter 側で行う。
//! - MJCF への直接入出力可否は実装側の推測ではなく、`io_scope`、`mjcf_mapping`、`adapter_artifact` に基づいて判定する。

use crate::PropertyValue;
use serde::{Deserialize, Serialize};

/// custom parameter の入出力範囲。
///
/// 役割:
/// - custom parameter を MJCF、Adapter 側補助成果物、保持のみ、未対応、source_raw のどれとして扱うかを表現する。
///
/// 注意点:
/// - MJCF への直接入出力可否は実装側の推測ではなく、本 enum と mapping 定義に基づいて判定する。
/// - `Both` は MJCF と Adapter 側補助成果物の両方を対象とする。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m2
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m3
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomParameterIoScope {
    /// MJCF へ直接入出力する。
    Mjcf,
    /// Adapter 側補助成果物へ出力する。
    AdapterArtifact,
    /// MJCF と Adapter 側補助成果物の両方へ出力する。
    Both,
    /// SansaVRM 内に保持するが外部成果物へは出力しない。
    PreserveOnly,
    /// 登録はされているが現時点では未対応として扱う。
    Unsupported,
    /// 解釈せず元情報として保持する。
    SourceRaw,
}

impl CustomParameterIoScope {
    /// MJCF 直接入出力対象かを判定する。
    ///
    /// 戻り値:
    /// - `true`: `Mjcf` または `Both` の場合。
    /// - `false`: それ以外の場合。
    pub fn requires_mjcf_mapping(&self) -> bool {
        matches!(self, Self::Mjcf | Self::Both)
    }

    /// Adapter 側補助成果物対象かを判定する。
    ///
    /// 戻り値:
    /// - `true`: `AdapterArtifact` または `Both` の場合。
    /// - `false`: それ以外の場合。
    pub fn requires_adapter_artifact(&self) -> bool {
        matches!(self, Self::AdapterArtifact | Self::Both)
    }
}

/// custom parameter の値型。
///
/// 役割:
/// - 登録スキーマで許可する値型を表現する。
///
/// 注意点:
/// - 実際の値は `PropertyValue` を利用する。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m2
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomParameterValueType {
    /// 文字列値。
    String,
    /// 数値。
    Number,
    /// 真偽値。
    Bool,
}

/// fallback 方針。
///
/// 役割:
/// - Adapter が custom parameter を解釈できない場合や対象 version で未対応の場合の処理方針を表現する。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m4
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomParameterFallbackBehavior {
    /// 登録スキーマの既定値を使用する。
    UseDefault,
    /// 出力せず SansaVRM 内に保持する。
    PreserveOnly,
    /// 警告として diagnostics または conversion report に記録する。
    Warn,
    /// エラーとして扱う。
    Error,
    /// 無視する。
    Ignore,
}

/// semantic version 範囲。
///
/// 役割:
/// - MuJoCo version や Adapter version の対応範囲を保持する。
///
/// 注意点:
/// - version 比較の実処理は Validator または Adapter 側で行う。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m4
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionRange {
    /// 対応する最小 version。未指定の場合は下限なし。
    pub min: Option<String>,
    /// 対応する最大 version。未指定の場合は上限なし。
    pub max: Option<String>,
}

/// MJCF mapping 定義。
///
/// 役割:
/// - MJCF へ直接入出力できる custom parameter の element / attribute / path / direction を保持する。
///
/// 注意点:
/// - `io_scope = Mjcf` または `io_scope = Both` の場合に原則必須とする。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m3
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MjcfMapping {
    /// 対応する MJCF 要素名。
    pub element: String,
    /// 対応する MJCF 属性名。
    pub attribute: String,
    /// MJCF 内の論理パス。
    pub path: String,
    /// 入出力方向。例: `import`、`export`、`import_export`。
    pub direction: String,
    /// 値変換ルール。未指定の場合は変換なし。
    pub value_conversion: Option<String>,
    /// 必要な MuJoCo version 範囲。
    pub required_mujoco_version: VersionRange,
}

/// Adapter 側補助成果物 mapping 定義。
///
/// 役割:
/// - MJCF に直接入出力しない custom parameter の補助成果物出力先を保持する。
///
/// 注意点:
/// - `io_scope = AdapterArtifact` または `io_scope = Both` の場合に原則必須とする。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m3
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_y5x3a8p4
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterArtifactMapping {
    /// 補助成果物の種別。例: `controller_config`。
    pub artifact_type: String,
    /// 補助成果物内の論理パス。
    pub path: String,
    /// 入出力方向。例: `import`、`export`、`import_export`。
    pub direction: String,
    /// 値変換ルール。未指定の場合は変換なし。
    pub value_conversion: Option<String>,
    /// 必要な Adapter version 範囲。
    pub required_adapter_version: VersionRange,
}

/// fallback 定義。
///
/// 役割:
/// - fallback の動作と必要に応じた代替値を保持する。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m4
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomParameterFallback {
    /// fallback の動作種別。
    pub behavior: CustomParameterFallbackBehavior,
    /// fallback 時に使用する代替値。不要な場合は `None`。
    pub value: Option<PropertyValue>,
}

/// custom parameter 登録スキーマ。
///
/// 役割:
/// - namespace 付き custom parameter の型、対象、制約、入出力範囲、mapping、fallback、version 情報を保持する。
///
/// 注意点:
/// - 完全自由な key-value ではなく、本スキーマに登録された parameter のみを検証対象とする。
/// - MJCF 直接入出力可否は `io_scope` と `mjcf_mapping` により判定する。
/// - Adapter 側補助成果物への分離先は `adapter_artifact` により判定する。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m2
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m3
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m4
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomParameterSchema {
    /// パラメータの namespace。
    pub namespace: String,
    /// namespace 内のパラメータ名。
    pub name: String,
    /// 適用対象種別。例: `model`、`joint`、`actuator`。
    pub target_type: String,
    /// 値型。
    pub value_type: CustomParameterValueType,
    /// 単位。不要な場合は `None`。
    pub unit: Option<String>,
    /// 必須パラメータかどうか。
    pub required: bool,
    /// 既定値。未指定の場合は `None`。
    pub default: Option<PropertyValue>,
    /// 数値最小値。不要な場合は `None`。
    pub min: Option<f64>,
    /// 数値最大値。不要な場合は `None`。
    pub max: Option<f64>,
    /// 許可値一覧。不要な場合は `None`。
    pub allowed_values: Option<Vec<PropertyValue>>,
    /// 人間向け説明。
    pub description: Option<String>,
    /// 対応 Adapter 名または対応状態の一覧。
    pub adapter_support: Vec<String>,
    /// fallback 方針。
    pub fallback: CustomParameterFallback,
    /// 入出力範囲。
    pub io_scope: CustomParameterIoScope,
    /// MJCF mapping。不要または未対応の場合は `None`。
    pub mjcf_mapping: Option<MjcfMapping>,
    /// Adapter 側補助成果物 mapping。不要または未対応の場合は `None`。
    pub adapter_artifact: Option<AdapterArtifactMapping>,
    /// 対象 MuJoCo version 範囲。
    pub mujoco_version: Option<VersionRange>,
    /// 対応開始 version。
    pub supported_since: Option<String>,
    /// 非推奨化 version。
    pub deprecated_since: Option<String>,
}

impl CustomParameterSchema {
    /// mapping 必須条件を満たすかを検証する。
    ///
    /// 役割:
    /// - `io_scope` に応じて `mjcf_mapping` と `adapter_artifact` の有無を確認する。
    ///
    /// 戻り値:
    /// - `true`: mapping 必須条件を満たす場合。
    /// - `false`: 必須 mapping が欠落している場合。
    pub fn has_required_mappings(&self) -> bool {
        if self.io_scope.requires_mjcf_mapping() && self.mjcf_mapping.is_none() {
            return false;
        }

        if self.io_scope.requires_adapter_artifact() && self.adapter_artifact.is_none() {
            return false;
        }

        true
    }
}

/// custom parameter 値。
///
/// 役割:
/// - 登録スキーマに従って、特定対象へ付与された custom parameter の値を保持する。
///
/// 注意点:
/// - 値の妥当性検証は登録スキーマと組み合わせて Validator 側で実施する。
///
/// @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m2
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomParameterValue {
    /// パラメータの namespace。
    pub namespace: String,
    /// namespace 内のパラメータ名。
    pub name: String,
    /// 適用対象種別。
    pub target_type: String,
    /// 適用対象 ID。
    pub target_id: String,
    /// パラメータ値。
    pub value: PropertyValue,
    /// 値の由来。不要な場合は `None`。
    pub source: Option<String>,
    /// 値に関連する diagnostics。
    pub diagnostics: Vec<String>,
}
