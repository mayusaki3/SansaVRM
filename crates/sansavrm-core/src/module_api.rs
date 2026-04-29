// crates/sansavrm-core/src/module_api.rs

use crate::{CoreResult, Model, Module, SansaVrmError};

/// Module を追加する。
///
/// 役割:
/// - Model に Module を追加する。
///
/// 注意点:
/// - module_id は Model 内で一意である必要がある。
///
/// TODO(trace): CoreAPI仕様 / add_module
pub fn add_module(mut model: Model, module: Module) -> CoreResult<Model> {
    if model
        .modules
        .iter()
        .any(|existing| existing.module_id == module.module_id)
    {
        return CoreResult::fail(SansaVrmError::DuplicateId(module.module_id));
    }

    model.modules.push(module);
    CoreResult::ok(model)
}

/// Module を更新する。
///
/// 役割:
/// - module_id に一致する Module を差し替える。
///
/// 注意点:
/// - patch は初期実装では部分更新ではなく全体差し替えとする。
/// - 差し替え後も module_id は維持する。
///
/// TODO(trace): CoreAPI仕様 / update_module
pub fn update_module(
    mut model: Model,
    module_id: impl AsRef<str>,
    mut patch: Module,
) -> CoreResult<Model> {
    let module_id = module_id.as_ref();
    patch.module_id = module_id.to_string();

    if let Some(module) = model
        .modules
        .iter_mut()
        .find(|module| module.module_id == module_id)
    {
        *module = patch;
        return CoreResult::ok(model);
    }

    CoreResult::fail(SansaVrmError::IdNotFound(module_id.to_string()))
}

/// Module を削除する。
///
/// 役割:
/// - module_id に一致する Module を削除する。
///
/// 注意点:
/// - 初期実装では参照が存在する場合は削除を拒否する。
/// - cascading delete は後続実装で追加する。
///
/// TODO(trace): CoreAPI仕様 / remove_module
pub fn remove_module(mut model: Model, module_id: impl AsRef<str>) -> CoreResult<Model> {
    let module_id = module_id.as_ref();

    if module_is_referenced(&model, module_id) {
        return CoreResult::fail(SansaVrmError::InvalidInput(format!(
            "Module {} is referenced",
            module_id
        )));
    }

    let before_len = model.modules.len();
    model
        .modules
        .retain(|module| module.module_id != module_id);

    if model.modules.len() == before_len {
        return CoreResult::fail(SansaVrmError::IdNotFound(module_id.to_string()));
    }

    CoreResult::ok(model)
}

/// Module が参照されているか確認する。
fn module_is_referenced(model: &Model, module_id: &str) -> bool {
    model
        .slots
        .iter()
        .any(|slot| slot.owner_module_id == module_id)
        || model
            .connections
            .iter()
            .any(|connection| connection.from_id == module_id || connection.to_id == module_id)
        || model.states.iter().any(|state| {
            state.actions.iter().any(|action| match action {
                crate::StateAction::ModuleEnable { module_id: target_id }
                | crate::StateAction::ModuleDisable { module_id: target_id } => {
                    target_id == module_id
                }
                crate::StateAction::VisibilityChange { target_id, .. } => target_id == module_id,
                _ => false,
            })
        })
}
