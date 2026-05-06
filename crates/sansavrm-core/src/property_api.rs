// crates/sansavrm-core/src/property_api.rs

use crate::{CoreResult, Model, Property, SansaVrmError};

/// Property を owner_id に対応する Model / Module / Slot へ追加する。
///
/// 役割:
/// - owner_id が Model ID の場合は Model 直下へ Property を追加する。
/// - owner_id が Module ID の場合は Module 配下へ Property を追加する。
/// - owner_id が Slot ID の場合は Slot 配下へ Property を追加する。
///
/// 注意点:
/// - property_id は Model 全体で一意とする。
///
/// 引数:
/// - model: 更新対象 Model。
/// - owner_id: Property 追加先の ID。
/// - property: 追加する Property。
///
/// 戻り値:
/// - CoreResult<Model>: 更新後 Model、または入力エラー。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_q3p7s6f5
pub fn add_property(
    mut model: Model,
    owner_id: impl AsRef<str>,
    property: Property,
) -> CoreResult<Model> {
    let owner_id = owner_id.as_ref();

    if property_exists_in_model(&model, &property.property_id) {
        return duplicate_property_result(&property.property_id);
    }

    if model.model_id == owner_id {
        model.properties.push(property);
        return CoreResult::ok(model);
    }

    for module in &mut model.modules {
        if module.module_id == owner_id {
            module.properties.push(property);
            return CoreResult::ok(model);
        }
    }

    for slot in &mut model.slots {
        if slot.slot_id == owner_id {
            slot.properties.push(property);
            return CoreResult::ok(model);
        }
    }

    CoreResult::fail(SansaVrmError::InvalidInput(format!(
        "Property owner not found: {}",
        owner_id
    )))
}

/// Property を更新する。
///
/// 役割:
/// - property_id に一致する Property の内容を置換する。
///
/// 注意点:
/// - owner の移動は行わない。
/// - replacement.property_id は無視し、既存 property_id を維持する。
///
/// 引数:
/// - model: 更新対象 Model。
/// - property_id: 更新対象 Property ID。
/// - replacement: 差し替え後 Property。
///
/// 戻り値:
/// - CoreResult<Model>: 更新後 Model、または入力エラー。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3f0
pub fn update_property(
    mut model: Model,
    property_id: impl AsRef<str>,
    mut replacement: Property,
) -> CoreResult<Model> {
    let property_id = property_id.as_ref();
    replacement.property_id = property_id.to_string();

    if replace_property(&mut model.properties, property_id, &replacement) {
        return CoreResult::ok(model);
    }

    for module in &mut model.modules {
        if replace_property(&mut module.properties, property_id, &replacement) {
            return CoreResult::ok(model);
        }
    }

    for slot in &mut model.slots {
        if replace_property(&mut slot.properties, property_id, &replacement) {
            return CoreResult::ok(model);
        }
    }

    CoreResult::fail(SansaVrmError::InvalidInput(format!(
        "Property not found: {}",
        property_id
    )))
}

/// Property を削除する。
///
/// 役割:
/// - property_id に一致する Property を Model / Module / Slot から削除する。
///
/// 引数:
/// - model: 更新対象 Model。
/// - property_id: 削除対象 Property ID。
///
/// 戻り値:
/// - CoreResult<Model>: 更新後 Model、または入力エラー。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3f1
pub fn remove_property(mut model: Model, property_id: impl AsRef<str>) -> CoreResult<Model> {
    let property_id = property_id.as_ref();

    if remove_property_from_vec(&mut model.properties, property_id) {
        return CoreResult::ok(model);
    }

    for module in &mut model.modules {
        if remove_property_from_vec(&mut module.properties, property_id) {
            return CoreResult::ok(model);
        }
    }

    for slot in &mut model.slots {
        if remove_property_from_vec(&mut slot.properties, property_id) {
            return CoreResult::ok(model);
        }
    }

    CoreResult::fail(SansaVrmError::InvalidInput(format!(
        "Property not found: {}",
        property_id
    )))
}

/// owner_id に対応する Property 一覧を取得する。
///
/// 役割:
/// - Model / Module / Slot のいずれかに属する Property を返す。
///
/// 引数:
/// - model: 参照対象 Model。
/// - owner_id: Property 所有元 ID。
///
/// 戻り値:
/// - CoreResult<Vec<Property>>: Property一覧、または入力エラー。
///
/// @hldocs.ref doc-20260504-000206Z-SV0G#sec_a1b2c3f2
pub fn list_properties(model: &Model, owner_id: impl AsRef<str>) -> CoreResult<Vec<Property>> {
    let owner_id = owner_id.as_ref();

    if model.model_id == owner_id {
        return CoreResult::ok(model.properties.clone());
    }

    for module in &model.modules {
        if module.module_id == owner_id {
            return CoreResult::ok(module.properties.clone());
        }
    }

    for slot in &model.slots {
        if slot.slot_id == owner_id {
            return CoreResult::ok(slot.properties.clone());
        }
    }

    CoreResult::fail(SansaVrmError::InvalidInput(format!(
        "Property owner not found: {}",
        owner_id
    )))
}

/// Property ID が Model 全体に存在するか判定する。
fn property_exists_in_model(model: &Model, property_id: &str) -> bool {
    has_property(&model.properties, property_id)
        || model
            .modules
            .iter()
            .any(|module| has_property(&module.properties, property_id))
        || model
            .slots
            .iter()
            .any(|slot| has_property(&slot.properties, property_id))
}

/// Property ID が一覧内に存在するか判定する。
fn has_property(properties: &[Property], property_id: &str) -> bool {
    properties.iter().any(|property| property.property_id == property_id)
}

/// Property ID 重複エラーを返す。
fn duplicate_property_result<T>(property_id: &str) -> CoreResult<T> {
    CoreResult::fail(SansaVrmError::InvalidInput(format!(
        "Duplicate property_id: {}",
        property_id
    )))
}

/// Property 一覧内の対象Propertyを置換する。
fn replace_property(properties: &mut [Property], property_id: &str, replacement: &Property) -> bool {
    for property in properties {
        if property.property_id == property_id {
            *property = replacement.clone();
            return true;
        }
    }
    false
}

/// Property 一覧から対象Propertyを削除する。
fn remove_property_from_vec(properties: &mut Vec<Property>, property_id: &str) -> bool {
    let old_len = properties.len();
    properties.retain(|property| property.property_id != property_id);
    properties.len() != old_len
}
