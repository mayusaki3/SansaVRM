<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-4202-extension-property-schema-specification
lang: ja-JP
canonical_title: Extension Property Schema Specification
document_type: spec
canonical_document: true
-->

[目次](../目次.md) > 仕様 > Custom Parameter Registry Layer > Extension Property Schema Specification

# Extension Property Schema Specification

## 1. 目的

本仕様は、SansaVRM Adapter Input JSON に含まれる `extension_property_schemas` の仕様を定義する。

`extension_property_schemas` は、`extension_properties` の検証、分類、出力先判定、fallback 判定に使用する。

---

## 2. 基本方針

Extension Property Schema は、runtime / format / adapter 固有情報を Core Semantic へ混入させず、schema により検証可能にするための定義である。

以下を原則とする。

```text
1. Extension Property は schema_ref により検証可能である
2. io_scope / adapter_scope は schema により制約される
3. value_type / unit / enum / range は schema により制約される
4. fallback_policy は schema に定義する
5. mapping_ref により Conversion Profile または runtime mapping と接続する
6. schema は Adapter と Runtime の双方で参照可能にする
```

---

## 3. Extension Property Schema Object

extension_property_schema は最低限以下を持つ。

```text
schema_id
namespace
property_role
target_type
value_type
unit
required
default_value
min
max
enum
io_scope
adapter_scope
mapping_ref
artifact_ref
fallback_policy
description
version
```

---

## 4. schema_id

`schema_id` は schema の一意識別子である。

要求：

```text
- Adapter Input JSON 内で一意である
- extension_properties[].schema_ref から参照可能である
- version 更新時も追跡可能である
```

---

## 5. namespace

`namespace` は schema の適用範囲を示す。

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

---

## 6. property_role

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

## 7. target_type

`target_type` は schema を適用可能な対象を示す。

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

---

## 8. value_type

`value_type` は normalized_value の型を示す。

許可候補：

```text
string
number
integer
boolean
array
object
null
custom
```

---

## 9. unit / range / enum

schema は値の制約を持てる。

対象：

```text
unit
min
max
enum
default_value
```

Adapter は normalized_value を処理する前に、これらの制約を検証する。

---

## 10. io_scope

`io_scope` は Extension Property の出力先または保持方針を制約する。

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

## 11. adapter_scope

`adapter_scope` は Extension Property の処理主体を制約する。

許可値：

```text
sansavrm_mujoco_adapter
meridian_mujoco_runtime
nisocon_vr_battle_runtime
preserve_only
unknown
```

---

## 12. mapping_ref

`mapping_ref` は Conversion Profile または runtime mapping への参照である。

用途：

```text
- MJCF mapping
- physics mapping
- actuator mapping
- sensor mapping
- runtime requirement mapping
```

---

## 13. artifact_ref

`artifact_ref` は Adapter Artifact または Runtime Artifact の出力先定義を参照する。

用途：

```text
- controller_config
- runtime_requirements
- diagnostics
- conversion_report
- external_metadata
```

---

## 14. fallback_policy

`fallback_policy` は、Adapter が property を直接処理できない場合の扱いを定義する。

許可候補：

```text
use_default
preserve_only
warn
error
ignore
abort
custom
```

---

## 15. version

schema は version を持つ。

最低限以下を保持する。

```text
schema_version
supported_since
deprecated_since
```

---

## 16. Custom Parameter Schema との関係

Custom Parameter Schema は Extension Property Schema の一種として扱える。

ただし、既存 validator / registry 連携のため、Adapter Input JSON では以下を分けて出力してよい。

```text
extension_property_schemas
custom_parameter_schemas
```

custom_parameter_schemas は extension_property_schemas へ正規化可能でなければならない。

---

## 17. Validator Requirements

Validator は以下を検査する。

```text
- schema_id uniqueness
- namespace validity
- property_role validity
- target_type validity
- value_type validity
- io_scope validity
- adapter_scope validity
- mapping_ref integrity
- artifact_ref integrity
- fallback_policy validity
- version compatibility
```

---

## 18. Adapter Requirements

Adapter は以下を満たす。

```text
- schema_ref を優先して extension_property を分類する
- schema が存在しない場合 diagnostics に記録する
- io_scope と adapter_scope の矛盾を検出する
- fallback_policy に従って処理する
- unsupported を silent loss しない
```

---

## 19. Runtime Requirements

Runtime は必要に応じて schema を参照できる。

対象例：

```text
- meridian_mujoco_runtime
- nisocon_vr_battle_runtime
- HIL/SIL runtime
```

Runtime は adapter_scope が自分に該当する property を処理できる。

---

## 20. 関連仕様

本仕様は以下と連携する。

```text
Adapter Input JSON Specification
Extension Property Payload Specification
Custom Parameter Registry Specification
Adapter Contract Specification
Conversion Profile Specification
```

---

## 21. 結論

Extension Property Schema は、Extension Property を安定して検証・分類・routing するための schema contract である。

これにより、MuJoCo / Meridian / sysid / HIL-SIL / nisocon-vr-battle-runtime 固有情報を Core Semantic へ混入させず、Adapter / Runtime が安全に処理できる。

---

[目次](../目次.md) > 仕様 > Custom Parameter Registry Layer > Extension Property Schema Specification
