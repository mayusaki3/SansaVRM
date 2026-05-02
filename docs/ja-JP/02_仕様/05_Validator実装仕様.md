[目次](../目次.md) > 仕様 > Validator実装仕様

# Validator実装仕様

## 1. 目的

本仕様は、SansaVRM Format に対して実行する Validator の実装要件および検証規則を定義する。

Validator は、`04_JSONスキーマ仕様.md` による構造検証を補完し、
JSON Schema だけでは完全に検証できない参照整合性、意味整合性、
相互整合性、および変換由来の妥当性確認を担う。

本仕様の目的は以下とする。

- JSON Schema 通過後の追加整合性検証を行う
- ID参照の実在性と一意性を検証する
- `connections` と `current_connections` の整合性を検証する
- `StateAction` と対象要素の整合性を検証する
- `owner_id` や `*_ref` の参照先妥当性を検証する
- 変換時に許容される不完全性とエラー扱いを明確化する
- Core API および変換処理の前段/後段で共通利用できる検証基盤を定義する

---

## 2. 基本方針

- Validator は JSON Schema 検証の後段で実行する
- Validator は構造検証ではなく意味・参照整合検証を担当する
- 検証結果は diagnostics 形式で返却可能とする
- 検証は Fail-Fast ではなく、可能な限り複数問題を収集する
- Error / Warning / Info の3段階以上の重大度を持つ
- 自動修正は本仕様の対象外とする
- Runtime 実行判定と Validator 判定は分離する
- Validator はフォーマット非依存のメタモデル上で動作する
- glTF / VRM / URDF 由来情報は必要に応じて補助参照してよい

---

## 3. 適用範囲

本仕様は以下を対象とする。

- `SansaVRM_model`
- `SansaVRM_modules`
- `SansaVRM_slots`
- `SansaVRM_states`
- `SansaVRM_rights`
- `SansaVRM_revenue`
- `SansaVRM_compatibility`
- `SansaVRM_diagnostics`
- `SansaVRM_extension_layer`

以下は本仕様の対象外とする。

- JSON Schema による型検証
- Runtime における逐次実行順序の妥当性
- C ABI の詳細
- UI 表示仕様
- 自動補正処理
- 実行時パフォーマンス最適化アルゴリズム

---

## 4. Validator の責務

Validator は以下の責務を持つ。

### 4.1 構造後段検証

JSON Schema 通過済みデータに対し、追加の意味検証を行う。

### 4.2 参照整合性検証

ID による参照先の存在確認および参照関係の妥当性を確認する。

### 4.3 相互整合性検証

複数拡張間の相互整合を確認する。

例:

- `rights_ref` と `SansaVRM_rights`
- `revenue_ref` と `SansaVRM_revenue`
- `diagnostics_ref` と `SansaVRM_diagnostics`

### 4.4 意味整合性検証

型として正しくても意味として不正な構成を検出する。

例:

- 存在しない Slot または Connection への bind
- `owner_id` と対象実体の不一致
- `current_connections` と `connections` の矛盾

---

## 5. 入出力

### 5.1 入力

Validator の入力は以下とする。

- JSON Schema 検証済みの SansaVRM Format JSON
- 必要に応じて glTF 標準部の配列長情報
- 必要に応じて外部変換コンテキスト情報

### 5.2 出力

Validator の出力は以下とする。

- `is_valid`
- `errors[]`
- `warnings[]`
- `infos[]`
- `diagnostics_update[]`

### 5.3 出力形式

出力形式は以下の構造を基本とする。

```json
{
  "is_valid": false,
  "errors": [
    {
      "code": "REF_NOT_FOUND",
      "message": "Referenced slot_id does not exist",
      "path": "extensions.SansaVRM_states.states[0].actions[1].target_slot_id",
      "severity": "error"
    }
  ],
  "warnings": [],
  "infos": [],
  "diagnostics_update": []
}
```

---

## 6. 検証フェーズ

Validator は以下の順で検証する。

