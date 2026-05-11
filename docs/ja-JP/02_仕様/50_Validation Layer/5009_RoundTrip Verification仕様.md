<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-5009-roundtrip-verification-specification
lang: ja-JP
canonical_title: RoundTrip Verification仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Validation Layer > RoundTrip Verification仕様

# RoundTrip Verification仕様

## 1. 目的

本仕様は、SansaVRM における RoundTrip semantic verification の定義および検証方法を定義する。

RoundTrip Verification は以下を目的とする。

```text
- semantic continuity verification
- preservation continuity verification
- approximation continuity verification
- runtime continuity verification
- migration continuity verification
- roundtrip difference analysis
- silent semantic loss detection
```

---

## 2. 基本方針

RoundTrip Verification は binary equality verification ではなく semantic continuity verification として扱う。

以下を原則とする。

```text
1. binary equality を必須としない
2. semantic continuity を重視する
3. preservation continuity を分析する
4. approximation continuity を分析する
5. runtime continuity を分析する
6. loss_report と連携する
7. compatibility analysis と連携する
```

---

## 3. RoundTrip Flow

RoundTrip Flow は以下を基本とする。

```text
source_format
↓
SansaVRM
↓
target_format
↓
SansaVRM
↓
verification
```

---

## 4. roundtrip_type

roundtrip_type は以下を許可する。

```text
same_format_roundtrip
cross_format_roundtrip
runtime_roundtrip
migration_roundtrip
partial_roundtrip
```

---

## 5. same_format_roundtrip

同一 format を用いた往復 verification。

例：

```text
VRM → SansaVRM → VRM
MMD → SansaVRM → MMD
FBX → SansaVRM → FBX
```

---

## 6. cross_format_roundtrip

異なる format を経由した往復 verification。

例：

```text
VRM → SansaVRM → FBX
FBX → SansaVRM → VRM
MMD → SansaVRM → VRM
```

---

## 7. runtime_roundtrip

runtime semantic を含む往復 verification。

例：

```text
MuJoCo → SansaVRM → MuJoCo
Unity Runtime → SansaVRM → Unity Runtime
```

---

## 8. migration_roundtrip

migration continuity を検証する往復 verification。

例：

```text
old spec
↓
new spec
↓
traceability reconstruction
```

---

## 9. partial_roundtrip

部分 semantic のみを対象とした verification。

例：

```text
expression only
physics only
runtime property only
```

---

## 10. roundtrip_level

roundtrip_level は以下を許可する。

```text
level_0_reexport
level_1_visual
level_2_behavior
level_3_runtime
level_4_semantic
```

---

## 11. level_0_reexport

再出力可能であることのみを保証する。

---

## 12. level_1_visual

視覚的 semantic continuity を保証する。

例：

```text
mesh
material
morph
```

---

## 13. level_2_behavior

挙動 semantic continuity を保証する。

例：

```text
animation
state transition
constraint behavior
```

---

## 14. level_3_runtime

runtime semantic continuity を保証する。

例：

```text
actuator semantic
sensor semantic
runtime binding
```

---

## 15. level_4_semantic

semantic equivalence を保証する。

完全 binary equality は要求しない。

---

## 16. semantic continuity

semantic continuity は以下を分析する。

```text
- structure continuity
- property continuity
- state continuity
- expression continuity
- physics continuity
- runtime continuity
```

---

## 17. preservation continuity

preservation continuity は preservation state の維持を分析する。

例：

```text
source_raw continuity
raw_binary continuity
adapter_artifact continuity
```

---

## 18. approximation continuity

approximation continuity は approximation semantic の変化を分析する。

例：

```text
tangent approximation
constraint approximation
physics approximation
```

---

## 19. runtime continuity

runtime continuity は runtime semantic の維持を分析する。

例：

```text
MuJoCo actuator semantic
simulation parameter semantic
runtime control semantic
```

---

## 20. migration continuity

migration continuity は relocation / migration 後の semantic continuity を分析する。

例：

```text
doc_id continuity
sec_id continuity
semantic_equivalent continuity
split relocation continuity
merge relocation continuity
```

---

## 21. roundtrip_difference

RoundTrip Verifier は semantic difference を検出する。

例：

```text
- unsupported semantic
- approximation increase
- preservation loss
- runtime semantic degradation
```

---

## 22. loss_report integration

RoundTrip Verification は Loss Report と連携する。

以下を分析する。

```text
- semantic_loss
- approximation
- unsupported_feature
- preservation_state
```

---

## 23. compatibility integration

RoundTrip Verification は Compatibility Analysis と連携する。

以下を分析する。

```text
- semantic compatibility
- runtime compatibility
- preservation compatibility
```

---

## 24. diagnostics integration

RoundTrip Verification は diagnostics と連携する。

例：

```text
ROUNDTRIP_SEMANTIC_LOSS
ROUNDTRIP_RUNTIME_DEGRADED
ROUNDTRIP_PRESERVATION_LOST
ROUNDTRIP_APPROXIMATION_INCREASED
```

---

## 25. coverage integration

RoundTrip Verification は Coverage Analysis と連携する。

以下を分析する。

```text
- unverified roundtrip semantic
- orphan roundtrip verification
- missing roundtrip coverage
```

---

## 26. CI Requirements

CI は roundtrip gate を設定可能とする。

例：

```text
- level_1_visual required
- runtime degradation forbidden
- preservation loss forbidden
- semantic equivalence unknown forbidden
```

---

## 27. HLDocS feedback

本仕様で得られた知見：

```text
- RoundTrip は binary equality ではない
- semantic continuity が重要
- migration continuity も RoundTrip semantic として扱える
- loss-aware verification が必要
- preservation continuity が重要
```

---

## 28. 関連仕様

本仕様は以下と連携する。

```text
Compatibility Analysis仕様
Loss Report仕様
Coverage Analysis仕様
Traceability Migration仕様
変換仕様
```

---

## 29. 結論

RoundTrip Verification は、SansaVRM における semantic preservation continuity を検証する verification layer である。

これにより、binary equality に依存せず、semantic continuity、runtime continuity、migration continuity を検証できる。

---

[目次](../../目次.md) > 仕様 > Validation Layer > RoundTrip Verification仕様
