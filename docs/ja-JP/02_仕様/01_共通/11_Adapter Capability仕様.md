<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260509-001100Z-SV0J
lang: ja-JP
canonical_title: Adapter Capability仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > 共通 > Adapter Capability仕様

# Adapter Capability仕様

## 1. 目的

本仕様は、SansaVRM と外部形式または外部 runtime を接続する Adapter が、自身の対応能力を宣言するための Adapter Capability の構造、判定規則、検証責務を定義する。

Adapter Capability は、Schema Registry と照合され、Adapter が特定の namespace、target_type、io_scope、mapping、artifact、runtime version、値変換に対応しているかを判定するために使用する。

本仕様により、以下を保証する。

- Adapter の対応範囲を実装推測で判定しない
- Schema Registry と Adapter の対応能力を機械的に照合できる
- Adapter 未対応時の fallback と diagnostics を一貫して出力できる
- MuJoCo、URDF、FBX、VRM、Unity、O3DE 等の Adapter に共通する能力宣言形式を提供する
- SansaVRM 本体を個別 Adapter の内部実装に依存させない

---

## 2. 基本方針

- Adapter は自身の対応能力を Adapter Capability として宣言しなければならない
- Adapter Capability は SansaVRM 本体または検証処理から参照できなければならない
- Adapter Capability は Schema Registry と照合可能でなければならない
- Adapter Capability に存在しない能力を、実装側の推測で補完してはならない
- Schema Registry と Adapter Capability が矛盾する場合、Schema Registry を優先する
- 矛盾または未対応は diagnostics に記録する
- Adapter Capability は runtime 非依存の共通構造と runtime 固有拡張の両方を持てる

---

## 3. スコープ

本仕様の対象は以下とする。

- Adapter Capability package の構造
- Adapter identity
- 対応 runtime / external format 宣言
- 対応 namespace 宣言
- 対応 target_type 宣言
- 対応 io_scope 宣言
- 対応 mapping 宣言
- 対応 artifact 宣言
- 対応 value_conversion 宣言
- 対応 runtime version 宣言
- Schema Registry との照合規則
- diagnostics 連携
- conversion report 連携

本仕様の対象外は以下とする。

- Adapter の内部実装
- 外部 runtime の実行
- 変換アルゴリズムの詳細
- 物理シミュレーション結果の評価
- UI / 可視化

---

## 4. Adapter Capability package

Adapter Capability package は、Adapter の対応能力を記録する正規単位である。

Adapter Capability package は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `capability_id` | 必須 | Capability package の一意識別子 |
| `capability_version` | 必須 | Capability package 自体のバージョン |
| `adapter_id` | 必須 | Adapter の一意識別子 |
| `adapter_version` | 必須 | Adapter 実装バージョン |
| `runtime` | 必須 | 対象 runtime または external format |
| `runtime_version_range` | 任意 | 対応 runtime version 範囲 |
| `supported_namespaces` | 必須 | 対応 namespace 配列 |
| `supported_targets` | 必須 | 対応 target_type 配列 |
| `supported_io_scopes` | 必須 | 対応 io_scope 配列 |
| `supported_mappings` | 任意 | 対応 mapping 配列 |
| `supported_artifacts` | 任意 | 対応 artifact 配列 |
| `supported_value_conversions` | 任意 | 対応 value_conversion 配列 |
| `unsupported_entries` | 任意 | 明示的な未対応 entry 配列 |
| `diagnostics_policy` | 必須 | diagnostics 出力方針 |

---

## 5. adapter_id

`adapter_id` は Adapter を一意に識別する文字列である。

例：

- `sansa-vrm-mujoco-adapter`
- `sansa-vrm-urdf-adapter`
- `sansa-vrm-fbx-adapter`
- `sansa-vrm-vrm-adapter`
- `sansa-vrm-unity-adapter`
- `sansa-vrm-o3de-adapter`

`adapter_id` は Schema Registry の `adapter_support` と照合される。

