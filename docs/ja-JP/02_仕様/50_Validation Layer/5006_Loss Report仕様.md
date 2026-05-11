<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-5006-loss-report-specification
lang: ja-JP
canonical_title: Loss Report仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Validation Layer > Loss Report仕様

# Loss Report仕様

## 1. 目的

本仕様は、SansaVRM における Loss Report の構造、意味、生成責務を定義する。

Loss Report は、変換、RoundTrip、migration において発生した semantic loss、approximation、preservation_only を明示的に記録するために使用する。

本仕様の目的は以下とする。

```text
- silent loss の防止
- semantic loss の可視化
- approximation の記録
- preservation_only の明示
- RoundTrip quality の評価
- CI による loss gating
- validator / adapter / migration 間の統一表現
```

---

## 2. 基本方針

Loss Report は「問題通知」ではなく、「意味損失記録」である。

以下を原則とする。

```text
1. semantic loss を必ず記録する
2. approximation を必ず記録する
3. unsupported を記録する
4. preservation_only を記録する
5. runtime-specific loss を記録する
6. migration loss を記録する
7. diagnostics と分離する
```

---

## 3. Loss Item

Loss Item は単一の意味損失を表す。

最低限以下を持つ。

```text
loss_id
loss_type
severity
source_format
target_format
semantic_scope
message
path
related_ref
preservation_status
```

---

## 4. loss_type

loss_type は以下を許可する。

```text
semantic_loss
approximation
unsupported
preservation_only
runtime_loss
migration_loss
roundtrip_loss
compatibility_loss
```

---

## 5. severity

severity は以下を許可する。

```text
minor
moderate
major
critical
```

---

## 6. semantic_scope

semantic_scope は損失対象の意味範囲を表す。

例：

```text
geometry
rig
skinning
morph
animation
physics
runtime
rights
metadata
connection
traceability
```

---

## 7. preservation_status

preservation_status は以下を許可する。

```text
preserved
preserved_raw_only
approximated
lost
unknown
```

---

## 8. source_format / target_format

Loss Report は変換元・変換先を記録できなければならない。

例：

```text
VRM0
VRM1
FBX
MMD
MuJoCo
URDF
SansaVRM
```

---

## 9. related_ref

related_ref は関連仕様、traceability、source_raw を参照する。

例：

```text
doc_id
sec_id
source_raw_ref
migration_entry_id
adapter_property_ref
```

---

## 10. Diagnostics との関係

Diagnostics と Loss Report は責務が異なる。

```text
Diagnostics:
問題通知

Loss Report:
意味損失記録
```

ただし、critical loss は diagnostics と連携してよい。

---

## 11. Approximation

Approximation は「意味完全一致ではないが近似可能」であることを示す。

例：

```text
- FBX tangent → VRM interpolation approximation
- MMD spring → generic spring approximation
- runtime parameter normalization
```

Approximation は必ず Loss Report に記録する。

---

## 12. preservation_only

preservation_only は、SansaVRM semantic へ統合されず raw 保持のみ行われた状態を示す。

例：

```text
- proprietary runtime parameter
- unknown FBX chunk
- unsupported MMD extension
- experimental MuJoCo parameter
```

---

## 13. RoundTrip Loss

RoundTrip Loss は往復変換時の semantic divergence を示す。

例：

```text
VRM → SansaVRM → VRM
FBX → SansaVRM → FBX
MMD → SansaVRM → MMD
```

RoundTrip Loss は以下を記録できなければならない。

```text
- semantic divergence
- approximation accumulation
- runtime semantic mismatch
- identifier instability
```

---

## 14. Migration Loss

Migration Loss は relocation / split / merge 時の traceability divergence を示す。

例：

```text
- sec_id continuity loss
- semantic split ambiguity
- orphan mapping
- unresolved merge
```

---

## 15. Validator Requirements

Validator は以下を Loss Report に出力できなければならない。

```text
- semantic contradiction
- unsupported semantic
- unresolved semantic mapping
- approximation requirement
```

---

## 16. Adapter Requirements

Adapter は以下を Loss Report に出力できなければならない。

```text
- unsupported export
- unsupported import
- runtime parameter loss
- interpolation approximation
- coordinate normalization
- unit conversion approximation
```

---

## 17. Migration Requirements

Migration validator は以下を Loss Report に出力できなければならない。

```text
- sec_id continuity loss
- semantic equivalence unknown
- placeholder relocation remaining
- staged relocation incomplete
```

---

## 18. CI Requirements

CI は Loss Report を gating 条件として利用できる。

例：

```text
- critical loss が存在する場合 fail
- unresolved migration loss が存在する場合 fail
- roundtrip_loss が threshold 超過時 fail
```

---

## 19. 出力例

```json
{
  "loss_id": "loss-0001",
  "loss_type": "approximation",
  "severity": "moderate",
  "source_format": "FBX",
  "target_format": "VRM1",
  "semantic_scope": "animation",
  "message": "FBX tangent interpolation was approximated.",
  "path": "/animations/0/channels/2",
  "related_ref": {
    "doc_id": "dry-doc-3003-animation-extension-specification",
    "sec_id": null
  },
  "preservation_status": "approximated"
}
```

---

## 20. 関連仕様

本仕様は以下と連携する。

```text
Diagnostics仕様
変換仕様
RoundTrip Semantic Criteria
Traceability Migration仕様
Format Compatibility Preservation Specification
```

---

## 21. 結論

Loss Report は、SansaVRM における semantic preservation observability の中核である。

Loss Report により、semantic loss、approximation、preservation_only、migration divergence を明示的に記録し、silent loss を防止する。

---

[目次](../../目次.md) > 仕様 > Validation Layer > Loss Report仕様
