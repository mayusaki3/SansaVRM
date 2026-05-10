<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260510-000006Z-SV02
lang: ja-JP
canonical_title: Adapter Extension Property Specification
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > 共通 > Adapter Extension Property Specification

# Adapter Extension Property Specification

## 1. 目的

本仕様は、Adapter / Runtime / Hardware / System Identification 依存情報を、SansaVRM Core 標準へ直接組み込まず、SansaVRM 内へ拡張プロパティとして保存するための仕様を定義する。

本仕様は以下を対象とする。

```text
- MuJoCo Adapter
- Meridian Runtime
- HIL/SIL Runtime
- System Identification
- Device Capability
- Hardware Runtime
- Vendor Runtime
```

---

## 2. 基本方針

SansaVRM は以下を原則とする。

```text
1. Core Semantic は最小限に保つ
2. Runtime / Adapter 固有情報は Extension Property へ分離する
3. Unknown Property を破壊しない
4. Adapter 固有情報を passthrough 可能にする
5. Runtime 更新結果を再格納可能にする
6. Runtime 非依存な SansaVRM Core を維持する
```

---

## 3. Extension Property の定義

Extension Property とは、SansaVRM Core 標準 semantic に属さないが、Adapter / Runtime / Hardware / Runtime Bridge が利用する追加情報である。

### 例

```text
- device_category
- capability_profile
- actuator_model
- behavior_model
- sysid_result_ref
- runtime_requirements
- diagnostics_ref
- controller_config
```

---

## 4. Core Semantic と Extension Property の分離

## 4.1 Core Semantic 側

Core 側へ含める対象。

```text
- Module
- Connection
- Slot
- Property
- structure semantic
- transform semantic
- humanoid semantic
- expression semantic
- basic physics semantic
```

---

## 4.2 Extension Property 側

Extension Property 側へ含める対象。

```text
- device_category
- device_identity
- capability_profile
- command_semantics
- command_transport
- actuator_model
- behavior_model
- sysid_result_ref
- runtime_requirements
- HIL/SIL metadata
- Meridian runtime metadata
- diagnostics_ref
- conversion_report_ref
- source_raw
```

---

## 5. 想定ユースケース

## 5.1 MMD → VRM 利用

```text
MMD
↓
SansaVRM
↓
VRM0.x
```

この場合：

```text
MuJoCo / Meridian / Runtime 用拡張情報
```

は：

```text
SansaVRM内部へ保持
```

されるが、VRM export 時は出力不要。

---

## 5.2 MuJoCo Runtime 利用

```text
SansaVRM
↓ Adapter
MuJoCoデータ
↓ meridian-mujoco-runtime
sysid / runtime 更新
↓ Adapter
SansaVRM
```

この場合、Runtime 更新結果は：

```text
SansaVRM Extension Property
```

として再格納可能。

---

## 6. Extension Property Structure

## 6.1 基本構造

Extension Property は最低限以下を持つ。

| field | 内容 |
|---|---|
| namespace | Property namespace |
| property_id | Property identifier |
| target_type | 対象種別 |
| target_id | 対象ID |
| property_role | Property role |
| value_type | 値型 |
| normalized_value | 正規化済み値 |
| source_raw | 元データ |
| source_format | 元フォーマット |
| adapter_scope | 利用 Adapter |
| io_scope | import/export/runtime |
| diagnostics_ref | diagnostics 参照 |
| conversion_report_ref | report 参照 |

---

## 6.2 namespace

namespace は Property の責務範囲を示す。

### 例

```text
sansavrm.runtime
sansavrm.mujoco
sansavrm.sysid
sansavrm.device
vendor.xxx
```

---

## 6.3 target_type

対象種別。

### 例

```text
module
connection
joint
actuator
sensor
runtime
```

---

## 6.4 property_role

Property の役割。

### 例

```text
runtime_requirement
capability
sysid
controller_config
adapter_metadata
runtime_state
```

---

## 7. io_scope

io_scope は Property の利用範囲を示す。

| io_scope | 内容 |
|---|---|
| import | Import 時利用 |
| export | Export 時利用 |
| runtime | Runtime 時利用 |
| preserve_only | 保持のみ |

---

## 8. adapter_scope

adapter_scope は利用 Adapter を示す。

### 例

```text
mujoco
urdf
vrm0
vrm1
meridian
vendor.xxx
```

---

## 9. normalized_value と source_raw

## 9.1 normalized_value

SansaVRM が意味を理解した正規化値。

---

## 9.2 source_raw

元データそのもの。

### 目的

```text
- unknown property preservation
- runtime re-export
- vendor compatibility
- future migration
```

---

## 10. Runtime 更新

Runtime 側は以下を更新可能。

```text
- sysid_result_ref
- actuator parameter
- diagnostics_ref
- runtime state
- runtime capability
```

ただし：

```text
Core Semantic の破壊は禁止
```

---

## 11. diagnostics_ref

Runtime / Adapter は diagnostics を外部成果物として保持可能。

Extension Property は diagnostics を参照できる。

### 例

```text
runtime_diagnostics.json
adapter_diagnostics.json
```

---

## 12. conversion_report_ref

変換結果レポート参照。

### 目的

```text
- semantic loss tracking
- approximation tracking
- unsupported tracking
```

---

## 13. System Identification

## 13.1 sysid_result_ref

sysid 結果は原則として外部成果物参照とする。

### 理由

```text
- 実測依存
- 個体差依存
- firmware依存
- runtime依存
```

ため。

---

## 13.2 Runtime 更新

Runtime は sysid_result_ref を更新可能。

ただし：

```text
Core Semantic は変更しない
```

---

## 14. HIL/SIL

HIL/SIL 情報は Extension Property として保持する。

### 例

```text
hil_capable
sil_capable
requires_external_control_loop
runtime_bridge_profile
```

---

## 15. Unknown Extension Handling

Unknown Extension Property は以下を原則とする。

```text
- 破棄禁止
- passthrough 許可
- preserve_only 許可
- validator warning 許可
- validator error 原則禁止
```

---

## 16. Validator Requirements

Validator は以下を検証する。

```text
- namespace validity
- target validity
- reference integrity
- io_scope consistency
- adapter_scope consistency
- runtime contradiction
```

---

## 17. RoundTrip Requirements

Extension Property は以下を満たす。

```text
- passthrough preservation
- namespace stability
- reference stability
- runtime re-import stability
```

---

## 18. 非目的

本仕様は以下を目的としない。

```text
- Runtime engine standardization
- Hardware API standardization
- Control protocol standardization
- Real-time synchronization standardization
```

---

## 19. 結論

SansaVRM は Runtime / Adapter / Hardware 固有情報を Core Semantic へ直接組み込まない。

代わりに：

```text
Extension Property
```

として保存する。

これにより：

```text
- VRM利用
- MMD利用
- MuJoCo利用
- Meridian runtime利用
- HIL/SIL利用
```

を分離したまま共存可能にする。

また、Runtime 更新結果を SansaVRM 内へ再格納し、流通可能にする。

---

[目次](../../目次.md) > 仕様 > 共通 > Adapter Extension Property Specification
