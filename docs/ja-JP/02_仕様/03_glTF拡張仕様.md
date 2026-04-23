[目次](../目次.md) > 仕様 > glTF拡張仕様

# glTF拡張仕様

## 1. 目的

本仕様は、SansaVRM Format を glTF の拡張として表現するための
データ構造および記述規則を定義する。

本仕様により、SansaVRM Core が扱う内部メタモデルを、
glTF を物理表現基盤とする外部交換表現へ
一貫してシリアライズ／デシリアライズ可能とする。

---

## 2. 基本方針

- glTF 2.0 を物理表現基盤として利用する
- SansaVRM Format は glTF 本体とは独立した論理フォーマットとする
- 本仕様では glTF extensions を用いて SansaVRM Format を保持する
- glTF 標準で表現可能な情報は標準構造を優先して用いる
- SansaVRM 固有情報のみを拡張領域に保持する
- 非可逆変換情報は diagnostics および source_raw に保持する
- 部分シリアライズを許容する
- VRM / URDF / custom 由来情報は Extension Layer に保持する

---

## 3. 適用範囲

本仕様は以下を対象とする。

- Model
- Module
- Slot
- State
- Rights
- Revenue
- Compatibility
- diagnostics
- source_raw
- 拡張層情報

以下は本仕様の対象外とする。

- glTF 標準仕様そのものの定義
- VRM 独自仕様の全文定義
- URDF 独自仕様の全文定義
- Core API の定義
- Runtime 実行ロジック

---

## 4. 拡張全体構成

SansaVRM Format は、glTF の `extensions` 領域に以下の拡張群として格納する。

- `SansaVRM_model`
- `SansaVRM_modules`
- `SansaVRM_slots`
- `SansaVRM_states`
- `SansaVRM_rights`
- `SansaVRM_revenue`
- `SansaVRM_compatibility`
- `SansaVRM_diagnostics`
- `SansaVRM_extension_layer`

### 4.1 extensionsUsed

SansaVRM を含む glTF では、使用する拡張を `extensionsUsed` に列挙する。

例:

```json
{
  "extensionsUsed": [
    "SansaVRM_model",
    "SansaVRM_modules",
    "SansaVRM_slots",
    "SansaVRM_states",
    "SansaVRM_rights",
    "SansaVRM_revenue",
    "SansaVRM_compatibility",
    "SansaVRM_diagnostics",
    "SansaVRM_extension_layer"
  ]
}
```

### 4.2 extensionsRequired

SansaVRM Format を完全な SansaVRM モデルとして解釈させる場合、
少なくとも以下を extensionsRequired に含めることを必須とする。

- `SansaVRM_model`
- `SansaVRM_modules`
- `SansaVRM_slots`

また、状態・権利・収益・適合条件の解釈を必要とする場合は、
対応する拡張を extensionsRequired に追加しなければならない。

---

## 5. ルート拡張配置

SansaVRM 拡張は原則として glTF ルートの `extensions` に配置する。

例:

```json
{
  "asset": {
    "version": "2.0"
  },
  "extensions": {
    "SansaVRM_model": { ... },
    "SansaVRM_modules": { ... },
    "SansaVRM_slots": { ... },
    "SansaVRM_states": { ... },
    "SansaVRM_rights": { ... },
    "SansaVRM_revenue": { ... },
    "SansaVRM_compatibility": { ... },
    "SansaVRM_diagnostics": { ... },
    "SansaVRM_extension_layer": { ... }
  }
}
```

ノード、メッシュ、マテリアル等に局所的に関連付ける場合は、
当該 glTF 要素の `extensions` に補助情報を配置してよい。
ただし、SansaVRM の正本はルート拡張とする。

---

## 6. IDおよび参照規則

### 6.1 ID形式

ID は以下のいずれかを許容する。

- UUID
- パス形式

例:

- `550e8400-e29b-41d4-a716-446655440000`
- `model/main`
- `model/main/module/body`
- `module/body.slot/head_mount`

### 6.2 スコープ

