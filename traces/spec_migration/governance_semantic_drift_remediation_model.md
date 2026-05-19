# governance semantic drift remediation model

## 1. 目的

本ドキュメントは、SansaVRM federation における governance semantic drift remediation model を定義する。

semantic drift remediation model は、semantic drift の検出、classification、containment、reconciliation、repair、stabilization を扱う。

---

## 2. 基本方針

本 model は以下を扱う。

```text
- semantic drift taxonomy
- drift severity
- drift containment
- drift remediation
- drift reconciliation
- drift replayability
- drift invalidation
- cross-project drift synchronization
```

本 model は以下を行わない。

```text
- silent semantic remap を許可しない
- projection drift を canonical semantic drift resolution と扱わない
- replay-incomplete remediation を production stabilization に使わない
- unresolved semantic drift を stable federation と扱わない
```

---

## 3. drift positioning

semantic drift remediation は以下に属する。

```text
Semantic Governance Layer
Semantic Vocabulary Governance Layer
Compatibility Governance Layer
Operational Traceability Layer
```

semantic drift は governance semantic consistency を破壊しうる。

---

## 4. semantic drift taxonomy

drift taxonomy：

```text
vocabulary_drift
semantic_mapping_drift
compatibility_drift
policy_semantic_drift
restriction_semantic_drift
projection_source_of_truth_drift
bridge_semantic_drift
```

---

## 5. drift severity

severity taxonomy：

```text
review_required
compatibility_risk
policy_risk
distribution_risk
production_blocker
federation_blocker
```

severity unknown は review_required 以上に倒す。

---

## 6. drift detection

対象例：

```text
silent semantic remap
vocabulary mismatch
invalidated semantic mapping active
projection treated as canonical
compatibility matrix mismatch
policy semantic reinterpretation
```

---

## 7. drift containment

containment 候補：

```text
distribution freeze
bridge isolation
compatibility downgrade
policy evaluation review escalation
runtime execution block
```

containment unresolved の場合、production stabilization を停止する。

---

## 8. drift remediation

remediation 候補：

```text
semantic mapping repair
vocabulary synchronization
compatibility matrix rebuild
policy expression rebuild
restriction propagation rebuild
```

silent remediation を禁止する。

---

## 9. drift reconciliation

reconciliation 候補：

```text
semantic reconciliation
vocabulary reconciliation
policy reconciliation
restriction reconciliation
compatibility reconciliation
```

reconciliation は replayable mandatory。

---

## 10. drift authority

authority taxonomy：

```text
semantic_authority
compatibility_authority
policy_authority
restriction_authority
review_authority
```

Authority ambiguity は review_required または blocker。

---

## 11. drift replayability

replayability 条件：

```text
- drift refs recorded
- remediation refs recorded
- reconciliation refs recorded
- authority refs recorded
- containment refs recorded
```

Replay 不可能 remediation は production stabilization に使ってはならない。

---

## 12. drift invalidation

invalidation trigger：

```text
semantic mapping invalidated
vocabulary invalidated
compatibility matrix invalidated
policy expression invalidated
restriction propagation invalidated
replayability lost
```

invalidated remediation を active stabilization evidence として扱ってはならない。

---

## 13. cross-project drift synchronization

対象例：

```text
SansaVRM
SansaVRM Studio AI
HLDocS
distribution pipeline
```

同期対象：

```text
drift taxonomy
severity taxonomy
containment taxonomy
remediation taxonomy
reconciliation taxonomy
```

---

## 14. drift report

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_semantic_drift_remediation_report",
  "drift_taxonomy": "semantic_mapping_drift",
  "drift_status": "review_required",
  "severity": "distribution_risk",
  "containment_refs": [],
  "source_of_truth_refs": []
}
```

---

## 15. reason codes

```text
silent_semantic_remap_detected
projection_treated_as_source_of_truth
semantic_mapping_invalidated_but_active
remediation_replayability_missing
cross_project_semantic_unsynchronized
semantic_drift_unresolved
```

---

## 16. orchestration relation

federation execution orchestration は以下を block する。

```text
- unresolved semantic drift in production scope
- replayability missing in remediation
- invalidated semantic remediation active
- projection treated as canonical source-of-truth
```

---

## 17. HLDocS feedback

```text
- semantic drift remediation model を formalize すべき
- silent semantic remap detection を governance layer に含めるべき
- containment / remediation / reconciliation を formal artifact 化すべき
- projection != source-of-truth invariant を強化すべき
```

---

## 18. 結論

governance semantic drift remediation model は、SansaVRM federation における semantic drift を replayable に修復・安定化する model である。

これにより、cross-project semantic consistency を維持できる。
