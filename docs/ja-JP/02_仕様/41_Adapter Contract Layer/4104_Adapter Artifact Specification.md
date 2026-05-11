<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-4104-adapter-artifact-specification
lang: ja-JP
canonical_title: Adapter Artifact Specification
document_type: spec
canonical_document: true
-->

[目次](../目次.md) > 仕様 > Adapter Contract Layer > Adapter Artifact Specification

# Adapter Artifact Specification

## 1. 目的

本仕様は、SansaVRM Adapter が生成・参照する Adapter Artifact / Runtime Artifact の仕様を定義する。

本仕様は以下を対象とする。

```text
- controller_config
- runtime_requirements
- diagnostics
- conversion_report
- loss_report
- external_metadata
- runtime cache
- optimization artifact
```

---

## 2. 基本方針

Adapter Artifact は、SansaVRM Core に直接含めるべきではない runtime / adapter 固有成果物を外部化するための仕組みである。

以下を原則とする。

```text
1. Core Semantic を artifact 固有情報で汚染しない
2. Artifact は Extension Property から参照可能にする
3. Artifact 生成結果は diagnostics / conversion_report に記録する
4. Runtime 固有成果物は runtime_artifact として分離する
5. Adapter 固有成果物は adapter_artifact として分離する
6. Artifact は再生成可能性と保存必要性を明示する
```

---

## 3. Artifact 分類

Adapter Artifact は以下へ分類する。

| 分類 | 用途 |
|---|---|
| adapter_artifact | Adapter が直接使用・生成する補助成果物 |
| runtime_artifact | Runtime が使用する成果物 |
| diagnostics_artifact | 診断結果 |
| conversion_artifact | 変換レポート |
| preservation_artifact | 元形式復元用情報 |
| cache_artifact | 再生成可能な cache |

---

## 4. Artifact Object

artifact は最低限以下を持つ。

```text
artifact_id
artifact_type
artifact_scope
adapter_id
runtime_id
path
format
schema_ref
source_ref
generated_from
regeneration_policy
required_for_runtime
preservation_required
```

---

## 5. artifact_type

artifact_type は成果物の種類を示す。

許可候補：

```text
controller_config
runtime_requirements
diagnostics
conversion_report
loss_report
external_metadata
runtime_cache
optimization_data
binary_blob
custom
```

---

## 6. artifact_scope

artifact_scope は artifact の責務範囲を示す。

許可候補：

```text
adapter_artifact
runtime_artifact
diagnostics_artifact
conversion_artifact
preservation_artifact
cache_artifact
```

---

## 7. adapter_id / runtime_id

`adapter_id` は artifact を生成または利用する Adapter を示す。

例：

```text
sansavrm_mujoco_adapter
```

`runtime_id` は artifact を使用する Runtime を示す。

例：

```text
meridian_mujoco_runtime
nisocon_vr_battle_runtime
```

---

## 8. path / format / schema_ref

Artifact は保存先と形式を明示する。

最低限以下を持つ。

```text
path
format
schema_ref
```

`format` の例：

```text
json
yaml
xml
binary
text
custom
```

---

## 9. generated_from

`generated_from` は artifact の生成元を示す。

対象例：

```text
extension_property_id
conversion_profile_id
runtime_binding_id
source_raw
```

---

## 10. regeneration_policy

`regeneration_policy` は artifact の再生成可否を示す。

許可候補：

```text
regeneratable
preserve_required
manual_update_required
external_source_required
unknown
```

---

## 11. required_for_runtime

`required_for_runtime` は runtime 実行に必須かを示す。

```text
true
false
```

Runtime 実行に必須の場合、artifact 欠落は validator error として扱う。

---

## 12. preservation_required

`preservation_required` は RoundTrip または再出力のために artifact 保持が必要かを示す。

```text
true
false
```

---

## 13. controller_config

controller_config は Adapter または Runtime が制御に使用する設定である。

用途例：

```text
actuator command delay
control gain
torque limit
runtime controller binding
```

---

## 14. runtime_requirements

runtime_requirements は Runtime 側へ渡す要件定義である。

用途例：

```text
required sensors
required actuator interface
sysid requirement
HIL/SIL requirement
runtime capability requirement
```

---

## 15. diagnostics / conversion_report / loss_report

Adapter は diagnostics / conversion_report / loss_report を artifact として出力できる。

これらは Extension Property から参照可能でなければならない。

参照項目：

```text
diagnostics_ref
conversion_report_ref
loss_report_ref
```

---

## 16. Artifact Routing

Artifact Routing は Extension Property の `io_scope` と `adapter_scope` に基づいて決定する。

例：

| io_scope | artifact_scope |
|---|---|
| adapter_artifact | adapter_artifact |
| runtime_artifact | runtime_artifact |
| both | adapter_artifact + runtime_artifact または MJCF + artifact |
| preserve_only | preservation_artifact または SansaVRM 内保持 |
| unsupported | diagnostics_artifact |

---

## 17. updated_extension_properties との関係

Artifact の生成結果は updated_extension_properties.json に反映できる。

反映例：

```text
- artifact_ref の追加
- diagnostics_ref の追加
- conversion_report_ref の追加
- runtime_artifact path の追加
- fallback result の追加
```

---

## 18. Validator Requirements

Validator は以下を検査する。

```text
- artifact_id uniqueness
- artifact_type validity
- artifact_scope validity
- path validity
- schema_ref integrity
- generated_from integrity
- required_for_runtime artifact existence
- preservation_required artifact existence
```

---

## 19. Adapter Requirements

Adapter は以下を満たす。

```text
- artifact を生成した場合、conversion_report に記録する
- artifact 生成失敗時は diagnostics に記録する
- runtime 必須 artifact の欠落を silent loss しない
- updated_extension_properties に artifact_ref を返却できる
```

---

## 20. 関連仕様

本仕様は以下と連携する。

```text
Adapter Input JSON Specification
Extension Property Payload Specification
Extension Property Schema Specification
Adapter Contract Specification
Custom Parameter Registry Specification
```

---

## 21. 結論

Adapter Artifact は、runtime / adapter 固有成果物を SansaVRM Core から分離しつつ、Extension Property と接続して追跡可能にするための仕組みである。

これにより、controller_config、runtime_requirements、diagnostics、conversion_report 等を Core Semantic に混入させずに扱える。

---

[目次](../目次.md) > 仕様 > Adapter Contract Layer > Adapter Artifact Specification
