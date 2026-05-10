<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-2001-adapter-extension-property-specification
lang: ja-JP
canonical_title: Adapter Extension Property Specification
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Preservation Compatibility Layer > Adapter Extension Property Specification

# Adapter Extension Property Specification

## 1. 目的

本ドキュメントは、Adapter 固有情報を SansaVRM Core から分離し、拡張プロパティとして保持するための仕様を定義する。

本仕様は以下を対象とする。

```text
- MuJoCo
- URDF
- FBX
- MMD
- VRM extension
- proprietary runtime
- future adapters
```

---

## 2. 基本方針

SansaVRM は以下を原則とする。

```text
1. Core Semantic を肥大化させない
2. Adapter 固有情報を削除しない
3. unknown property を破壊しない
4. passthrough を許可する
5. namespace 分離を行う
6. Runtime 固有情報を隔離する
```

---

## 3. Adapter Extension Property の定義

Adapter Extension Property とは以下を意味する。

```text
SansaVRM Core Semantic に属さないが、
特定 Adapter または Runtime に必要な情報
```

---

## 4. 保存方針

Adapter Extension Property は以下へ保存できる。

| 保存先 | 用途 |
|---|---|
| namespaced extension | structured semantic |
| passthrough | unknown data |
| preservation_only | Core 未理解データ |
| adapter artifact | runtime sidecar |

---

## 5. Namespace 要求

Adapter Extension Property は namespace を必須とする。

### 例

```text
sansavrm.mujoco.*
sansavrm.urdf.*
sansavrm.fbx.*
sansavrm.mmd.*
vendor.*
```

---

## 6. MuJoCo Property

## 6.1 対象

```text
- actuator
- sensor
- sysid
- runtime mapping
- solver parameter
- runtime control
```

## 6.2 Core責務

Core は semantic preservation のみを保証する。

## 6.3 Adapter責務

MuJoCo Adapter は runtime semantic を管理する。

---

## 7. FBX Property

## 7.1 対象

```text
- DCC metadata
- proprietary transform
- tangent semantic
- animation stack
- layer semantic
```

## 7.2 保存方針

```text
preservation_only
```

を許可する。

---

## 8. MMD Property

## 8.1 対象

```text
- behavior semantic
- motion semantic
- morph semantic
- rigidbody semantic
- toon semantic
```

## 8.2 保存方針

```text
approximation + preservation_only
```

を許可する。

---

## 9. VRM Extension

VRM extension は以下を許可する。

```text
- passthrough
- preservation_only
- vendor extension
```

unknown extension の破棄は禁止する。

---

## 10. Property Classification

Adapter Extension Property は以下へ分類する。

| classification | 意味 |
|---|---|
| semantic_extension | semantic を持つ |
| runtime_extension | runtime 専用 |
| preservation_only | 保持のみ |
| passthrough | 未理解 |
| proprietary | vendor 固有 |

---

## 11. Typed Property と Extension

SansaVRM は以下を分離する。

| 種類 | 意味 |
|---|---|
| Typed Property | Core Semantic |
| Extension Property | Adapter semantic |
| Passthrough | unknown semantic |

---

## 12. Adapter Artifact 分離

Runtime 依存情報は adapter artifact として外部化できる。

### 例

```text
MuJoCo runtime cache
physics bake data
runtime optimization data
```

---

## 13. Preservation 要求

Adapter Extension Property は以下を満たす。

```text
- Import 後に保持される
- Export 後に保持される
- RoundTrip で削除されない
- diagnostics で可視化可能
```

---

## 14. Diagnostics Requirements

diagnostics は以下を記録する。

```text
- extension namespace
- preservation state
- unsupported extension
- approximation reason
- passthrough count
```

---

## 15. Validator Requirements

Validator は以下を検査する。

```text
- namespace validity
- extension conflict
- invalid passthrough
- preservation inconsistency
```

---

## 16. Core との責務境界

## Core

```text
- semantic preservation
- traceability
- diagnostics
- conversion report
```

## Adapter

```text
- runtime implementation
- proprietary handling
- actual import/export
- runtime optimization
```

---

## 17. 今後追加される仕様

```text
- Format Compatibility Preservation Specification
- Conversion Profile Specification
- Runtime Mapping Specification
- Namespace Migration Policy
```

---

## 18. 結論

Adapter Extension Property は、Core Semantic を肥大化させずに、Format / Runtime 固有情報を保持するための仕組みである。

これにより：

```text
- MuJoCo
- URDF
- FBX
- MMD
- VRM extension
```

等の異なる semantic を、破壊せず統合できる。

---

[目次](../../目次.md) > 仕様 > Preservation Compatibility Layer > Adapter Extension Property Specification
