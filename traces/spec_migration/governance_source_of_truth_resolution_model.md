# governance source-of-truth resolution model

## 1. 目的

本ドキュメントは、SansaVRM federation における governance source-of-truth resolution model を定義する。

governance source-of-truth resolution model は、canonical source 判定、authority conflict resolution、projection fallback boundary、cross-project source synchronization を扱う。

---

## 2. 基本方針

本 model は以下を扱う。

```text
- source-of-truth taxonomy
- canonical source resolution
- authority-based source resolution
- projection fallback boundary
- source replayability
- source invalidation
- source conflict resolution
- cross-project source synchronization
```

本 model は以下を行わない。

```text
- stale projection を canonical source と扱わない
- authority ambiguity を silent resolution しない
- replay-incomplete source transition を production source と扱わない
- temporary recovery bridge を canonical source と扱わない
```

---

## 3. source-of-truth positioning

source-of-truth governance は以下に属する。

```text
Source-of-Truth Governance Layer
Federated Governance Layer
Operational Traceability Layer
Authority Governance Layer
```

source-of-truth は governance decision の canonical semantic origin を表す。

---

## 4. source-of-truth taxonomy

source taxonomy：

```text
canonical_source
project_source
federation_source
projection_source
recovery_source
temporary_bridge_source
archived_source
```

canonical_source 以外は条件付き利用となる。

---

## 5. canonical_source

canonical_source：

```text
- replayable
- authority verified
- baseline synchronized
- non-invalidated
- federation compatible
```

canonical_source は governance semantic authority を持つ。

---

## 6. projection_source

projection_source：

```text
- visualization only
- summarized state
- operational convenience
```

projection_source は canonical semantic authority を持たない。

---

## 7. recovery_source

recovery_source：

```text
- recovery temporary usage
- reconstruction candidate
- replayability review mandatory
```

recovery_source は canonical_source と同義ではない。

---

## 8. temporary_bridge_source

temporary_bridge_source：

```text
- migration bridge
- emergency bridge
- rollback compatibility bridge
```

temporary_bridge_source は expiration mandatory。

---

## 9. canonical source resolution

canonical source resolution は deterministic であるべき。

必要：

```text
- authority ownership
- replayability
- baseline validity
- synchronization completeness
- invalidation status
```

---

## 10. source resolution order

候補 resolution order：

```text
canonical_source
↓
federation_source
↓
project_source
↓
recovery_source
↓
projection_source
```

projection_source は final fallback としても authority を持たない。

---

## 11. authority-based source resolution

source resolution は authority に依存する。

候補 authority：

```text
semantic_authority
release_authority
audit_authority
distribution_authority
recovery_authority
```

Authority ambiguity は review_required または blocker。

---

## 12. source replayability

source replayability 条件：

```text
- source refs recorded
- authority refs recorded
- synchronization refs recorded
- invalidation refs recorded
- baseline refs recorded
```

Replay 不可能 source transition は production blocker。

---

## 13. source invalidation

以下は source invalidation trigger：

```text
- authority invalidated
- replayability lost
- baseline invalidated
- synchronization failure
- stale projection in critical scope
- unresolved semantic conflict
```

invalidated source を canonical source として扱ってはならない。

---

## 14. source conflict taxonomy

source conflict taxonomy：

```text
authority_conflict
semantic_conflict
projection_conflict
baseline_conflict
recovery_conflict
compatibility_conflict
```

Conflict severity は replayable でなければならない。

---

## 15. source conflict resolution

conflict resolution 候補：

```text
- authority precedence
- latest replayable canonical source
- freeze-valid source preference
- federation-approved baseline preference
- manual review escalation
```

silent source override を禁止する。

---

## 16. projection fallback boundary

projection fallback boundary：

```text
projection usable for:
- visualization
- operational summary
- temporary monitoring

projection unusable for:
- canonical semantic authority
- release authorization
- distribution authorization
- audit evidence source
```

---

## 17. recovery fallback boundary

recovery fallback boundary：

```text
recovery source usable for:
- recovery reconstruction
- emergency stabilization

recovery source unusable for:
- permanent canonical source
- long-term semantic authority
```

---

## 18. source synchronization

同期対象：

```text
baseline refs
authority refs
freeze refs
compatibility refs
replay refs
```

unsynchronized source state は federation risk。

---

## 19. cross-project source synchronization

対象例：

```text
SansaVRM
SansaVRM Studio AI
HLDocS
distribution pipeline
```

同期対象：

```text
source taxonomy
resolution order
authority precedence
projection policy
recovery fallback policy
```

unsynchronized source policy は federation ambiguity risk。

---

## 20. source lifecycle

source lifecycle：

```text
source_pending
source_active
source_review_required
source_invalidated
source_superseded
source_archived
```

invalidated source を active canonical source として扱ってはならない。

---

## 21. source report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_source_resolution_report",
  "source_taxonomy": "canonical_source",
  "source_status": "source_review_required",
  "authority_refs": [],
  "baseline_refs": [],
  "source_of_truth_refs": []
}
```

---

## 22. reason codes

候補 reason code：

```text
source_authority_ambiguous
source_projection_used_as_canonical
source_replayability_missing
source_invalidated_but_active
source_cross_project_unsynchronized
source_conflict_unresolved
source_recovery_bridge_expired
```

---

## 23. orchestration relation

federation execution orchestration は以下を block する。

```text
- projection source used as canonical source
- authority ambiguity in production scope
- unresolved source conflict
- replayability missing in source transition
```

---

## 24. dashboard relation

Dashboard は source summary を表示できる。

表示対象：

```text
- canonical source status
- projection freshness
- synchronization status
- authority ownership
- source conflict summary
```

Dashboard は canonical authority を独自決定しない。

---

## 25. CI mapping

CI fail 条件：

```text
- projection source used as canonical source
- invalidated source used as active canonical source
- unresolved authority conflict in production scope
- replayability missing in canonical source transition
```

CI warn 条件：

```text
- source_review_required
- projection stale outside critical scope
- cross-project source acknowledgment pending outside production scope
```

---

## 26. 禁止事項

以下を禁止する。

```text
- stale projection を canonical source と扱うこと
- authority ambiguity を silent resolution すること
- temporary recovery bridge を canonical source と扱うこと
- replay-incomplete source transition を production source に使うこと
```

---

## 27. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバック候補とする。

```text
- source-of-truth resolution model を formalize すべき
- canonical source / projection / recovery source を分離すべき
- authority-based source resolution を formal artifact 化すべき
- projection fallback boundary を formal invariant 化すべき
- cross-project source synchronization を governance layer に含めるべき
```

---

## 28. 結論

governance source-of-truth resolution model は、SansaVRM federation における canonical semantic source を replayable に決定する model である。

これにより、projection、recovery bridge、authority conflict、cross-project synchronization を含む複雑な federation 環境でも、canonical source ambiguity を防止できる。
