//! SansaVRM URDF adapter.
//!
//! @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n2
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
#[derive(Debug, Deserialize)]
struct UrdfRobot {
    #[serde(rename = "@name")]
    name: Option<String>,

    #[serde(rename = "link", default)]
    links: Vec<UrdfLink>,
}

/// URDF link。
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
/// - `property`: 分類対象の Property
///
/// 戻り値:
/// - `UrdfPropertyTarget`: URDF 出力対象分類
///
/// 注意点:
/// - URDF では visual / collision の詳細構造が別途必要になるため、
///   本関数は初期分類のみを行う。
/// - `Physics + Simulation` は inertial として扱う。
/// - `Geometry / Material / Texture + Rendering` は visual として扱う。
/// - `Geometry + Simulation` は collision として扱う。
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n2
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
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_x6w4z9n2
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