---

## 6. runtime

`runtime` は Adapter が対象とする runtime または external format を表す。

使用可能な値の例は以下とする。

- `mujoco`
- `urdf`
- `fbx`
- `vrm`
- `unity`
- `o3de`
- `generic`

MuJoCo Adapter の場合は `mujoco` とする。

---

## 7. supported_namespaces

`supported_namespaces` は Adapter が解釈可能な namespace を表す。

例：

```json
[
  "mujoco",
  "sansa",
  "experimental"
]
```

Schema Registry の registry entry に含まれる `namespace` が `supported_namespaces` に存在しない場合、Adapter 未対応と判定する。

---

## 8. supported_targets

`supported_targets` は Adapter が処理可能な `target_type` を表す。

使用可能な値は Schema Registry の `target_type` と一致させる。

- `model`
- `module`
- `connection`
- `slot`
- `property`
- `joint`
- `collider`
- `actuator`
- `sensor`

registry entry の `target_type` が `supported_targets` に存在しない場合、Adapter 未対応と判定する。

---

## 9. supported_io_scopes

`supported_io_scopes` は Adapter が処理可能な `io_scope` を表す。

使用可能な値は以下とする。

- `mjcf`
- `adapter_artifact`
- `both`
- `preserve_only`
- `unsupported`
- `source_raw`

Adapter が `mjcf` に対応していない場合、MJCF 直接入出力を行ってはならない。

Adapter が `adapter_artifact` に対応していない場合、補助成果物を出力してはならない。

`preserve_only` と `source_raw` は、Adapter が値を解釈しない場合でも、保持または転記の扱いとして宣言してよい。

---

## 10. supported_mappings

`supported_mappings` は Adapter が対応する external format 側 mapping を表す。

MuJoCo Adapter の場合、`supported_mappings` は MJCF 要素および属性への対応を表す。

`supported_mappings` は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `element` | 必須 | 対応要素 |
| `attribute` | 任意 | 対応属性 |
| `direction` | 必須 | 対応方向 |
| `runtime_version` | 任意 | 対応 runtime version 範囲 |
| `value_conversion_types` | 任意 | 対応値変換種別 |

`direction` は以下のいずれかとする。

- `import`
- `export`
- `import_export`

---

## 11. supported_artifacts

`supported_artifacts` は Adapter が生成または読み取り可能な補助成果物を表す。

`supported_artifacts` は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `artifact_type` | 必須 | 補助成果物種別 |
| `direction` | 必須 | 対応方向 |
| `schema_version` | 任意 | 補助成果物 schema version |
| `path_patterns` | 任意 | 対応 path pattern |

`artifact_type` は以下を使用できる。

- `controller_config`
- `runtime_config`
- `conversion_report`
- `diagnostics`
- `external_metadata`

---

## 12. supported_value_conversions

`supported_value_conversions` は Adapter が対応する値変換種別を表す。

例：

- `identity`
- `unit_scale`
- `symmetric_range`
- `enum_map`
- `vector_expand`
- `vector_reduce`
- `custom`

Schema Registry の `value_conversion.type` が `supported_value_conversions` に存在しない場合、Adapter 未対応と判定する。

`custom` を使用する場合は、Adapter 固有実装への依存が発生するため、conversion report に根拠を記録しなければならない。

---

## 13. unsupported_entries

`unsupported_entries` は、Adapter が明示的に未対応と宣言する registry entry を表す。

`unsupported_entries` は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `namespace` | 必須 | namespace |
| `name` | 必須 | parameter name |
| `target_type` | 必須 | target_type |
| `reason` | 必須 | 未対応理由 |
| `fallback_hint` | 任意 | 推奨 fallback |

`unsupported_entries` に一致する registry entry は、他の supported 条件を満たしていても未対応として扱う。

---

## 14. 照合順序

Schema Registry と Adapter Capability の照合は、以下の順で行う。

