//! SansaVRM URDF adapter.
//!
//! 役割:
//! - URDF 文書を SansaVRM Model へ import する。
//! - SansaVRM Model を URDF 文書へ export する。
//! - SansaVRM Property を URDF 出力対象へ分類する。
//!
//! 注意点:
//! - 現段階では URDF link の import/export と Property の初期分類を扱う。
//! - visual / collision / inertial の詳細 XML 生成は後続実装で扱う。
//! - property_type を主判定とし、context は補助判定として扱う。
//!
//! @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n2
//! @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n3
//! @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n4
//! @hldocs.ref doc-20260504-000203Z-SV0D#sec_c6t5v8s3

use quick_xml::de::from_str;
use serde::Deserialize;
use sansavrm_core::{
    CoreResult, Model, Module, ModuleType, Property, PropertyContext, PropertyType,
    SansaVrmError, UrdfDocument,
};

/// URDF export 時に Property をどの URDF 要素へ反映するかを表す。
///
/// 役割:
/// - SansaVRM Property の property_type / context を URDF の出力対象へ分類する。
///
/// 注意点:
/// - この分類は export 前段の判定であり、実際の URDF XML 生成は別処理で行う。
/// - property_type を優先し、context は補助判定として扱う。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n2
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n3
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n4
/// @hldocs.ref doc-20260504-000203Z-SV0D#sec_c6t5v8s3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrdfPropertyTarget {
    Inertial,
    Visual,
    Collision,
    Joint,
    Ignore,
}

/// URDF robot。
///
/// 役割:
/// - URDF `<robot>` 要素の最小読み込み構造を表す。
#[derive(Debug, Deserialize)]
struct UrdfRobot {
    #[serde(rename = "@name")]
    name: Option<String>,

    #[serde(rename = "link", default)]
    links: Vec<UrdfLink>,
}

/// URDF link。
///
/// 役割:
/// - URDF `<link>` 要素の最小読み込み構造を表す。
#[derive(Debug, Deserialize)]
struct UrdfLink {
    #[serde(rename = "@name")]
    name: String,
}

/// Property を URDF 出力対象へ分類する。
///
/// 役割:
/// - Physics 系 Property を inertial / collision 生成候補に分類する。
/// - Geometry / Material / Texture 系 Property を visual / collision 生成候補に分類する。
/// - Rig / Constraint / Actuator / Sensor 系 Property は現段階では URDF XML へ直接出力しない。
///
/// 引数:
/// - `property`: 分類対象の Property。
///
/// 戻り値:
/// - `UrdfPropertyTarget`: URDF 出力対象分類。
///
/// 注意点:
/// - URDF では visual / collision の詳細構造が別途必要になるため、
///   本関数は初期分類のみを行う。
/// - `Physics + Simulation` は inertial として扱う。
/// - `Geometry / Material / Texture + Rendering` は visual として扱う。
/// - `Geometry + Simulation` は collision として扱う。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n2
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n3
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n4
/// @hldocs.ref doc-20260504-000203Z-SV0D#sec_c6t5v8s3
pub fn classify_urdf_property(property: &Property) -> UrdfPropertyTarget {
    match property.property_type {
        PropertyType::Physics => match property.context {
            PropertyContext::Simulation | PropertyContext::Execution => {
                UrdfPropertyTarget::Inertial
            }
            _ => UrdfPropertyTarget::Ignore,
        },

        PropertyType::Geometry => match property.context {
            PropertyContext::Rendering => UrdfPropertyTarget::Visual,
            PropertyContext::Simulation => UrdfPropertyTarget::Collision,
            _ => UrdfPropertyTarget::Ignore,
        },

        PropertyType::Material | PropertyType::Texture => match property.context {
            PropertyContext::Rendering => UrdfPropertyTarget::Visual,
            _ => UrdfPropertyTarget::Ignore,
        },

        _ => UrdfPropertyTarget::Ignore,
    }
}

/// URDF を SansaVRM Model へ import する。
///
/// 役割:
/// - URDF XML を解析し、SansaVRM Model を生成する。
/// - URDF link を SansaVRM Module として登録する。
///
/// 引数:
/// - `document`: import 対象の URDF XML 文書。
///
/// 戻り値:
/// - `CoreResult<Model>`: 成功時は生成した Model、失敗時はエラー情報。
///
/// 注意点:
/// - 現段階では link のみを最小対応とする。
/// - joint / visual / collision / inertial の詳細 import は後続実装で扱う。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n2
pub fn import_urdf(document: UrdfDocument) -> CoreResult<Model> {
    let robot = match from_str::<UrdfRobot>(&document) {
        Ok(robot) => robot,
        Err(error) => {
            return CoreResult::fail(SansaVrmError::InvalidInput(format!(
                "Failed to parse URDF XML: {}",
                error
            )));
        }
    };

    let mut model = if let Some(name) = robot.name {
        Model::with_id(name)
    } else {
        Model::new()
    };

    for link in robot.links {
        model.modules.push(Module {
            module_id: link.name,
            module_type: ModuleType::Module,
            slots: vec![],
            properties: vec![],
        });
    }

    CoreResult::ok(model)
}

/// SansaVRM Model を URDF へ export する。
///
/// 役割:
/// - SansaVRM Model から最小 URDF XML 文書を生成する。
/// - SansaVRM Module を URDF link として出力する。
///
/// 引数:
/// - `model`: export 対象の SansaVRM Model。
///
/// 戻り値:
/// - `CoreResult<UrdfDocument>`: 成功時は URDF XML 文書、失敗時はエラー情報。
///
/// 注意点:
/// - 現段階では link のみを最小出力する。
/// - joint / visual / collision / inertial の詳細 export は後続実装で扱う。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n2
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n3
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n4
pub fn export_urdf(model: &Model) -> CoreResult<UrdfDocument> {
    let mut document = format!(r#"<robot name="{}">"#, model.model_id);

    for module in &model.modules {
        document.push_str(&format!(
            r#"
    <link name="{}"/>"#,
            module.module_id
        ));
    }

    document.push_str("\n</robot>");

    CoreResult::ok(document)
}
