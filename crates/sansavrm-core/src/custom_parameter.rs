// crates/sansavrm-core/src/custom_parameter.rs

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
    Mjcf,
    AdapterArtifact,
    Both,
    PreserveOnly,
    Unsupported,
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
    String,
    Number,
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
    UseDefault,
    PreserveOnly,
    Warn,
    Error,
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
    pub min: Option<String>,
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
    pub element: String,
    pub attribute: String,
    pub path: String,
    pub direction: String,
    pub value_conversion: Option<String>,
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
    pub artifact_type: String,
    pub path: String,
    pub direction: String,
    pub value_conversion: Option<String>,
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
    pub behavior: CustomParameterFallbackBehavior,
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
    pub namespace: String,
    pub name: String,
    pub target_type: String,
    pub value_type: CustomParameterValueType,
    pub unit: Option<String>,
    pub required: bool,
    pub default: Option<PropertyValue>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub allowed_values: Option<Vec<PropertyValue>>,
    pub description: Option<String>,
    pub adapter_support: Vec<String>,
    pub fallback: CustomParameterFallback,
    pub io_scope: CustomParameterIoScope,
    pub mjcf_mapping: Option<MjcfMapping>,
    pub adapter_artifact: Option<AdapterArtifactMapping>,
    pub mujoco_version: Option<VersionRange>,
    pub supported_since: Option<String>,
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
    pub namespace: String,
    pub name: String,
    pub target_type: String,
    pub target_id: String,
    pub value: PropertyValue,
    pub source: Option<String>,
    pub diagnostics: Vec<String>,
}
