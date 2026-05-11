<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-4102-adapter-input-json-specification
lang: ja-JP
canonical_title: Adapter Input JSON Specification
document_type: spec
canonical_document: true
-->

[目次](../目次.md) > 仕様 > Adapter Contract Layer > Adapter Input JSON Specification

# Adapter Input JSON Specification

## 1. 目的

本仕様は、SansaVRM Rust モジュールと Adapter の境界として使用する Adapter 入力用 JSON を定義する。

本仕様の主対象ファイルは以下である。

```text
sansavrm_adapter_input.json
```

本仕様により、Adapter は SansaVRM Rust 内部構造へ直接依存せず、安定した中間 JSON を入力として処理できる。

---

## 2. 基本方針

Adapter Input JSON は以下を原則とする。

```text
1. Rust 内部構造を直接公開しない
2. Adapter が必要とする情報を安定 JSON として出力する
3. Core 標準構造と Extension Property を分離して保持する
4. MuJoCo / Meridian / sysid / HIL-SIL 固有情報を Core へ混入させない
5. Adapter は Extension Property を分類して出力先を決定する
6. Adapter からの更新は updated_extension_properties.json として返却する
```

---

## 3. 対象 Adapter

本仕様は少なくとも以下を対象とする。

```text
sansavrm_mujoco_adapter
meridian_mujoco_runtime
nisocon_vr_battle_runtime
future runtime adapters
```

---

## 4. 出力コマンド

SansaVRM Rust モジュールは、Adapter 入力用 JSON を出力する機能を提供する。

想定コマンド：

```powershell
sansavrm export-adapter-input `
  --input robot.sansavrm `
  --adapter sansavrm_mujoco_adapter `
  --output sansavrm_adapter_input.json
```

---

## 5. 取り込みコマンド

Adapter が返却した Extension Property 更新情報は、SansaVRM へ再格納できなければならない。

対象ファイル：

```text
updated_extension_properties.json
```

想定コマンド：

```powershell
sansavrm import-extension-properties `
  --input robot.sansavrm `
  --extension-properties output/updated_extension_properties.json `
  --output robot.updated.sansavrm
```

---

## 6. Adapter Input JSON Root

Adapter Input JSON は最低限以下を持つ。

```text
schema_version
adapter_id
model_metadata
modules
connections
slots
properties
joints
colliders
actuators
sensors
extension_properties
extension_property_schemas
custom_parameters
custom_parameter_schemas
conversion_profile_refs
diagnostics_refs
```

---

## 7. model_metadata

model_metadata は SansaVRM model の基本情報を保持する。

最低限以下を持つ。

```text
model_id
name
version
source_format
source_path
created_at
updated_at
```

---

## 8. Core Structure Payload

Adapter Input JSON は Core 標準構造として以下を含める。

```text
modules
connections
slots
properties
```

これらは SansaVRM Core の安定 API から出力する。

---

## 9. Runtime Data Payload

Adapter Input JSON は runtime / physics / control 連携用に以下を含める。

```text
joints
colliders
actuators
sensors
```

これらは Core 標準構造、Data Model Layer、Runtime Integration Layer から Adapter 用に正規化して出力する。

---

## 10. Extension Properties

extension_properties は Adapter が分類・変換・出力する拡張情報である。

MuJoCo / Meridian / sysid / HIL-SIL 固有情報は Core 標準仕様へ直接組み込まず、Extension Property として扱う。

---

## 11. Extension Property 固定項目

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

## 12. Extension Property 分類必須項目

Adapter が分類に使用する必須項目は以下である。

```text
namespace
target_type
target_id
property_role
io_scope
adapter_scope
normalized_value
schema_ref
```

これらが欠落している場合、Adapter は diagnostics に記録し、strict mode では処理を中断できる。

---

## 13. io_scope

io_scope は Extension Property の出力先または保持方針を示す。

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

意味：

| 値 | 意味 |
|---|---|
| mjcf | MJCF へ出力する |
| adapter_artifact | controller_config 等の Adapter 側補助成果物へ出力する |
| runtime_artifact | runtime_requirements 等の Runtime 側成果物へ出力する |
| both | MJCF と補助成果物の両方へ出力する |
| preserve_only | SansaVRM 内に保持のみ |
| unsupported | 登録済みだが現在未対応 |
| source_raw | 解釈せず元情報として保持 |

