<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-4002-mujoco-runtime-integration-specification
lang: ja-JP
canonical_title: MuJoCo Runtime Integration Specification
document_type: spec
canonical_document: true
-->

[目次](../目次.md) > 仕様 > Runtime Integration Layer > MuJoCo Runtime Integration Specification

# MuJoCo Runtime Integration Specification

## 1. 目的

本仕様は、SansaVRM と MuJoCo runtime / MJCF / MuJoCo Adapter の連携における runtime-specific semantic を定義する。

本仕様では、Adapter Contract、Adapter Input JSON、Extension Property、Custom Parameter Registry、Adapter Artifact の generic 仕様を再定義しない。

本仕様の対象は以下である。

```text
- MuJoCo runtime mapping
- MJCF emission policy
- actuator binding
- sensor binding
- sysid integration
- HIL/SIL integration
- Meridian runtime integration
- MuJoCo-specific diagnostics
```

---

## 2. 基本方針

MuJoCo 連携は、SansaVRM の標準ファイル変換ではなく runtime integration として扱う。

以下を原則とする。

```text
1. SansaVRM Core は MuJoCo runtime に依存しない
2. MuJoCo 固有情報は Extension Property として保持する
3. Adapter は Adapter Input JSON を入力とする
4. MJCF 出力可否は Extension Property Schema / Custom Parameter Registry に従う
5. runtime_requirements は Adapter Artifact として出力する
6. sysid / HIL-SIL 結果は updated_extension_properties として再取り込み可能にする
```

---

## 3. 非対象範囲

以下は本仕様の直接対象外とする。

```text
- Adapter Input JSON root 仕様
- Extension Property payload 共通仕様
- Extension Property Schema 共通仕様
- Adapter Artifact 共通仕様
- Custom Parameter Registry 共通仕様
- MuJoCo solver 実装
- MuJoCo 実行ランタイムそのもの
```

---

## 4. MuJoCo Mapping Scope

MuJoCo runtime integration は以下を扱う。

```text
body
joint
geom
actuator
sensor
site
tendon
constraint
runtime requirement
```

---

## 5. SansaVRM → MuJoCo Mapping

最低限の対応は以下とする。

| SansaVRM | MuJoCo / MJCF |
|---|---|
| module | body |
| connection(joint) | joint |
| collider / geometry | geom |
| actuator | actuator |
| sensor | sensor |
| slot | site または adapter-side interface |
| extension_property | MJCF / adapter_artifact / runtime_artifact |

---

## 6. MuJoCo → SansaVRM Mapping

MuJoCo から SansaVRM へ取り込む場合、以下を行う。

```text
- body を module 候補として取り込む
- joint を connection または physics joint として取り込む
- geom を collider / geometry として取り込む
- actuator を actuator payload として取り込む
- sensor を sensor payload として取り込む
- MuJoCo 固有属性を Extension Property として保持する
- 未解釈情報を source_raw として保持する
```

---

## 7. MJCF Emission Policy

MJCF へ直接出力できる情報は、以下に基づいて判定する。

```text
- extension_property.io_scope = mjcf または both
- schema_ref が有効である
- mapping_ref が MJCF mapping を示す
- adapter_scope が sansavrm_mujoco_adapter または preserve_only 以外の矛盾を持たない
```

MJCF へ直接出力できない情報は、以下へ分離する。

```text
- adapter_artifact
- runtime_artifact
- preserve_only
- unsupported diagnostics
- source_raw
```

---

## 8. Actuator Binding

MuJoCo actuator binding は以下を扱う。

```text
- actuator target joint
- actuator type
- control range
- force / torque range
- command delay
- control mode
- runtime controller binding
```

MuJoCo 固有 actuator parameter は Extension Property として保持する。

---

## 9. Sensor Binding

MuJoCo sensor binding は以下を扱う。

```text
- sensor target
- sensor type
- sensor output unit
- runtime sampling requirement
- sensor routing
```

Runtime 側で必要な sampling / routing 情報は runtime_artifact へ分離する。

---

## 10. Runtime Requirements

MuJoCo Adapter は runtime_requirements を Adapter Artifact として出力できる。

対象例：

```text
- required actuator interface
- required sensor interface
- control loop frequency
- sysid data requirement
- HIL/SIL requirement
- Meridian protocol requirement
```

---

## 11. SysID Integration

sysid integration では、MuJoCo runtime または Meridian runtime で得られた識別結果を Extension Property として再格納できる。

対象例：

```text
- identified inertia
- damping
- friction
- actuator delay
- torque constant
- sensor noise model
```

出力は `updated_extension_properties.json` を使用する。

---

## 12. HIL / SIL Integration

HIL / SIL 連携では、Runtime 側に必要な情報を Extension Property または runtime_artifact として扱う。

対象例：

```text
- hardware interface requirement
- simulated sensor routing
- real actuator binding
- safety limit
- emergency stop requirement
```

---

## 13. Meridian Runtime Integration

Meridian runtime 連携では、Adapter が直接処理しない情報を `adapter_scope = meridian_mujoco_runtime` として分類する。

これらは `runtime_artifact` または `runtime_requirements` に分離する。

---

## 14. nisocon-vr-battle-runtime Integration

nisocon-vr-battle-runtime 向け情報は、Adapter が直接処理しない場合がある。

その場合は以下を使用する。

```text
adapter_scope = nisocon_vr_battle_runtime
io_scope = runtime_artifact または preserve_only
```

---

## 15. Diagnostics Requirements

MuJoCo runtime integration diagnostics は以下を含める。

```text
- MJCF出力不可理由
- actuator binding failure
- sensor binding failure
- unsupported MuJoCo parameter
- runtime_artifact 分離結果
- sysid 取り込み結果
- HIL/SIL 要件不整合
```

---

## 16. Conversion Report Requirements

MuJoCo Adapter は以下を conversion_report に記録する。

```text
- MJCF emission result
- adapter_artifact export result
- runtime_artifact export result
- preserve_only count
- unsupported count
- fallback result
- updated_extension_properties output
```

---

## 17. Validator Requirements

Validator は以下を検査する。

```text
- body / joint / geom mapping integrity
- actuator target integrity
- sensor target integrity
- Extension Property schema_ref integrity
- io_scope / adapter_scope consistency
- required runtime_artifact existence
- updated_extension_properties schema validity
```

---

## 18. Adapter側一時停止解除との関係

SansaVRM-MuJoCo-Adapter 側の一時停止解除には、本仕様だけでなく以下が確定している必要がある。

```text
- Adapter Input JSON Specification
- Extension Property Payload Specification
- Extension Property Schema Specification
- Adapter Artifact Specification
```

---

## 19. 関連仕様

本仕様は以下と連携する。

```text
物理・制御メタモデル仕様
Adapter Contract Specification
Adapter Input JSON Specification
Extension Property Payload Specification
Extension Property Schema Specification
Adapter Artifact Specification
Custom Parameter Registry Specification
Physics Extension Specification
Conversion Profile Specification
```

---

## 20. 結論

MuJoCo Runtime Integration は、SansaVRM の runtime-specific integration の一つである。

MuJoCo 固有情報は Core Semantic へ直接入れず、Extension Property / Adapter Artifact / Runtime Artifact として扱う。

これにより、SansaVRM Core を runtime 非依存に保ちながら、MuJoCo、Meridian、sysid、HIL/SIL、nisocon-vr-battle-runtime との連携を可能にする。

---

[目次](../目次.md) > 仕様 > Runtime Integration Layer > MuJoCo Runtime Integration Specification
