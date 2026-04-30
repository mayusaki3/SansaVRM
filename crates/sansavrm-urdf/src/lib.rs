//! SansaVRM URDF adapter.

use quick_xml::de::from_str;
use serde::Deserialize;
use sansavrm_core::{CoreResult, Model, Module, ModuleType, SansaVrmError, UrdfDocument};

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

/// URDF を SansaVRM Model へ import する。
///
/// TODO(trace): 変換仕様 / URDF Import
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
/// TODO(trace): 変換仕様 / URDF Export
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
