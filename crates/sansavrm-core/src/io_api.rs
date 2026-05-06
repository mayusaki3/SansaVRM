// crates/sansavrm-core/src/io_api.rs

use serde::{Deserialize, Serialize};

use crate::{CoreResult, Model, SansaVrmError};

/// glTF 文書型。
///
/// 注意:
/// - 初期実装では文字列として保持する。
/// - Adapter 実装時に専用型へ置換する可能性がある。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3f5
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3f6
pub type GltfDocument = String;

/// VRM 文書型。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3f7
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_r2q8t5g6
pub type VrmDocument = String;

/// URDF 文書型。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3f9
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3g0
pub type UrdfDocument = String;

/// MuJoCo / MJCF 文書型。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3g1
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3g2
pub type MjcfDocument = String;

/// VRM export 対象バージョン。
///
/// 役割:
/// - export_vrm の出力対象VRMバージョンを表現する。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_r2q8t5g6
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
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_r2q8t5g6
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IoOptions {}

/// glTF を import する。
///
/// 役割:
/// - glTF文書からModelを生成する。
///
/// 注意点:
/// - Core層では初期実装として未実装エラーを返す。
/// - 実変換はAdapter層で実装する。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3f5
pub fn import_gltf(_document: GltfDocument) -> CoreResult<Model> {
    not_implemented("import_gltf")
}

/// glTF を export する。
///
/// 役割:
/// - ModelからglTF文書を生成する。
///
/// 注意点:
/// - Core層では初期実装として未実装エラーを返す。
/// - 実変換はAdapter層で実装する。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3f6
pub fn export_gltf(_model: &Model) -> CoreResult<GltfDocument> {
    not_implemented("export_gltf")
}

/// VRM を import する。
///
/// 役割:
/// - VRM文書からModelを生成する。
///
/// 注意点:
/// - Core層では初期実装として未実装エラーを返す。
/// - 実変換はAdapter層で実装する。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3f7
pub fn import_vrm(_document: VrmDocument) -> CoreResult<Model> {
    not_implemented("import_vrm")
}

/// VRM を export する。
///
/// 役割:
/// - Modelから指定バージョンのVRM文書を生成する。
///
/// 注意点:
/// - Core層では初期実装として未実装エラーを返す。
/// - 実変換はAdapter層で実装する。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_r2q8t5g6
pub fn export_vrm(
    _model: &Model,
    _version: VrmVersion,
    _options: IoOptions,
) -> CoreResult<VrmDocument> {
    not_implemented("export_vrm")
}

/// URDF を import する。
///
/// 役割:
/// - URDF文書からModelを生成する。
///
/// 注意点:
/// - Core層では初期実装として未実装エラーを返す。
/// - 実変換はAdapter層で実装する。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3f9
pub fn import_urdf(_document: UrdfDocument) -> CoreResult<Model> {
    not_implemented("import_urdf")
}

/// URDF を export する。
///
/// 役割:
/// - ModelからURDF文書を生成する。
///
/// 注意点:
/// - Core層では初期実装として未実装エラーを返す。
/// - 実変換はAdapter層で実装する。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3g0
pub fn export_urdf(_model: &Model) -> CoreResult<UrdfDocument> {
    not_implemented("export_urdf")
}

/// MuJoCo / MJCF を import する。
///
/// 役割:
/// - MuJoCo / MJCF文書からModelを生成する。
///
/// 注意点:
/// - Core層では初期実装として未実装エラーを返す。
/// - 実変換はAdapter層で実装する。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3g1
pub fn import_mujoco(_document: MjcfDocument) -> CoreResult<Model> {
    not_implemented("import_mujoco")
}

/// MuJoCo / MJCF を export する。
///
/// 役割:
/// - ModelからMuJoCo / MJCF文書を生成する。
///
/// 注意点:
/// - Core層では初期実装として未実装エラーを返す。
/// - 実変換はAdapter層で実装する。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3g2
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