1. `adapter_id` を照合する
2. `runtime` を照合する
3. `runtime_version_range` を照合する
4. registry entry の `namespace` を `supported_namespaces` と照合する
5. registry entry の `target_type` を `supported_targets` と照合する
6. registry entry の `io_scope` を `supported_io_scopes` と照合する
7. `unsupported_entries` に一致しないことを確認する
8. `mjcf_mapping` または external mapping を `supported_mappings` と照合する
9. `adapter_artifact` を `supported_artifacts` と照合する
10. `value_conversion` を `supported_value_conversions` と照合する
11. 照合失敗時は fallback を評価する
12. fallback できない場合は diagnostics error とする

---

## 15. 照合結果

照合結果は以下のいずれかとする。

- `supported`
- `supported_with_fallback`
- `preserve_only`
- `source_raw`
- `unsupported`
- `error`

各値の意味は以下とする。

| 結果 | 意味 |
|---|---|
| `supported` | Adapter が registry entry を仕様通り処理できる |
| `supported_with_fallback` | fallback 適用により処理可能 |
| `preserve_only` | SansaVRM 内に保持するのみ |
| `source_raw` | 解釈せず元情報として保持 |
| `unsupported` | 登録済みだが Adapter は未対応 |
| `error` | strict モードで成果物出力を禁止するエラー |

---

## 16. diagnostics出力

Adapter Capability 照合では、少なくとも以下を diagnostics に記録する。

| 項目 | 説明 |
|---|---|
| `code` | diagnostics code |
| `severity` | `info` / `warning` / `error` |
| `adapter_id` | 使用した Adapter |
| `capability_id` | 使用した Adapter Capability |
| `schema_id` | 使用した Schema Registry |
| `entry_key` | `namespace.name.target_type` |
| `target_id` | 対象 SansaVRM 要素 ID |
| `capability_check` | 失敗または成功した照合項目 |
| `result` | 照合結果 |
| `fallback_applied` | 適用 fallback |
| `reason` | 判定理由 |

---

## 17. diagnostics code

Adapter Capability 照合で使用する diagnostics code は以下を基本とする。

| code | severity | 意味 |
|---|---|---|
| `ADAPTER_CAPABILITY_UNSUPPORTED_NAMESPACE` | warning / error | namespace 未対応 |
| `ADAPTER_CAPABILITY_UNSUPPORTED_TARGET` | warning / error | target_type 未対応 |
| `ADAPTER_CAPABILITY_UNSUPPORTED_IO_SCOPE` | warning / error | io_scope 未対応 |
| `ADAPTER_CAPABILITY_UNSUPPORTED_MAPPING` | warning / error | mapping 未対応 |
| `ADAPTER_CAPABILITY_UNSUPPORTED_ARTIFACT` | warning / error | artifact 未対応 |
| `ADAPTER_CAPABILITY_UNSUPPORTED_VALUE_CONVERSION` | warning / error | value_conversion 未対応 |
| `ADAPTER_CAPABILITY_EXPLICIT_UNSUPPORTED_ENTRY` | warning / error | unsupported_entries に一致 |
| `ADAPTER_CAPABILITY_VERSION_MISMATCH` | warning / error | runtime または Adapter version 不一致 |
| `ADAPTER_CAPABILITY_FALLBACK_APPLIED` | info / warning | fallback 適用 |
| `ADAPTER_CAPABILITY_CONFLICT_WITH_REGISTRY` | warning / error | Registry と Capability の矛盾 |

---

## 18. conversion report連携

conversion report には Adapter Capability の照合結果を記録しなければならない。

少なくとも以下を記録する。

- 使用した Adapter Capability package
- 使用した Adapter ID
- 使用した Adapter version
- 対象 runtime
- 対象 runtime version
- Schema Registry との照合結果
- supported と判定した registry entry
- supported_with_fallback と判定した registry entry
- preserve_only と判定した registry entry
- source_raw と判定した registry entry
- unsupported と判定した registry entry
- error と判定した registry entry
- diagnostics summary