- `model_id` はグローバル一意
- `module_id` は `model_id` 配下で一意
- `slot_id` は `module_id` 配下で一意
- `state_id` は `model_id` 配下で一意
- `property_id` は所有要素配下で一意

### 6.3 参照方式

参照は以下を許容する。

- ID直接参照
- パス参照

例:

```json
{
  "module_id": "module/body",
  "slot_id": "module/body.slot/head_mount"
}
```

### 6.4 glTF要素との参照

必要に応じて glTF 標準要素への参照を保持する。

- `node_index`
- `mesh_index`
- `material_index`
- `texture_index`
- `animation_index`

これらは glTF 配列インデックスを参照する。

---

## 7. SansaVRM_model

`SansaVRM_model` はモデル全体のメタ情報および接続グラフを保持する。

### 7.1 構造

```json
{
  "model_id": "model/main",
  "modules": ["module/body", "module/hat_a"],
  "slots": ["module/body.slot/head_mount"],
  "states": ["state/default", "state/hat_a"],
  "rights_ref": "rights/model/main",
  "revenue_ref": "revenue/model/main",
  "diagnostics_ref": "diagnostics/model/main",
  "connections": [
    {
      "from_slot_id": "module/hat_a.slot/root",
      "to_slot_id": "module/body.slot/head_mount",
      "connection_type": "equipment"
    }
  ]
}
```

### 7.2 必須項目

- `model_id`

### 7.3 任意項目

- `modules`
- `slots`
- `states`
- `rights_ref`
- `revenue_ref`
- `diagnostics_ref`
- `connections`

---

## 8. SansaVRM_modules

`SansaVRM_modules` はモジュール一覧を保持する。

### 8.1 構造

```json
{
  "modules": [
    {
      "module_id": "module/body",
      "type": "Body",
      "slots": [
        "module/body.slot/head_mount"
      ],
      "properties": [
        {
          "property_id": "prop/body/display_name",
          "key": "display_name",
          "value": "Body",
          "value_type": "string"
        }
      ],
      "state_refs": [
        "state/default"
      ],
      "glTF_binding": {
        "node_index": 0,
        "mesh_index": 0
      }
    }
  ]
}
```

### 8.2 module.type

以下を基本とする。

- Body
- Clothing
- Accessory
- Equipment
- Prop
- Module
- WorldObject
- CompositeRoot
- Custom

### 8.3 glTF_binding

glTF 標準要素との対応を保持する。

- `node_index`
- `mesh_index`
- `material_indices[]`
- `animation_indices[]`

---

## 9. SansaVRM_slots

`SansaVRM_slots` は接続・装着・状態制御の基盤を保持する。

### 9.1 構造

```json
{
  "slots": [
    {
      "slot_id": "module/body.slot/head_mount",
      "slot_type": "Equipment",
      "owner_module_id": "module/body",
      "target_slot_types": ["Structure", "Equipment"],
      "current_connections": [
        "module/hat_a.slot/root"
      ],
      "connection_rules": {
        "min_connections": 0,
        "max_connections": 1,
        "exclusive": true,
        "replace_mode": "replace",
        "priority": 100
      },
      "constraints": [
        {
          "property_id": "constraint/head_mount/required_tag",
          "key": "required_tag",
          "value": "head_item",
          "value_type": "string"
        }
      ],
      "properties": [
        {
          "property_id": "prop/head_mount/display_name",
          "key": "display_name",
          "value": "Head Mount",
          "value_type": "string"
        }
      ]
    }
  ]
}
```

モデル全体の正規接続関係は SansaVRM_model.connections を正本とする。
SansaVRM_slots.current_connections は各 Slot から見た局所参照であり、
SansaVRM_model.connections と整合しなければならない。

### 9.2 slot_type

以下を基本とする。

- Structure
- Equipment
- State
- Rights
- Revenue
- Custom

### 9.3 connection_rules

- `min_connections`
- `max_connections`
- `exclusive`
- `replace_mode`
- `priority`

### 9.4 constraints

