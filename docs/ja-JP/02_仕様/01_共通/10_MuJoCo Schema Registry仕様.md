<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260509-001000Z-SV0J
lang: ja-JP
canonical_title: MuJoCo Schema Registry仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > 共通 > MuJoCo Schema Registry仕様

# MuJoCo Schema Registry仕様

## 1. 目的

本仕様は、SansaVRM と MuJoCo を schema-driven に連携するための MuJoCo Schema Registry の構造、判定規則、検証責務、拡張単位を定義する。

MuJoCo Schema Registry は、SansaVRM-MuJoCo-Adapter が MJCF 入出力、Adapter 側補助成果物出力、保持のみ、未対応、source_raw 保持を判断するための正規判定情報である。

本仕様により、以下を保証する。

- MJCF 入出力可否を実装推測で判定しない
- MuJoCo バージョン差分を schema により判定する
- Adapter の対応能力を schema と照合できる
- fallback、diagnostics、conversion report の根拠を追跡できる
- SansaVRM 本体を MuJoCo runtime に依存させない

---

## 2. 基本方針

- MuJoCo Schema Registry は SansaVRM 本体が保持または参照できる登録情報とする
- SansaVRM 本体は MuJoCo runtime に依存してはならない
- Adapter は SansaVRM 本体の公開 API 経由で registry を取得する
- Adapter は registry に存在しない MuJoCo 固有パラメータを推測変換してはならない
- registry に存在しない値は `source_raw` または diagnostics の対象とする
- MJCF へ直接入出力できるかどうかは `io_scope` と `mjcf_mapping` により判定する
- Adapter 側補助成果物へ分離するかどうかは `io_scope` と `adapter_artifact` により判定する
- 対象 MuJoCo バージョンで利用できるかどうかは version schema により判定する
- Adapter が対応しているかどうかは Adapter Capability と照合して判定する

---

## 3. スコープ

本仕様の対象は以下とする。

- MuJoCo Schema Registry の構造
- registry entry の必須項目
- `io_scope` 判定規則
- `mjcf_mapping` 判定規則
- `adapter_artifact` 判定規則
- MuJoCo version schema
- Adapter Capability との照合規則
- fallback 判定規則
- diagnostics 出力規則
- conversion report への根拠記録規則

本仕様の対象外は以下とする。

- MuJoCo simulation runtime の実行
- MuJoCo 本体の API 呼び出し仕様
- 物理シミュレーション結果の妥当性評価
- リアルタイム制御
- UI / 可視化

---

## 4. Registry構成

MuJoCo Schema Registry は、以下の単位で構成する。

- registry package
- runtime schema
- registry entry
- mapping rule
- artifact rule
- fallback rule
- diagnostics rule

### 4.1 registry package

registry package は、特定 runtime または runtime version 群に対する schema 集合である。

registry package は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `schema_id` | 必須 | registry package の一意識別子 |
| `schema_version` | 必須 | registry package 自体のバージョン |
| `runtime` | 必須 | 対象 runtime。MuJoCo の場合は `mujoco` |
| `runtime_version_range` | 必須 | 対象 runtime version 範囲 |
| `entries` | 必須 | registry entry 配列 |
| `diagnostics_policy` | 必須 | diagnostics 出力方針 |

### 4.2 runtime schema

runtime schema は、対象 runtime のバージョン差分を表現するための情報である。

MuJoCo の場合、runtime schema は MJCF 要素、属性、意味、非推奨状態、fallback 方針を含む。

### 4.3 registry entry

registry entry は、1つのカスタムパラメータまたは runtime 固有パラメータの正規定義である。

registry entry の単位は `namespace + name + target_type` とする。

---

## 5. Registry Entry構造

registry entry は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `namespace` | 必須 | パラメータ名前空間 |
| `name` | 必須 | パラメータ名 |
| `target_type` | 必須 | 適用対象 |
| `value_type` | 必須 | 値型 |
| `unit` | 任意 | 単位 |
| `required` | 必須 | 必須有無 |
| `default` | 任意 | 既定値 |
| `constraints` | 任意 | 値制約 |
| `io_scope` | 必須 | 入出力範囲 |
| `mjcf_mapping` | 条件付き必須 | MJCF 直接入出力定義 |
| `adapter_artifact` | 条件付き必須 | Adapter 側補助成果物定義 |
| `runtime_version` | 必須 | 対象 MuJoCo バージョン範囲 |
| `adapter_support` | 必須 | Adapter 対応状態 |
| `fallback` | 必須 | fallback 方針 |
| `diagnostics` | 必須 | diagnostics 出力規則 |
| `description` | 必須 | 説明 |

---

## 6. target_type

`target_type` は registry entry の適用先を表す。

使用可能な値は以下とする。

- `model`
- `module`
- `connection`
- `slot`
- `property`
- `joint`
- `collider`
- `actuator`
- `sensor`