---

## 19. Capability package例

MuJoCo Adapter の Capability package 例を以下に示す。

```json
{
  "capability_id": "capability-sansa-vrm-mujoco-adapter-0.1.0",
  "capability_version": "0.1.0",
  "adapter_id": "sansa-vrm-mujoco-adapter",
  "adapter_version": "0.1.0",
  "runtime": "mujoco",
  "runtime_version_range": {
    "min": "2.3.0",
    "max": null
  },
  "supported_namespaces": [
    "mujoco",
    "sansa"
  ],
  "supported_targets": [
    "model",
    "module",
    "connection",
    "joint",
    "collider",
    "actuator",
    "sensor",
    "property"
  ],
  "supported_io_scopes": [
    "mjcf",
    "adapter_artifact",
    "both",
    "preserve_only",
    "source_raw"
  ],
  "supported_mappings": [
    {
      "element": "joint",
      "attribute": "armature",
      "direction": "import_export",
      "runtime_version": {
        "min": "2.3.0",
        "max": null
      },
      "value_conversion_types": [
        "identity"
      ]
    },
    {
      "element": "actuator",
      "attribute": "forcerange",
      "direction": "export",
      "runtime_version": {
        "min": "2.3.0",
        "max": null
      },
      "value_conversion_types": [
        "symmetric_range"
      ]
    }
  ],
  "supported_artifacts": [
    {
      "artifact_type": "controller_config",
      "direction": "export",
      "schema_version": "0.1.0",
      "path_patterns": [
        "actuators[]"
      ]
    },
    {
      "artifact_type": "conversion_report",
      "direction": "export",
      "schema_version": "0.1.0",
      "path_patterns": [
        "*"
      ]
    },
    {
      "artifact_type": "diagnostics",
      "direction": "export",
      "schema_version": "0.1.0",
      "path_patterns": [
        "*"
      ]
    }
  ],
  "supported_value_conversions": [
    "identity",
    "unit_scale",
    "symmetric_range",
    "enum_map"
  ],
  "unsupported_entries": [
    {
      "namespace": "mujoco",
      "name": "compiler_plugin",
      "target_type": "model",
      "reason": "MuJoCo plugin integration is not supported by this adapter version.",
      "fallback_hint": "source_raw"
    }
  ],
  "diagnostics_policy": {
    "unsupported_is_error_in_strict": true,
    "fallback_is_warning": true
  }
}
```

---

## 20. 実装責務分離

### 20.1 SansaVRM本体

SansaVRM 本体は以下を担当する。

- Adapter Capability の保持または参照
- Adapter Capability の構造検証
- Schema Registry との照合 API の提供
- Adapter から返却された diagnostics と conversion report の保持

### 20.2 Adapter

Adapter は以下を担当する。

- 自身の Adapter Capability を提供すること
- Capability に存在しない機能を暗黙に使用しないこと
- Schema Registry と Capability の照合結果に従うこと
- fallback、diagnostics、conversion report を生成すること

---

## 21. 検証要件

Adapter Capability に対するテスト仕様では、少なくとも以下を検証する。

- `adapter_id` と Schema Registry の `adapter_support` が照合されること
- `runtime` が不一致の場合に error となること
- `namespace` 未対応時に diagnostics が出力されること
- `target_type` 未対応時に diagnostics が出力されること
- `io_scope` 未対応時に diagnostics が出力されること
- `unsupported_entries` に一致する entry が未対応として扱われること
- `mjcf_mapping` 未対応時に fallback が評価されること
- `adapter_artifact` 未対応時に fallback が評価されること
- `value_conversion` 未対応時に fallback が評価されること
- strict モードで `error` がある場合、成果物出力が禁止されること
- conversion report に Adapter Capability 照合結果が記録されること

---

[目次](../../目次.md) > 仕様 > 共通 > Adapter Capability仕様
