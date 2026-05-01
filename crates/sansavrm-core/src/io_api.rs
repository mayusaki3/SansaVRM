// crates/sansavrm-core/src/io_api.rs

use serde::{Deserialize, Serialize};

use crate::{CoreResult, Model, SansaVrmError};

/// glTF 文書型。
///
/// 注意:
/// - 初期実装では文字列として保持する。
/// - Adapter 実装時に専用型へ置換する可能性がある。
pub type GltfDocument = String;

/// VRM 文書型。
pub type VrmDocument = String;

/// URDF 文書型。
pub type UrdfDocument = String;

/// MuJoCo / MJCF 文書型。
pub type MjcfDocument = String;

/// VRM export 対象バージョン。
///
/// TODO(trace): CoreAPI仕様 / export_vrm
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VrmVersion {
    V0x,
    V1_0,
}

/// I/O API オプション。
///
/// 注意:
/// - 初期実装では空構造体とする。
/// - preserve_raw / strict / lossy_allowed 等は Adapter 実装時に拡張する。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IoOptions {}

/// glTF を import する。
///
/// TODO(trace): CoreAPI仕様 / import_gltf
pub fn import_gltf(_document: GltfDocument) -> CoreResult<Model> {
    not_implemented("import_gltf")
}

/// glTF を export する。
///
/// TODO(trace): CoreAPI仕様 / export_gltf
pub fn export_gltf(_model: &Model) -> CoreResult<GltfDocument> {
    not_implemented("export_gltf")
}

/// VRM を import する。
///
/// TODO(trace): CoreAPI仕様 / import_vrm
pub fn import_vrm(_document: VrmDocument) -> CoreResult<Model> {
    not_implemented("import_vrm")
}

/// VRM を export する。
///
/// 既定では VRM 1.0 を指定して呼び出す想定。
///
/// TODO(trace): CoreAPI仕様 / export_vrm
pub fn export_vrm(
    _model: &Model,
    _version: VrmVersion,
    _options: IoOptions,
) -> CoreResult<VrmDocument> {
    not_implemented("export_vrm")
}

/// URDF を import する。
///
/// TODO(trace): CoreAPI仕様 / import_urdf
pub fn import_urdf(_document: UrdfDocument) -> CoreResult<Model> {
    not_implemented("import_urdf")
}

/// URDF を export する。
///
/// TODO(trace): CoreAPI仕様 / export_urdf
pub fn export_urdf(_model: &Model) -> CoreResult<UrdfDocument> {
    not_implemented("export_urdf")
}

/// MuJoCo / MJCF を import する。
///
/// TODO(trace): CoreAPI仕様 / import_mujoco
pub fn import_mujoco(_document: MjcfDocument) -> CoreResult<Model> {
    not_implemented("import_mujoco")
}

/// MuJoCo / MJCF を export する。
///
/// TODO(trace): CoreAPI仕様 / export_mujoco
pub fn export_mujoco(_model: &Model) -> CoreResult<MjcfDocument> {
    not_implemented("export_mujoco")
}

/// 未実装API用の共通エラーを返す。
fn not_implemented<T>(api_name: &str) -> CoreResult<T> {
    CoreResult::fail(SansaVrmError::InvalidInput(format!(
        "{} is not implemented yet",
        api_name
    )))
}