1. 事前インデックス構築
2. ID一意性検証
3. 参照実在性検証
4. 相互整合性検証
5. 意味整合性検証
6. glTF 補助整合性検証
7. diagnostics 生成

---

## 7. 事前インデックス構築

Validator は検証前に以下のインデックスを構築する。

- model_id index
- module_id index
- slot_id index
- state_id index
- rights_id index
- revenue_id index
- diagnostics_id index
- extension_layer_id index
- property_id index

### 7.1 目的

- 参照先の高速探索
- 重複IDの検出
- owner 参照の妥当性確認

### 7.2 要件

各 index は一意なキー集合として構築し、
重複があれば直ちに error を記録する。

---

## 8. ID一意性検証

### 8.1 検証対象

- `model_id`
- `module_id`
- `slot_id`
- `state_id`
- `rights_id`
- `revenue_id`
- `diagnostics_id`
- `extension_layer_id`
- `property_id`

### 8.2 検証規則

- 同一スコープ内で重複してはならない
- `model_id` は全体で一意でなければならない
- `module_id` は model内で一意でなければならない
- `slot_id` は module内で一意でなければならない
- `state_id` は model内で一意でなければならない
- `property_id` は owner 内で一意でなければならない

### 8.3 エラーコード

- `DUPLICATE_MODEL_ID`
- `DUPLICATE_MODULE_ID`
- `DUPLICATE_SLOT_ID`
- `DUPLICATE_STATE_ID`
- `DUPLICATE_PROPERTY_ID`

---

## 9. 参照実在性検証

### 9.1 検証対象

- `modules[]`
- `slots[]`
- `states[]`
- `rights_ref`
- `revenue_ref`
- `diagnostics_ref`
- `owner_module_id`
- `current_connections[]`
- `state_refs[]`
- `module_id`
- `slot_id`
- `target_slot_id`
- `expression_id`
- `property_id`
- `owner_id`

### 9.2 検証規則

参照される ID は必ず対応する実体が存在しなければならない。

### 9.3 エラーコード

- `REF_NOT_FOUND`
- `OWNER_NOT_FOUND`
- `INVALID_REF_TYPE`

---

## 10. `connections` と `current_connections` の整合性検証

### 10.1 正本

`connections` の構造は以下とする。

- connection_id
- from_id
- to_id
- connection_type
- enabled
- conditions

モデル全体の正規接続関係は `SansaVRM_model.connections` を正本とする。

### 10.2 局所参照

`current_connections` は各 Slot から見た局所接続状態を表す。

保持する値は connection_id の配列とする。

### 10.3 検証規則

- `connections` に存在する接続は、対応する双方または少なくとも定義方針に沿った `current_connections` に反映されなければならない
- `current_connections` に存在する参照は、`connections` に対応する接続が存在しなければならない
- 接続先 Slot は `target_slot_types` に適合しなければならない
- `max_connections` を超えてはならない
- `exclusive = true` の場合、複数接続してはならない
- `current_connections` に含まれる connection_id は `connections` に存在しなければならない
- `connections` に存在する connection_id は `current_connections` に反映されることが望ましい

### 10.4 エラーコード

- `CONNECTION_MISMATCH`
- `CURRENT_CONNECTION_MISMATCH`
- `TARGET_SLOT_TYPE_INVALID`
- `MAX_CONNECTION_EXCEEDED`
- `EXCLUSIVE_CONNECTION_VIOLATION`

---

## 11. `StateAction` 整合性検証

### 11.1 検証対象

- `module_enable(module_id)`
- `module_disable(module_id)`
- `slot_bind(slot_id, target_slot_id)`
- `slot_unbind(slot_id, target_slot_id)`
- `expression_change(expression_id)`
- `property_override(property_id, value)`
- `visibility_change(target_id, visible)`

### 11.2 検証規則

#### module_enable / module_disable

- `module_id` が実在しなければならない

#### slot_bind / slot_unbind

- `slot_id` が実在しなければならない
- `target_slot_id` が実在しなければならない
- `slot_id` と `target_slot_id` は接続可能な関係でなければならない

#### expression_change

