<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-5007-loss-report-specification
lang: ja-JP
canonical_title: Loss Report仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Validation Layer > Loss Report仕様

# Loss Report仕様

## 1. 目的

本仕様は、SansaVRM における semantic loss、approximation、unsupported feature、および preservation 状態の記録方法を定義する。

Loss Report は以下を目的とする。

```text
- non-reversible conversion の可視化
- semantic loss の明示
- approximation の記録
- unsupported feature の記録
- preservation_only 状態の記録
- roundtrip verification の補助
- silent loss の防止
```

---

## 2. 基本方針

Loss Report は diagnostics とは独立した semantic preservation report として扱う。

以下を原則とする。

```text
1. semantic loss を構造化して記録する
2. unsupported feature を記録する
3. approximation を記録する
4. preservation_only を記録する
5. source_format / target_format を保持する
6. roundtrip verification と連携する
```

---

## 3. Loss Report Container

Loss Report Container は変換全体の semantic loss 情報を保持する。

最低限以下を持つ。

```text
loss_report_id
source_format
target_format
conversion_profile_ref
entry_list
summary
```

---

## 4. Loss Entry

Loss Entry は単一 semantic loss を表す。

最低限以下を持つ。

```text
loss_id
loss_type
severity
source_ref
target_ref
description
preservation_state
approximation_state
```

---

## 5. loss_type

loss_type は以下を許可する。

```text
semantic_loss
unsupported_feature
approximation
preservation_only
runtime_loss
roundtrip_difference
compatibility_loss
migration_loss
```

---

## 6. severity

severity は以下を許可する。

```text
info
warning
error
critical
```

---

## 7. preservation_state

preservation_state は以下を許可する。

```text
fully_preserved
partially_preserved
preserved_raw_only
not_preserved
unknown
```

---

## 8. approximation_state

approximation_state は以下を許可する。

```text
none
approximated
estimated
fallback_applied
runtime_substituted
```

---

## 9. source_ref / target_ref

source_ref / target_ref は loss 対象を参照する。

例：

```text
module_id
property_id
morph_id
animation_id
physics_id
runtime_binding_id
```

---

## 10. unsupported feature

Loss Report は unsupported feature を記録できなければならない。

例：

```text
- unsupported FBX tangent
- unsupported MMD morph
- unsupported VRM metadata
- unsupported MuJoCo actuator semantic
```

---

## 11. approximation

Loss Report は approximation を記録できなければならない。

例：

```text
- tangent approximation
- constraint approximation
- collision approximation
- runtime parameter approximation
```

---

## 12. preservation_only

preservation_only は semantic interpretation 不能だが raw 保持された状態を示す。

以下を許可する。

```text
- source_raw
- raw_binary_ref
- adapter_artifact_ref
```

---

## 13. roundtrip_difference

roundtrip_difference は RoundTrip 後の semantic difference を示す。

例：

```text
VRM → SansaVRM → VRM
FBX → SansaVRM → FBX
MMD → SansaVRM → MMD
```

---

## 14. migration_loss

migration_loss は relocation / migration に伴う semantic loss を示す。

例：

```text
- sec_id continuity lost
- orphan migration mapping
- semantic_equivalent unknown
```

---

## 15. Diagnostics との関係

Loss Report と Diagnostics は分離する。

```text
Diagnostics:
validation / runtime / migration issue notification

Loss Report:
semantic preservation analysis
```

ただし、critical semantic loss は diagnostics にも出力してよい。

---

## 16. Validator Requirements

Validator は Loss Report を検査対象に含めることができる。

例：

```text
- preservation_state = not_preserved
- semantic_loss severity = critical
- unsupported_feature severity = error
```

---

## 17. RoundTrip Verification Requirements

RoundTrip verifier は Loss Report と連携する。

以下を検査する。

```text
- semantic difference
- unsupported feature
- approximation applied
- raw preservation continuity
```

---

## 18. Compatibility Analysis Requirements

Compatibility analyzer は Loss Report を利用できる。

例：

```text
VRM 0.x compatibility
VRM 1.0 compatibility
MMD compatibility
FBX compatibility
MuJoCo compatibility
```

---

## 19. CI Requirements

CI は Loss Report を gate 条件に含めることができる。

例：

```text
- critical semantic_loss 禁止
- not_preserved 禁止
- unsupported_feature error 禁止
- migration_loss error 禁止
```

---

## 20. 出力例

```json
{
  "loss_id": "loss-0001",
  "loss_type": "approximation",
  "severity": "warning",
  "source_ref": "fbx_tangent_001",
  "target_ref": "sansavrm_material_003",
  "description": "FBX tangent data approximated.",
  "preservation_state": "partially_preserved",
  "approximation_state": "approximated"
}
```

---

## 21. HLDocS feedback

本仕様で得られた知見：

```text
- diagnostics と semantic loss は分離した方が強い
- preservation_state が重要
- approximation_state が必要
- migration_loss が必要
- silent semantic loss を禁止すべき
```

---

## 22. 関連仕様

本仕様は以下と連携する。

```text
Diagnostics仕様
Coverage Analysis仕様
RoundTrip Semantic Criteria
変換仕様
Traceability Migration仕様
```

---

## 23. 結論

Loss Report は、SansaVRM における semantic preservation observability の中核である。

これにより、unsupported、approximation、preservation_only、migration_loss を可視化し、silent semantic loss を防止できる。

---

[目次](../../目次.md) > 仕様 > Validation Layer > Loss Report仕様
