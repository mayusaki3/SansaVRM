use sansavrm_core::{Model, SansaVrmError};

const VRM_HUMANOID_BONE_PREFIX: &str = "vrm.humanoid.human_bones.";
const VRM_HUMANOID_BONE_NODE_SUFFIX: &str = ".node";

/// SansaVRM: VRM validator が要求する humanoid bones。
///
/// 注意:
/// - sansavrm-vrm/src/common.rs にも VRM_HUMANOID_BONES が存在する。
/// - 現段階では crate 間依存を増やさないため重複を許容する。
/// - 将来、VRM仕様定義専用 crate を作成する場合は共通化する。
///
/// TODO(trace): Validator実装仕様 / VRM Humanoid Required Bones
const VRM_REQUIRED_HUMANOID_BONES: &[&str] = &[
    "hips",
    "spine",
    "chest",
    "neck",
    "head",
    "leftUpperLeg",
    "leftLowerLeg",
    "leftFoot",
    "rightUpperLeg",
    "rightLowerLeg",
    "rightFoot",
    "leftUpperArm",
    "leftLowerArm",
    "leftHand",
    "rightUpperArm",
    "rightLowerArm",
    "rightHand",
];

fn is_vrm_humanoid_bone_node_key(key: &str) -> bool {
    key.starts_with(VRM_HUMANOID_BONE_PREFIX)
        && key.ends_with(VRM_HUMANOID_BONE_NODE_SUFFIX)
}

fn vrm_humanoid_bone_node_key(bone_name: &str) -> String {
    format!(
        "{}{}{}",
        VRM_HUMANOID_BONE_PREFIX, bone_name, VRM_HUMANOID_BONE_NODE_SUFFIX
    )
}

/// VRM humanoid 最小検証
/// TODO(trace): Validator実装仕様 / VRM Humanoid Validation
pub(crate) fn validate_vrm_humanoid(model: &Model, errors: &mut Vec<SansaVrmError>) {
    let humanoid_properties = model
        .properties
        .iter()
        .filter(|property| is_vrm_humanoid_bone_node_key(&property.key))
        .collect::<Vec<_>>();

    if humanoid_properties.is_empty() {
        return;
    }

    for bone_name in VRM_REQUIRED_HUMANOID_BONES {
        let key = vrm_humanoid_bone_node_key(bone_name);

        if !humanoid_properties.iter().any(|property| property.key == key) {
            errors.push(SansaVrmError::InvalidInput(format!(
                "VRM humanoid {} bone is missing",
                bone_name
            )));
        }
    }

    for property in humanoid_properties {
        let Some(module_id) = property.value.as_string() else {
            errors.push(SansaVrmError::InvalidInput(format!(
                "VRM humanoid bone {} must reference module by string value",
                property.key
            )));
            continue;
        };

        let exists = model
            .modules
            .iter()
            .any(|module| module.module_id == module_id);

        if !exists {
            errors.push(SansaVrmError::InvalidInput(format!(
                "VRM humanoid bone {} references unknown module {}",
                property.key,
                module_id
            )));
        }
    }
}
