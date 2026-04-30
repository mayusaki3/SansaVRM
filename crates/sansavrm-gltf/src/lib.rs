//! SansaVRM glTF adapter.

use sansavrm_core::{CoreResult, GltfDocument, Model, Module, ModuleType, SansaVrmError};
use serde::{Deserialize, Serialize};

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

/// glTF export 用 asset。
#[derive(Debug, Serialize)]
struct GltfAssetOut {
    version: String,
}

/// glTF export 用 node。
#[derive(Debug, Serialize)]
struct GltfNodeOut {
    name: String,
}

/// glTF export 用 root。
#[derive(Debug, Serialize)]
struct GltfRootOut {
    asset: GltfAssetOut,
    nodes: Vec<GltfNodeOut>,
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
pub fn export_gltf(model: &Model) -> CoreResult<GltfDocument> {
    let gltf = GltfRootOut {
        asset: GltfAssetOut {
            version: "2.0".into(),
        },
        nodes: model
            .modules
            .iter()
            .map(|module| GltfNodeOut {
                name: module.module_id.clone(),
            })
            .collect(),
    };

    match serde_json::to_string_pretty(&gltf) {
        Ok(document) => CoreResult::ok(document),
        Err(error) => CoreResult::fail(SansaVrmError::InvalidInput(format!(
            "Failed to export glTF JSON: {}",
            error
        ))),
    }
}
