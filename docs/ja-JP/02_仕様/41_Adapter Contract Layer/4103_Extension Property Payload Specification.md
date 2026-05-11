<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-4103-extension-property-payload-specification
lang: ja-JP
canonical_title: Extension Property Payload Specification
document_type: spec
canonical_document: true
-->

[目次](../目次.md) > 仕様 > Adapter Contract Layer > Extension Property Payload Specification

# Extension Property Payload Specification

## 1. 目的

本仕様は、SansaVRM Adapter Input JSON に含まれる `extension_properties` の payload 仕様を定義する。

本仕様は以下を固定する。

```text
- extension_property_id
- namespace
- target_type
- target_id
- property_role
- io_scope
- adapter_scope
- source_format
- source_raw
- normalized_value
- schema_ref
- diagnostics_ref
- conversion_report_ref
```

---

## 2. 基本方針

Extension Property は、SansaVRM Core に直接入れない runtime / format / adapter 固有情報を保持するための payload である。

以下を原則とする。

```text
1. Core Semantic を runtime 固有情報で汚染しない
2. Adapter が分類可能な固定項目を持つ
3. schema_ref により検証可能にする
4. normalized_value と source_raw を分離する
5. diagnostics / conversion_report と接続可能にする
6. updated_extension_properties.json により再取り込み可能にする
```

---

## 3. Extension Property Object

extension_property は最低限以下を持つ。

```text
extension_property_id
namespace
target_type
target_id
property_role
io_scope
adapter_scope
source_format
source_raw
normalized_value
schema_ref
diagnostics_ref
conversion_report_ref
```

---

## 4. extension_property_id

`extension_property_id` は Extension Property の一意識別子である。

要求：

```text
- Adapter Input JSON 内で一意である
- updated_extension_properties.json で更新対象を識別できる
- 同一 property を更新する場合は同じ ID を使用する
```

---

## 5. namespace

`namespace` は property の由来または責務範囲を示す。

例：

```text
mujoco
meridian
sysid
hil_sil
nisocon_vr_battle
vendor
experimental
```

Namespace は Core Semantic ではなく、Extension Property の分類に使用する。

---

## 6. target_type / target_id

`target_type` は Extension Property の付与対象種別である。

許可候補：

```text
model
module
connection
slot
property
joint
collider
actuator
sensor
runtime
adapter_artifact
```

`target_id` は対象要素の ID を示す。

---

## 7. property_role

`property_role` は property の意味上の役割を示す。

許可候補：

```text
physics
control
sensor
actuator
runtime_requirement
sysid_parameter
hil_sil_parameter
adapter_parameter
conversion_hint
preservation_metadata
```

---

## 8. io_scope

`io_scope` は Extension Property の出力先または保持方針を示す。

許可値：

```text
mjcf
adapter_artifact
runtime_artifact
both
preserve_only
unsupported
source_raw
```

---

## 9. adapter_scope

`adapter_scope` は Extension Property の処理主体を示す。

許可値：

```text
sansavrm_mujoco_adapter
meridian_mujoco_runtime
nisocon_vr_battle_runtime
preserve_only
unknown
```

---

## 10. source_format

`source_format` は property の由来となる外部形式または runtime を示す。

例：

```text
mujoco
urdf
fbx
mmd
vrm
meridian
manual
unknown
```

---

## 11. source_raw

`source_raw` は元形式由来の未正規化情報を保持する。

用途：

```text
- 元形式への RoundTrip
- Adapter 側再解釈
- diagnostics
- preservation_only
```

`source_raw` は、Adapter が理解できない場合でも破棄してはならない。

---

## 12. normalized_value

`normalized_value` は Adapter が分類・変換に使用する正規化済み値である。

要求：

```text
- schema_ref により型検証できる
- source_raw と矛盾しない
- 単位変換済みの場合は schema 側で単位を明示する
- Adapter は原則 normalized_value を処理入力とする
```

---

## 13. schema_ref

`schema_ref` は Extension Property Schema への参照である。

用途：

```text
- value type validation
- io_scope validation
- adapter_scope validation
- fallback policy lookup
- mapping lookup
```

---

## 14. diagnostics_ref

`diagnostics_ref` は Extension Property に関連する diagnostics への参照である。

用途：

```text
- unsupported reason
- approximation reason
- fallback reason
- validation warning
- validation error
```

---

## 15. conversion_report_ref

`conversion_report_ref` は変換レポートへの参照である。

用途：

```text
- loss_report
- approximation report
- artifact export report
- runtime artifact report
```

---

## 16. Classification Rule

Adapter は以下の順に Extension Property を分類する。

```text
1. schema_ref
2. io_scope
3. adapter_scope
4. property_role
5. namespace
6. target_type / target_id
```

`schema_ref` が存在しない場合は diagnostics に記録する。

---

## 17. Output Routing

Adapter は io_scope と adapter_scope により出力先を決定する。

| io_scope | 出力先 |
|---|---|
| mjcf | MJCF |
| adapter_artifact | Adapter artifact |
| runtime_artifact | Runtime artifact |
| both | MJCF と補助成果物 |
| preserve_only | 出力せず保持 |
| unsupported | diagnostics |
| source_raw | source_raw として保持 |

---

## 18. updated_extension_properties 対応

Adapter は処理後に Extension Property を更新できる。

更新例：

```text
- normalized_value の補完
- diagnostics_ref の追加
- conversion_report_ref の追加
- adapter_scope の確定
- io_scope の fallback 結果記録
```

---

## 19. Core 境界

以下は Core 標準仕様へ直接昇格しない。

```text
MuJoCo 固有 actuator metadata
Meridian runtime requirement
sysid calibration parameter
HIL-SIL runtime parameter
nisocon-vr-battle-runtime metadata
```

これらは Extension Property として扱う。

---

## 20. Validator Requirements

Validator は以下を検査する。

```text
- extension_property_id uniqueness
- required field presence
- namespace validity
- target reference integrity
- io_scope validity
- adapter_scope validity
- schema_ref integrity
- normalized_value schema validity
- diagnostics_ref integrity
- conversion_report_ref integrity
```

---

## 21. 関連仕様

本仕様は以下と連携する。

```text
Adapter Input JSON Specification
Adapter Contract Specification
Custom Parameter Registry Specification
Format Compatibility Preservation Specification
Conversion Profile Specification
```

---

## 22. 結論

Extension Property Payload は、SansaVRM Core と runtime / format / adapter 固有情報の境界を維持するための中核 payload である。

Adapter は Extension Property を固定項目に基づいて分類し、MJCF、adapter artifact、runtime artifact、preserve_only、unsupported、source_raw へ振り分ける。

---

[目次](../目次.md) > 仕様 > Adapter Contract Layer > Extension Property Payload Specification
