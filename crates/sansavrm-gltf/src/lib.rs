//! SansaVRM glTF adapter.

use sansavrm_core::{CoreResult, GltfDocument, Model, Module, ModuleType, SansaVrmError};
use serde::Deserialize;

/// glTF asset。
#[derive(Debug, Deserialize)]
struct GltfAsset {
    version: String,
}

/// glTF node。
#[derive(Debug, Deserialize)]
struct GltfNode {
    name: Option<String>,
}

/// glTF root。
#[derive(Debug, Deserialize)]
struct GltfRoot {
    asset: GltfAsset,
    #[serde(default)]
    nodes: Vec<GltfNode>,
}

/// glTF を SansaVRM Model へ import する。
///
/// TODO(trace): 変換仕様 / glTF Import
pub fn import_gltf(document: GltfDocument) -> CoreResult<Model> {
    let gltf = match serde_json::from_str::<GltfRoot>(&document) {
        Ok(gltf) => gltf,
        Err(error) => {
            return CoreResult::fail(SansaVrmError::InvalidInput(format!(
                "Failed to parse glTF JSON: {}",
                error
            )));
        }
    };

    if gltf.asset.version.trim().is_empty() {
        return CoreResult::fail(SansaVrmError::InvalidInput(
            "glTF asset.version is empty".into(),
        ));
    }

    let mut model = Model::new();

    for (index, node) in gltf.nodes.iter().enumerate() {
        model.modules.push(Module {
            module_id: node
                .name
                .clone()
                .filter(|name| !name.trim().is_empty())
                .unwrap_or_else(|| format!("gltf_node_{}", index)),
            module_type: ModuleType::Module,
            slots: vec![],
            properties: vec![],
        });
    }

    CoreResult::ok(model)
}

/// SansaVRM Model を glTF へ export する。
///
/// TODO(trace): 変換仕様 / glTF Export
pub fn export_gltf(_model: &Model) -> CoreResult<GltfDocument> {
    CoreResult::fail(SansaVrmError::InvalidInput(
        "gltf export is not implemented yet".into(),
    ))
}