- `expression_id` は有効な表現識別子でなければならない
- 現段階で専用定義が未整備の場合、warning として扱ってよい

#### property_override

- `property_id` が実在しなければならない
- `value` は対象 Property の PropertyValue 形式と整合しなければならない
- `property_type` および `context` は対象 Property の意味分類・処理文脈と整合しなければならない

#### visibility_change

- `target_id` が実在しなければならない
- `visible` は boolean でなければならない

### 11.3 エラーコード

- `STATE_ACTION_TARGET_NOT_FOUND`
- `STATE_ACTION_INVALID_BIND`
- `PROPERTY_OVERRIDE_TYPE_MISMATCH`
- `EXPRESSION_REF_UNRESOLVED`

---

## 12. `owner_id` / `*_ref` 整合性検証

### 12.1 対象

- `rights.owner_id`
- `revenue.owner_id`
- `diagnostics.owner_id`
- `extension_layer.owner_id`
- `rights_ref`
- `revenue_ref`
- `diagnostics_ref`

### 12.2 検証規則

- `owner_id` は model / module / slot 等の正当な所有対象を参照しなければならない
- `rights_ref` は `SansaVRM_rights.rights_id` を参照しなければならない
- `revenue_ref` は `SansaVRM_revenue.revenue_id` を参照しなければならない
- `diagnostics_ref` は `SansaVRM_diagnostics.diagnostics_id` を参照しなければならない

### 12.3 エラーコード

- `INVALID_OWNER_REF`
- `RIGHTS_REF_NOT_FOUND`
- `REVENUE_REF_NOT_FOUND`
- `DIAGNOSTICS_REF_NOT_FOUND`

---

## 13. `Compatibility` 整合性検証

### 13.1 検証対象

- `required_slots[]`
- `forbidden_slots[]`
- `required_tags[]`
- `forbidden_tags[]`
- `constraint_rules[]`
- `target_module_types[]`

### 13.2 検証規則

- `required_slots[]` / `forbidden_slots[]` の参照先は実在しなければならない
- 同一 ID が required / forbidden の両方に同時に存在してはならない
- `constraint_rules.result` は定義済み列挙値でなければならない
- `target_module_types[]` は許容集合内でなければならない

### 13.3 エラーコード

- `COMPATIBILITY_SLOT_NOT_FOUND`
- `COMPATIBILITY_CONFLICT`
- `COMPATIBILITY_RULE_INVALID`

---

## 14. `Rights` / `Revenue` 整合性検証

### 14.1 Rights

- `source_raw.format_type` は許容集合内でなければならない
- `normalized` 必須項目は空であってはならない
- `authors[]` は空配列でもよいが、運用上 warning を出してよい

### 14.2 Revenue

- `policy` は許容集合内でなければならない
- `weight` は 0 以上でなければならない
- `royalty_rate` は null または 0 以上でなければならない
- `fixed_fee` は null または 0 以上でなければならない

### 14.3 エラーコード

- `RIGHTS_FORMAT_INVALID`
- `RIGHTS_REQUIRED_FIELD_EMPTY`
- `REVENUE_POLICY_INVALID`
- `REVENUE_WEIGHT_INVALID`
- `REVENUE_RATE_INVALID`
- `REVENUE_FIXED_FEE_INVALID`

---

## 15. `Property` 分類整合性検証

### 15.1 検証対象

- `property_type`
- `context`
- `key`
- `value.type`

### 15.2 検証規則

- `property_type` は定義済み列挙値でなければならない
- `context` は定義済み列挙値でなければならない
- Property の値型は PropertyValue により表現される。
- `property_type` は Property が何の種類の情報かを表す。
- `context` は Property をどの処理文脈で解釈するかを表す。
- `context` は単一値でなければならない。
- Property の配置場所（model.properties / module.properties / slot.properties）と `context` は独立している。
- `property_type = Physics` の場合、`key` は物理系 Property として解釈可能でなければならない。
- `property_type = Actuator` の場合、`context` は `Control` / `Runtime` / `Simulation` のいずれかと整合する必要がある。
- `property_type = Sensor` の場合、`context` は `IO` / `Runtime` / `Control` のいずれかと整合する必要がある。
- `property_type = Geometry` / `Material` / `Texture` の場合、`context` は `Rendering` / `Conversion` のいずれかと整合する必要がある。
- `property_type = Constraint` の場合、`context` は `Validation` と整合する必要がある。

