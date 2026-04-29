// crates/sansavrm-core/src/property_api.rs

use crate::{CoreResult, Model, Property, SansaVrmError};

/// Property を追加する。
///
/// 役割:
/// - `owner_id` が指す Module または Slot に Property を追加する。
///
/// 注意点:
/// - `owner_id` は Module ID または Slot ID を指定する。
/// - `property_id` は Model 全体で一意である必要がある。
///
/// 引数:
/// - `model`: 更新対象 Model
/// - `owner_id`: Property 所有者 ID
/// - `property`: 追加する Property
///
/// 戻り値:
/// - 成功時: Property 追加後の Model
/// - 失敗時: owner_id 不明、または property_id 重複
///
/// TODO(trace): CoreAPI仕様 / add_property
pub fn add_property(
    mut model: Model,
    owner_id: impl AsRef<str>,
    property: Property,
) -> CoreResult<Model> {
    let owner_id = owner_id.as_ref();

    if property_exists(&model, &property.property_id) {
        return CoreResult::fail(SansaVrmError::DuplicateId(property.property_id));
    }

    if let Some(module) = model
        .modules
        .iter_mut()
        .find(|module| module.module_id == owner_id)
    {
        module.properties.push(property);
        return CoreResult::ok(model);
    }

    if let Some(slot) = model.slots.iter_mut().find(|slot| slot.slot_id == owner_id) {
        slot.properties.push(property);
        return CoreResult::ok(model);
    }

    CoreResult::fail(SansaVrmError::IdNotFound(owner_id.to_string()))
}

/// Property を更新する。
///
/// 役割:
/// - `property_id` に一致する Property を差し替える。
///
/// 注意点:
/// - `patch` は初期実装では部分更新ではなく全体差し替えとする。
/// - 差し替え後も property_id は維持する。
///
/// 引数:
/// - `model`: 更新対象 Model
/// - `property_id`: 更新対象 Property ID
/// - `patch`: 更新後 Property
///
/// 戻り値:
/// - 成功時: Property 更新後の Model
/// - 失敗時: property_id 不明
///
/// TODO(trace): CoreAPI仕様 / update_property
pub fn update_property(
    mut model: Model,
    property_id: impl AsRef<str>,
    mut patch: Property,
) -> CoreResult<Model> {
    let property_id = property_id.as_ref();
    patch.property_id = property_id.to_string();

    for module in &mut model.modules {
        if let Some(property) = module
            .properties
            .iter_mut()
            .find(|property| property.property_id == property_id)
        {
            *property = patch;
            return CoreResult::ok(model);
        }
    }

    for slot in &mut model.slots {
        if let Some(property) = slot
            .properties
            .iter_mut()
            .find(|property| property.property_id == property_id)
        {
            *property = patch;
            return CoreResult::ok(model);
        }
    }

    CoreResult::fail(SansaVrmError::IdNotFound(property_id.to_string()))
}

/// Property を削除する。
///
/// 役割:
/// - `property_id` に一致する Property を Module または Slot から削除する。
///
/// 引数:
/// - `model`: 更新対象 Model
/// - `property_id`: 削除対象 Property ID
///
/// 戻り値:
/// - 成功時: Property 削除後の Model
/// - 失敗時: property_id 不明
///
/// TODO(trace): CoreAPI仕様 / remove_property
pub fn remove_property(mut model: Model, property_id: impl AsRef<str>) -> CoreResult<Model> {
    let property_id = property_id.as_ref();

    for module in &mut model.modules {
        let before_len = module.properties.len();
        module
            .properties
            .retain(|property| property.property_id != property_id);

        if module.properties.len() != before_len {
            return CoreResult::ok(model);
        }
    }

    for slot in &mut model.slots {
        let before_len = slot.properties.len();
        slot.properties
            .retain(|property| property.property_id != property_id);

        if slot.properties.len() != before_len {
            return CoreResult::ok(model);
        }
    }

    CoreResult::fail(SansaVrmError::IdNotFound(property_id.to_string()))
}

/// Property 一覧を取得する。
///
/// 役割:
/// - `owner_id` が指す Module または Slot の Property 一覧を返す。
///
/// 引数:
/// - `model`: 参照対象 Model
/// - `owner_id`: Property 所有者 ID
///
/// 戻り値:
/// - 成功時: Property 一覧
/// - 失敗時: owner_id 不明
///
/// TODO(trace): CoreAPI仕様 / list_properties
pub fn list_properties(model: &Model, owner_id: impl AsRef<str>) -> CoreResult<Vec<Property>> {
    let owner_id = owner_id.as_ref();

    if let Some(module) = model.modules.iter().find(|module| module.module_id == owner_id) {
        return CoreResult::ok(module.properties.clone());
    }

    if let Some(slot) = model.slots.iter().find(|slot| slot.slot_id == owner_id) {
        return CoreResult::ok(slot.properties.clone());
    }

    CoreResult::fail(SansaVrmError::IdNotFound(owner_id.to_string()))
}

/// Property ID が Model 全体で既に存在するか確認する。
fn property_exists(model: &Model, property_id: &str) -> bool {
    model
        .modules
        .iter()
        .flat_map(|module| module.properties.iter())
        .any(|property| property.property_id == property_id)
        || model
            .slots
            .iter()
            .flat_map(|slot| slot.properties.iter())
            .any(|property| property.property_id == property_id)
}