`target_type` は SansaVRM 側の保持先と MuJoCo 側の対応要素を接続するために使用する。

---

## 7. value_type

`value_type` は値型を表す。

使用可能な値は以下とする。

- `boolean`
- `integer`
- `number`
- `string`
- `vector2`
- `vector3`
- `vector4`
- `array`
- `object`
- `enum`
- `raw`

`raw` は解釈せず保持する値にのみ使用する。

---

## 8. io_scope

`io_scope` は入出力範囲を表す。

使用可能な値は以下とする。

- `mjcf`
- `adapter_artifact`
- `both`
- `preserve_only`
- `unsupported`
- `source_raw`

各値の意味は以下とする。

| io_scope | 意味 |
|---|---|
| `mjcf` | MJCF に直接入出力できる |
| `adapter_artifact` | MJCF には出力せず、Adapter 側補助成果物へ出力する |
| `both` | MJCF と Adapter 側補助成果物の両方へ出力する |
| `preserve_only` | SansaVRM 内に保持するが、外部成果物へ出力しない |
| `unsupported` | 登録済みだが現在は未対応として扱う |
| `source_raw` | 解釈せず元情報として保持する |

---

## 9. io_scope整合条件

`io_scope` と関連項目は、以下を満たさなければならない。

| io_scope | `mjcf_mapping` | `adapter_artifact` |
|---|---|---|
| `mjcf` | 必須 | 任意または禁止 |
| `adapter_artifact` | null | 必須 |
| `both` | 必須 | 必須 |
| `preserve_only` | null | null |
| `unsupported` | null | null |
| `source_raw` | null | null |

`mjcf_mapping` が必須にもかかわらず存在しない場合、schema validation error とする。

`adapter_artifact` が必須にもかかわらず存在しない場合、schema validation error とする。

`unsupported` または `source_raw` に mapping 情報が存在する場合、schema validation error とする。

---

## 10. mjcf_mapping

`mjcf_mapping` は、MJCF に直接入出力できるパラメータの対応先を表す。

`mjcf_mapping` は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `element` | 必須 | MJCF 要素名 |
| `attribute` | 任意 | MJCF 属性名 |
| `path` | 必須 | MJCF 内の論理パス |
| `direction` | 必須 | 入出力方向 |
| `value_conversion` | 任意 | 値変換規則 |
| `required_mujoco_version` | 必須 | 必要 MuJoCo バージョン |

`direction` は以下のいずれかとする。

- `import`
- `export`
- `import_export`

---

## 11. adapter_artifact

`adapter_artifact` は、Adapter 側補助成果物への出力先を表す。

`adapter_artifact` は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `artifact_type` | 必須 | 補助成果物種別 |
| `path` | 必須 | 補助成果物内の論理パス |
| `direction` | 必須 | 入出力方向 |
| `value_conversion` | 任意 | 値変換規則 |
| `required_adapter_version` | 必須 | 必要 Adapter バージョン |

`artifact_type` は以下を使用できる。

- `controller_config`
- `runtime_config`
- `conversion_report`
- `diagnostics`
- `external_metadata`

---

## 12. version判定

MuJoCo version 判定は、以下の順で行う。

1. registry package の `runtime_version_range` を確認する
2. registry entry の `runtime_version` を確認する
3. `mjcf_mapping.required_mujoco_version` を確認する
4. 対象 version が非対応または非推奨の場合、fallback を評価する
5. fallback できない場合、diagnostics error とする

version 判定では、実装側の推測により利用可否を補完してはならない。

---

## 13. Adapter Capability照合

Adapter は、自身の対応能力を Adapter Capability として宣言する。

registry entry を変換に使用する場合、以下を照合する。

- `adapter_support`
- Adapter Capability の対応 namespace
- Adapter Capability の対応 target_type
- Adapter Capability の対応 io_scope
- Adapter Capability の対応 artifact_type
- Adapter Capability の対応 MuJoCo version

Adapter Capability と registry が矛盾する場合、registry を優先し、diagnostics に矛盾を記録する。

---

## 14. fallback判定

fallback は、以下の場合に評価する。

- 対象 MuJoCo バージョンで利用できない
- Adapter Capability が未対応
- 値制約に違反している
- 必須値が不足している
- mapping 先が存在しない

fallback の挙動は以下のいずれかとする。

- `use_default`
- `preserve_only`
- `warn`
- `error`
- `ignore`

`error` の場合、strict モードでは変換成果物を出力してはならない。

---

## 15. diagnostics出力

schema-driven validation では、少なくとも以下を diagnostics に記録する。

| 項目 | 説明 |
|---|---|
| `code` | diagnostics code |
| `severity` | `info` / `warning` / `error` |
| `schema_id` | 判定に使用した registry package |
| `entry_key` | `namespace.name.target_type` |
| `target_id` | 対象 SansaVRM 要素 ID |
| `reason` | 判定理由 |
| `fallback_applied` | 適用された fallback |
| `output_scope` | 実際の出力先 |