`constraints` は Property.constraints と同一仕様を持つ条件定義とする。

---

## 10. SansaVRM_states

`SansaVRM_states` は状態定義を保持する。

### 10.1 構造

```json
{
  "states": [
    {
      "state_id": "state/hat_a",
      "category": "Equipment",
      "conditions": {
        "operator": "AND",
        "expressions": [
          {
            "left": "prop/current_mode",
            "comparator": "eq",
            "right": "hat_a"
          }
        ]
      },
      "actions": [
        {
          "action": "module_enable",
          "module_id": "module/hat_a"
        },
        {
          "action": "slot_bind",
          "slot_id": "module/hat_a.slot/root",
          "target_slot_id": "module/body.slot/head_mount"
        }
      ],
      "priority": 100,
      "enabled": true
    }
  ]
}
```

### 10.2 category

以下を基本とする。

- Expression
- Configuration
- Equipment
- Visibility

### 10.3 conditions

```json
{
  "operator": "AND",
  "expressions": [
    {
      "left": "prop/current_mode",
      "comparator": "eq",
      "right": "default"
    }
  ]
}
```

#### comparator

- `eq`
- `neq`
- `gt`
- `gte`
- `lt`
- `lte`
- `in`
- `not_in`

### 10.4 actions

action は以下を基本とする。

- `module_enable`
- `module_disable`
- `slot_bind`
- `slot_unbind`
- `expression_change`
- `property_override`
- `visibility_change`

### 10.5 action パラメータ

#### module_enable / module_disable

- `module_id`

#### slot_bind / slot_unbind

- `slot_id`
- `target_slot_id`

#### expression_change

- `expression_id`

#### property_override

- `property_id`
- `value`

#### visibility_change

- `target_id`
- `visible`

---

## 11. SansaVRM_rights

`SansaVRM_rights` は権利情報を保持する。

### 11.1 構造

```json
{
  "rights": [
    {
      "rights_id": "rights/model/main",
      "owner_id": "model/main",
      "normalized": {
        "authors": ["author_a"],
        "license_name": "Custom License",
        "commercial_use_allowed": true,
        "redistribution_allowed": false,
        "modification_allowed": true,
        "attribution_required": true,
        "violent_expression_allowed": true,
        "sexual_expression_allowed": false,
        "political_or_religious_use_allowed": false,
        "prohibited_uses": ["illegal_use"],
        "additional_notes": "sample"
      },
      "source_raw": {
        "format_type": "VRM",
        "raw": {}
      },
      "diagnostics": [
        "unmapped_license_field"
      ]
    }
  ]
}
```

### 11.2 normalized

要件定義の Rights Slot に対応する正規化情報を保持する。

### 11.3 source_raw

- `format_type`
- `raw`

### 11.4 diagnostics

権利変換上の問題を保持する。

---

## 12. SansaVRM_revenue

`SansaVRM_revenue` は収益分配情報を保持する。

### 12.1 構造

```json
{
  "revenue": [
    {
      "revenue_id": "revenue/model/main",
      "owner_id": "model/main",
      "distribution_rules": [
        {
          "rule_id": "rule/default",
          "policy": "proportional"
        }
      ],
      "stakeholders": [
        {
          "contributor_id": "user_a",
          "contributor_type": "individual",
          "contribution_role": "creator",
          "weight": 1.0,
          "royalty_rate": 0.5,
          "fixed_fee": null,
          "external_account": "acct_001"
        }
      ],
      "logs": []
    }
  ]
}
```

### 12.2 policy

- `proportional`
- `fixed_priority`
- `minimum_guarantee`
- `exclusive`

---

## 13. SansaVRM_compatibility

`SansaVRM_compatibility` は適合条件を保持する。

### 13.1 構造

