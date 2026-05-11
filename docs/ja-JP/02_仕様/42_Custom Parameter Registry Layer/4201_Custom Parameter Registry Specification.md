<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-4201-custom-parameter-registry-specification
lang: ja-JP
canonical_title: Custom Parameter Registry Specification
document_type: spec
canonical_document: true
-->

[目次](../目次.md) > 仕様 > Custom Parameter Registry Layer > Custom Parameter Registry Specification

# Custom Parameter Registry Specification

## 1. 目的

本仕様は、SansaVRM における runtime / format 固有 parameter の登録・保持・参照方法を定義する。

本仕様は以下を対象とする。

```text
- MuJoCo parameter
- URDF parameter
- FBX metadata
- MMD metadata
- Unity runtime parameter
- O3DE runtime parameter
- proprietary runtime parameter
```

---

## 2. 基本方針

Custom Parameter Registry は generic layer として扱う。

以下を原則とする。

```text
1. runtime-specific parameter を Core Semantic に持ち込まない
2. namespace により parameter を分離する
3. io_scope を保持する
4. mapping semantic を保持する
5. adapter artifact を参照可能にする
6. fallback / unsupported を記録可能にする
```

---

## 3. Registry Entry

Registry Entry は parameter 定義単位である。

最低限以下を持つ。

```text
registry_id
namespace
parameter_name
parameter_type
io_scope
mapping_ref
fallback_policy
source_raw
```

---

## 4. Namespace

namespace は runtime / format を識別する。

例：

```text
sansavrm.mujoco.*
sansavrm.urdf.*
sansavrm.fbx.*
sansavrm.mmd.*
vendor.*
```

---

## 5. io_scope

io_scope は parameter の適用範囲を示す。

最低限以下を許可する。

```text
import
export
runtime
validation
preservation_only
custom
```

---

## 6. Mapping Reference

mapping_ref は Conversion Profile または runtime mapping と接続する。

対象例：

```text
coordinate mapping
physics mapping
material mapping
runtime binding
```

---

## 7. Adapter Artifact Reference

Registry は adapter artifact を参照できる。

対象例：

```text
runtime cache
baked parameter
optimization artifact
binary blob
```

---

## 8. Fallback Policy

Fallback Policy は unsupported 時の処理を示す。

最低限以下を許可する。

```text
ignore
warning
approximation
preserve_only
abort
custom
```

---

## 9. Preservation Requirements

Registry は以下を満たす。

```text
- unknown parameter を破棄しない
- source_raw を保持する
- raw_binary_ref を参照できる
- loss_report と接続できる
```

---

## 10. Validator Requirements

Validator は以下を検査する。

```text
- namespace validity
- parameter type validity
- io_scope validity
- mapping reference integrity
- fallback policy validity
```

---

## 11. 関連仕様

本仕様は以下と連携する。

```text
Adapter Contract Specification
Runtime Integration Layer
Format Compatibility Preservation Specification
Conversion Profile Specification
```

---

## 12. 結論

Custom Parameter Registry は、runtime / format 固有 parameter を generic に保持するための cross-runtime layer である。

これにより、MuJoCo 固有として扱われていた io_scope / mapping / artifact / fallback semantic を、SansaVRM 全体で再利用可能にする。

---

[目次](../目次.md) > 仕様 > Custom Parameter Registry Layer > Custom Parameter Registry Specification