---

## 16. conversion report連携

conversion report には、registry 判定結果を記録しなければならない。

少なくとも以下を記録する。

- 使用した registry package
- 使用した registry entry
- 使用した Adapter Capability
- MJCF に出力したパラメータ
- Adapter 側補助成果物へ出力したパラメータ
- preserve_only としたパラメータ
- unsupported としたパラメータ
- source_raw としたパラメータ
- fallback を適用したパラメータ
- diagnostics summary

---

## 17. schema validation error

以下は schema validation error とする。

- 必須項目が存在しない
- `io_scope` と `mjcf_mapping` が整合しない
- `io_scope` と `adapter_artifact` が整合しない
- `value_type` と `constraints` が整合しない
- `required = true` かつ `default` も入力値も存在しない
- 対象 MuJoCo version が registry package の範囲外である
- Adapter Capability と照合できない
- `source_raw` に解釈済み mapping が定義されている

---

## 18. 例

### 18.1 MJCF直接入出力

```json
{
  "namespace": "mujoco",
  "name": "armature",
  "target_type": "joint",
  "value_type": "number",
  "unit": "kg*m^2",
  "required": false,
  "default": 0.0,
  "constraints": {
    "min": 0.0,
    "max": null,
    "enum": null
  },
  "io_scope": "mjcf",
  "mjcf_mapping": {
    "element": "joint",
    "attribute": "armature",
    "path": "joint.@armature",
    "direction": "import_export",
    "value_conversion": null,
    "required_mujoco_version": {
      "min": "2.3.0",
      "max": null
    }
  },
  "adapter_artifact": null,
  "runtime_version": {
    "min": "2.3.0",
    "max": null
  },
  "adapter_support": {
    "sansa-vrm-mujoco-adapter": "supported"
  },
  "fallback": {
    "behavior": "use_default",
    "value": 0.0
  },
  "diagnostics": {
    "unsupported_code": "MUJOCO_SCHEMA_UNSUPPORTED_PARAMETER",
    "fallback_code": "MUJOCO_SCHEMA_FALLBACK_APPLIED"
  },
  "description": "MuJoCo joint-side reflected inertia."
}
```

### 18.2 Adapter補助成果物

```json
{
  "namespace": "mujoco",
  "name": "command_delay_ms",
  "target_type": "actuator",
  "value_type": "number",
  "unit": "ms",
  "required": false,
  "default": 0,
  "constraints": {
    "min": 0,
    "max": null,
    "enum": null
  },
  "io_scope": "adapter_artifact",
  "mjcf_mapping": null,
  "adapter_artifact": {
    "artifact_type": "controller_config",
    "path": "actuators[].command_delay_ms",
    "direction": "export",
    "value_conversion": null,
    "required_adapter_version": {
      "min": "0.1.0",
      "max": null
    }
  },
  "runtime_version": {
    "min": null,
    "max": null
  },
  "adapter_support": {
    "sansa-vrm-mujoco-adapter": "supported"
  },
  "fallback": {
    "behavior": "warn",
    "value": 0
  },
  "diagnostics": {
    "unsupported_code": "MUJOCO_SCHEMA_UNSUPPORTED_PARAMETER",
    "fallback_code": "MUJOCO_SCHEMA_FALLBACK_APPLIED"
  },
  "description": "Command delay applied by the MuJoCo adapter runtime."
}
```

---

## 19. 実装責務分離

### 19.1 SansaVRM本体

SansaVRM 本体は以下を担当する。

- registry package の保持または参照
- registry entry の構造検証
- カスタムパラメータ値の保持
- Adapter へ公開 API 経由で registry を提供すること
- Adapter から返却された diagnostics と conversion report の保持

### 19.2 SansaVRM-MuJoCo-Adapter

SansaVRM-MuJoCo-Adapter は以下を担当する。

- registry package の取得
- Adapter Capability の提示
- registry と Adapter Capability の照合
- MJCF 生成
- Adapter 側補助成果物生成
- fallback 適用
- diagnostics 生成
- conversion report 生成

---

## 20. 検証要件

MuJoCo Schema Registry に対するテスト仕様では、少なくとも以下を検証する。

- `io_scope = mjcf` で `mjcf_mapping` が必須であること
- `io_scope = adapter_artifact` で `adapter_artifact` が必須であること
- `io_scope = both` で両方が必須であること
- `io_scope = preserve_only` で外部成果物へ出力されないこと
- `io_scope = unsupported` で diagnostics が出力されること
- `io_scope = source_raw` で解釈されないこと
- version 範囲外で fallback が適用されること
- fallback `error` では strict モードの成果物出力が禁止されること
- Adapter Capability 未対応時に diagnostics が出力されること
- conversion report に registry 判定根拠が記録されること

---

[目次](../../目次.md) > 仕様 > 共通 > MuJoCo Schema Registry仕様
