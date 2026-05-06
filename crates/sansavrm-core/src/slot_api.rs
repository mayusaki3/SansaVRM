// crates/sansavrm-core/src/slot_api.rs

use crate::{CoreResult, Model, SansaVrmError, Slot};

/// Slot を追加する。
///
/// 役割:
/// - Model に Slot を追加する。
/// - owner_module_id が指す Module の slots に slot_id を追加する。
///
/// 注意点:
/// - slot_id は Model 内で一意である必要がある。
/// - owner_module_id は既存 Module を参照する必要がある。
///
/// 引数:
/// - model: 更新対象Model。
/// - slot: 追加するSlot。
///
/// 戻り値:
/// - CoreResult<Model>: 更新後Model、または入力エラー。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3d7
pub fn add_slot(mut model: Model, slot: Slot) -> CoreResult<Model> {
    if model
        .slots
        .iter()
        .any(|existing| existing.slot_id == slot.slot_id)
    {
        return CoreResult::fail(SansaVrmError::DuplicateId(slot.slot_id));
    }

    let Some(owner_module) = model
        .modules
        .iter_mut()
        .find(|module| module.module_id == slot.owner_module_id)
    else {
        return CoreResult::fail(SansaVrmError::IdNotFound(slot.owner_module_id));
    };

    if !owner_module.slots.iter().any(|slot_id| slot_id == &slot.slot_id) {
        owner_module.slots.push(slot.slot_id.clone());
    }

    model.slots.push(slot);
    CoreResult::ok(model)
}

/// Slot を更新する。
///
/// 役割:
/// - slot_id に一致する Slot を差し替える。
///
/// 注意点:
/// - patch は初期実装では部分更新ではなく全体差し替えとする。
/// - 差し替え後も slot_id は維持する。
/// - owner_module_id は既存 Module を参照する必要がある。
///
/// 引数:
/// - model: 更新対象Model。
/// - slot_id: 更新対象Slot ID。
/// - patch: 差し替え後Slot。
///
/// 戻り値:
/// - CoreResult<Model>: 更新後Model、または入力エラー。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3d9
pub fn update_slot(
    mut model: Model,
    slot_id: impl AsRef<str>,
    mut patch: Slot,
) -> CoreResult<Model> {
    let slot_id = slot_id.as_ref();
    patch.slot_id = slot_id.to_string();

    if !model
        .modules
        .iter()
        .any(|module| module.module_id == patch.owner_module_id)
    {
        return CoreResult::fail(SansaVrmError::IdNotFound(patch.owner_module_id));
    }

    let Some(old_owner_module_id) = model
        .slots
        .iter()
        .find(|slot| slot.slot_id == slot_id)
        .map(|slot| slot.owner_module_id.clone())
    else {
        return CoreResult::fail(SansaVrmError::IdNotFound(slot_id.to_string()));
    };

    if let Some(slot) = model.slots.iter_mut().find(|slot| slot.slot_id == slot_id) {
        *slot = patch.clone();
    }

    if old_owner_module_id != patch.owner_module_id {
        if let Some(old_owner) = model
            .modules
            .iter_mut()
            .find(|module| module.module_id == old_owner_module_id)
        {
            old_owner.slots.retain(|existing_slot_id| existing_slot_id != slot_id);
        }

        if let Some(new_owner) = model
            .modules
            .iter_mut()
            .find(|module| module.module_id == patch.owner_module_id)
        {
            if !new_owner.slots.iter().any(|existing_slot_id| existing_slot_id == slot_id) {
                new_owner.slots.push(slot_id.to_string());
            }
        }
    }

    CoreResult::ok(model)
}

/// Slot を削除する。
///
/// 役割:
/// - slot_id に一致する Slot を削除する。
/// - owner Module の slots からも削除する。
///
/// 注意点:
/// - 初期実装では Connection / StateAction から参照されている Slot は削除を拒否する。
/// - cascading delete は後続実装で追加する。
///
/// 引数:
/// - model: 更新対象Model。
/// - slot_id: 削除対象Slot ID。
///
/// 戻り値:
/// - CoreResult<Model>: 更新後Model、または入力エラー。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3d8
pub fn remove_slot(mut model: Model, slot_id: impl AsRef<str>) -> CoreResult<Model> {
    let slot_id = slot_id.as_ref();

    if slot_is_referenced(&model, slot_id) {
        return CoreResult::fail(SansaVrmError::InvalidInput(format!(
            "Slot {} is referenced",
            slot_id
        )));
    }

    let Some(owner_module_id) = model
        .slots
        .iter()
        .find(|slot| slot.slot_id == slot_id)
        .map(|slot| slot.owner_module_id.clone())
    else {
        return CoreResult::fail(SansaVrmError::IdNotFound(slot_id.to_string()));
    };

    model.slots.retain(|slot| slot.slot_id != slot_id);

    if let Some(owner_module) = model
        .modules
        .iter_mut()
        .find(|module| module.module_id == owner_module_id)
    {
        owner_module
            .slots
            .retain(|existing_slot_id| existing_slot_id != slot_id);
    }

    CoreResult::ok(model)
}

/// Slot が参照されているか確認する。
fn slot_is_referenced(model: &Model, slot_id: &str) -> bool {
    model
        .connections
        .iter()
        .any(|connection| connection.from_id == slot_id || connection.to_id == slot_id)
        || model.states.iter().any(|state| {
            state.actions.iter().any(|action| match action {
                crate::StateAction::SlotBind {
                    slot_id: source_slot_id,
                    target_slot_id,
                }
                | crate::StateAction::SlotUnbind {
                    slot_id: source_slot_id,
                    target_slot_id,
                } => source_slot_id == slot_id || target_slot_id == slot_id,
                crate::StateAction::VisibilityChange { target_id, .. } => target_id == slot_id,
                _ => false,
            })
        })
}