---

## 14. adapter_scope

adapter_scope は Extension Property を処理する責務主体を示す。

許可値：

```text
sansavrm_mujoco_adapter
meridian_mujoco_runtime
nisocon_vr_battle_runtime
preserve_only
unknown
```

意味：

| 値 | 意味 |
|---|---|
| sansavrm_mujoco_adapter | Adapter が直接処理する |
| meridian_mujoco_runtime | meridian_mujoco_runtime 用 runtime_requirements へ分離する |
| nisocon_vr_battle_runtime | Adapter では直接処理せず、対象 runtime 用に保持する |
| preserve_only | SansaVRM 内に保持する |
| unknown | 処理主体が未確定 |

---

## 15. extension_property_schemas

extension_property_schemas は extension_properties の検証・分類に使用する schema 群である。

最低限以下を持つ。

```text
schema_id
namespace
property_role
target_type
value_type
io_scope
adapter_scope
required
fallback_policy
```

---

## 16. custom_parameters との関係

custom_parameters は Extension Property の一種として扱う。

ただし、互換性および既存 validator 連携のため、Adapter Input JSON では以下を許可する。

```text
extension_properties
custom_parameters
```

custom_parameters を別配列として出力する場合でも、各 custom_parameter は extension_properties へ正規化可能でなければならない。

---

## 17. custom_parameter_schemas

custom_parameter_schemas は Custom Parameter Registry Layer に基づく schema 情報である。

最低限以下を持つ。

```text
schema_id
namespace
parameter_name
target_type
value_type
io_scope
adapter_scope
mapping_ref
fallback_policy
```

---

## 18. updated_extension_properties.json

Adapter は変換後、必要に応じて updated_extension_properties.json を出力する。

このファイルは、Adapter または Runtime 側で更新・補完・分類された Extension Property を SansaVRM へ戻すために使用する。

---

## 19. updated_extension_properties Root

updated_extension_properties.json は最低限以下を持つ。

```text
schema_version
adapter_id
source_model_id
updated_extension_properties
diagnostics
conversion_report_refs
```

---

## 20. updated_extension_properties の取り込み方針

SansaVRM は updated_extension_properties を読み込み、既存 Extension Property と統合できなければならない。

統合方針：

```text
- extension_property_id が一致する場合は更新
- extension_property_id が存在しない場合は追加
- schema_ref が存在する場合は schema validation を行う
- diagnostics_ref が存在する場合は diagnostics と接続する
- conversion_report_ref が存在する場合は conversion report と接続する
```

---

## 21. Core と Extension Property の境界

以下は Core 標準仕様へ直接入れない。

```text
MuJoCo 固有情報
Meridian 固有情報
sysid 固有情報
HIL-SIL 固有情報
nisocon-vr-battle-runtime 固有情報
```

これらは Extension Property として保持する。

---

## 22. Adapter 側停止解除条件

SansaVRM-MuJoCo-Adapter 側の一時停止解除には、最低限以下が必要である。

```text
1. sansavrm_adapter_input.json の必須項目が確定している
2. extension_property 固定項目が確定している
3. io_scope enum が確定している
4. adapter_scope enum が確定している
5. updated_extension_properties.json の取り込み方針が確定している
6. Core と Extension Property の境界が明文化されている
```

---

## 23. Validator Requirements

Validator は Adapter Input JSON に対して以下を検査する。

```text
- required field presence
- extension_property schema validity
- io_scope validity
- adapter_scope validity
- target reference integrity
- schema_ref integrity
- diagnostics_ref integrity
- conversion_report_ref integrity
```

---

## 24. 関連仕様

本仕様は以下と連携する。

```text
Adapter Contract Specification
Custom Parameter Registry Specification
Format Compatibility Preservation Specification
Conversion Profile Specification
Physics Extension Specification
物理・制御メタモデル仕様
```

---

## 25. 結論

Adapter Input JSON は、SansaVRM Rust モジュールと Adapter の安定境界である。

この境界を固定することで、Adapter は SansaVRM 内部構造に依存せず、Extension Property を通じて MuJoCo / Meridian / sysid / HIL-SIL / nisocon-vr-battle-runtime 固有情報を扱える。

---

[目次](../目次.md) > 仕様 > Adapter Contract Layer > Adapter Input JSON Specification