```json
{
  "compatibility": [
    {
      "compatibility_id": "compat/module/hat_a",
      "owner_id": "module/hat_a",
      "required_slots": ["module/body.slot/head_mount"],
      "forbidden_slots": [],
      "required_tags": ["humanoid"],
      "forbidden_tags": ["static_only"],
      "constraint_rules": [
        {
          "rule_id": "rule/head_mount_required",
          "condition": "required_slots contains module/body.slot/head_mount",
          "result": "ALLOW",
          "severity": "normal"
        }
      ],
      "target_module_types": ["Body"]
    }
  ]
}
```

### 13.2 判定結果

- `ALLOW`
- `CONDITIONAL`
- `DENY`

### 13.3 severity

- `info`
- `normal`
- `warning`
- `critical`

---

## 14. SansaVRM_diagnostics

`SansaVRM_diagnostics` は非正規情報および変換・判定上の問題を保持する。

### 14.1 構造

```json
{
  "diagnostics": [
    {
      "diagnostics_id": "diagnostics/model/main",
      "owner_id": "model/main",
      "items": [
        {
          "type": "non_reversible_conversion",
          "message": "Some VRM metadata was not fully mapped",
          "severity": "warning",
          "source": "VRM"
        }
      ]
    }
  ]
}
```

### 14.2 type

- `non_reversible_conversion`
- `validation_error`
- `rights_warning`
- `structure_error`
- `compatibility_error`
- `runtime_warning`
- `custom`

---

## 15. SansaVRM_extension_layer

`SansaVRM_extension_layer` は外部フォーマット由来情報を保持する。

### 15.1 構造

```json
{
  "extension_layers": [
    {
      "extension_layer_id": "ext/model/main",
      "owner_id": "model/main",
      "format_type": "VRM",
      "extensions": {
        "VRM_meta": {}
      },
      "source_raw": {
        "raw": {}
      },
      "diagnostics": [
        "partial_mapping"
      ]
    }
  ]
}
```

### 15.2 format_type

- `glTF`
- `VRM`
- `URDF`
- `custom`

---

## 16. Property 共通表現

Property は以下の共通構造で表現する。

```json
{
  "property_id": "prop/example",
  "key": "display_name",
  "value": "Body",
  "value_type": "string",
  "constraints": {
    "required": true
  },
  "metadata": {
    "description": "display label",
    "unit": null,
    "source": "manual",
    "notes": null
  }
}
```

### 16.1 value_type

- `string`
- `number`
- `boolean`
- `object`
- `array`

### 16.2 constraints

以下を基本とする。

- `required`
- `enum`
- `min`
- `max`
- `min_length`
- `max_length`
- `pattern`

---

## 17. シリアライズ方針

### 17.1 基本方針

- JSON 構造を基本とする
- glTF ルート `extensions` に格納する
- 必要に応じてノード等の局所 `extensions` に補助配置してよい
- ルート拡張を正本とする
- 部分シリアライズを許容する

### 17.2 部分シリアライズ

以下を許容する。

- states のみ保持
- rights のみ保持
- revenue のみ保持
- diagnostics のみ保持

ただし、部分シリアライズであることが解釈に影響する場合は
diagnostics に記録する。

---

## 18. バリデーション規則

### 18.1 必須

- `SansaVRM_model.model_id` は必須
- 参照先 ID は存在しなければならない
- `slot_bind` の対象 Slot は実在しなければならない
- `current_connections` は `connections` と整合しなければならない

### 18.2 推奨

- ID は UUID または一貫したパス形式を用いる
- `format_type` は既定値集合を用いる
- severity は既定値集合を用いる

---

## 19. 非可逆変換

変換時に失われる情報は以下に保持する。

- `SansaVRM_diagnostics`
- `SansaVRM_extension_layer.source_raw`
- `SansaVRM_rights.source_raw`

---

## 20. 非スコープ

- glTF 標準仕様全文
- VRM 標準仕様全文
- URDF 標準仕様全文
- Runtime API の詳細
- C ABI の詳細

---

## 21. 将来拡張

- AI 属性保持
- 条件式言語拡張
- 分散状態同期
- 外部ライセンスサービス連携
- マーケットプレイス固有拡張

---

[目次](../目次.md) > 仕様 > glTF拡張仕様