### 15.3 Property context 整合性検証

property_type と context は以下の関係を満たす必要がある。

- property_type = Physics
  - context は Simulation または Execution でなければならない
- property_type = Geometry / Material / Texture
  - context は Rendering または Conversion でなければならない
- property_type = Actuator
  - context は Control / Execution / Simulation のいずれかでなければならない
- property_type = Sensor
  - context は IO / Execution / Control のいずれかでなければならない
- property_type = Constraint
  - context は Validation でなければならない
- property_type = Metadata
  - context は Description でなければならない

上記に違反する場合、PROPERTY_CONTEXT_INVALID を error として返す。

### 15.4 エラーコード

- `PROPERTY_TYPE_INVALID`
- `PROPERTY_CONTEXT_INVALID`
- `PROPERTY_CLASSIFICATION_MISMATCH`
- `PROPERTY_GEOM_DATA_INVALID`

---

## 16. glTF 補助整合性検証

### 16.1 対象

- `node_index`
- `mesh_index`
- `material_indices[]`
- `animation_indices[]`

### 16.2 検証規則

glTF 標準部が入力に含まれる場合、各 index は対応する配列範囲内でなければならない。

### 16.3 エラーコード

- `GLTF_NODE_INDEX_OUT_OF_RANGE`
- `GLTF_MESH_INDEX_OUT_OF_RANGE`
- `GLTF_MATERIAL_INDEX_OUT_OF_RANGE`
- `GLTF_ANIMATION_INDEX_OUT_OF_RANGE`

---

## 17. diagnostics 生成規則

### 17.1 基本方針

Validator は検出した問題を diagnostics 形式へ変換可能でなければならない。

### 17.2 出力項目

- `type`
- `message`
- `severity`
- `source`

### 17.3 severity 対応

- error → `critical`
- warning → `warning`
- info → `info`

### 17.4 type 対応例

- `REF_NOT_FOUND` → `validation_error`
- `CONNECTION_MISMATCH` → `structure_error`
- `COMPATIBILITY_CONFLICT` → `compatibility_error`
- `RIGHTS_FORMAT_INVALID` → `rights_warning`

---

## 18. 重大度定義

### 18.1 error

仕様上不正であり、当該データを有効な SansaVRM Format とみなせない。

### 18.2 warning

解釈は可能だが、不完全または非推奨な状態である。

### 18.3 info

参考情報であり、処理継続を妨げない。

---

## 19. 実行インターフェース

Validator は以下のインターフェースを満たすことを推奨する。

### 19.1 入力

- `document`
- `options`
- `context`

### 19.2 出力

- `result`
- `diagnostics`

### 19.3 擬似シグネチャ

```text
validate(document, options, context) -> ValidationResult
```

---

## 20. 検証オプション

以下のオプションを許容する。

- `strict`
- `validate_gltf_indices`
- `allow_partial`
- `warning_as_error`

### 20.1 strict

仕様逸脱を可能な限り error として扱う。

### 20.2 validate_gltf_indices

glTF 標準部の index 整合も検証する。

### 20.3 allow_partial

部分シリアライズを許容する。

allow_partial が true の場合でも、以下は必須とする：

- model_id
- 参照される全ての ID は定義されていなければならない
- 検証対象拡張単位での整合性は維持されなければならない

### 20.4 warning_as_error

warning を error として扱う。

---

## 21. 非スコープ

- 自動修正
- 差分修復
- Runtime スケジューリング
- Core API の呼び出し制御
- UI への表示方法

---

## 22. 将来拡張

- 自動修正候補提示
- 変換元フォーマット別 Validator
- DSL 条件式専用 Validator
- ストリーミング検証
- 差分検証

---

[目次](../目次.md) > 仕様 > Validator実装仕様
