# cross-project governance response model

## 1. 目的

本ドキュメントは、SansaVRM federation における cross-project governance response model を定義する。

cross-project governance response model は、ある project の governance / schema / validator / provenance / orchestration 変更が、他 project にどう伝播し、どう invalidation / rerun / review / cleanup hold を発生させるかを扱う。

---

## 2. 基本方針

cross-project governance response model は以下を扱う。

```text
- cross-project response taxonomy
- response propagation lifecycle
- federation dependency graph
- response severity / scope
- response stabilization
- acknowledgment tracking
- response replayability
- federation isolation boundary
```

cross-project governance response model は以下を行わない。

```text
- local experimental change を federation-wide breaking change としない
- unstable propagation を panic rerun に接続しない
- stale response を active federation evidence と扱わない
- response projection を source of truth と扱わない
```

---

## 3. federation positioning

cross-project governance response は以下に属する。

```text
Federated Governance Layer
Federated Operational Orchestration Layer
Cross-Project Dependency Layer
```

Core Semantic Layer ではない。

---

## 4. federation dependency graph

federation dependency graph は project dependency を表す。

対象例：

```text
SansaVRM
SansaVRM Studio AI
SansaXR
HLDocS
```

依存例：

```text
SansaVRM provenance schema
↓
Studio AI provenance workflow
↓
SansaXR federation projection
↓
HLDocS governance alignment
```

---

## 5. response taxonomy

response taxonomy：

```text
informational
review_required
rerun_required
cleanup_hold
checkpoint_invalidation
authorization_block
completion_superseded
federation_breaking
```

response taxonomy ambiguity は governance warning または blocker とする。

---

## 6. response severity

response severity：

```text
low
moderate
high
critical
federation_critical
```

severity は propagation scope / acknowledgment requirement / replay retention に影響する。

---

## 7. response scope

response scope：

```text
project_local
cross_project
federation_wide
distribution_wide
```

scope mismatch は propagation inconsistency とする。

---

## 8. response lifecycle

response lifecycle：

```text
response_generated
response_propagated
response_acknowledged
response_in_effect
response_stale
response_superseded
response_resolved
response_archived
```

stale / superseded response を active federation governance evidence として扱ってはならない。

---

## 9. response propagation

response propagation は dependency graph に従う。

例：

```text
schema change
↓
validator invalidation
↓
checkpoint invalidation
↓
cleanup_hold
↓
completion_superseded
```

Propagation chain は replayable でなければならない。

---

## 10. propagation triggers

propagation trigger 候補：

```text
- schema change
- validator taxonomy change
- provenance graph model change
- cleanup lifecycle change
- rollback semantics change
- authorization policy change
- federation packaging change
```

trigger severity に応じて propagation scope を制御する。

---

## 11. response stabilization

一時的 change による federation panic rerun を防ぐ。

stabilization policy 候補：

```text
- debounce window
- staged propagation
- confirmation threshold
- repeated-change aggregation
- stabilization hold
```

stabilization pending 中は federation-wide invalidation を抑制できる。

---

## 12. acknowledgment tracking

propagation 先 project は acknowledgment を返せる。

acknowledgment 状態：

```text
ack_pending
ack_received
ack_review_required
ack_rejected
ack_superseded
```

ack missing が critical scope の場合、federation blocker としうる。

---

## 13. response replayability

response chain は replayable であるべき。

再構成対象：

```text
- why federation_breaking triggered
- why cleanup_hold propagated
- why checkpoint invalidated
- why authorization blocked
```

Replay に必要：

```text
- source refs
- dependency graph refs
- policy version
- propagation chain
- acknowledgment chain
```

---

## 14. federation isolation boundary

isolation boundary により propagation を制限する。

例：

```text
experimental project
sandbox federation
preview federation
```

は：

```text
production federation
```

へ直接 invalidation propagation しない。

---

## 15. isolation policy

isolation policy 候補：

```text
isolated
semi_isolated
review_gated
fully_federated
```

isolated project の response は federation-wide propagation を禁止できる。

---

## 16. response invalidation

以下は response invalidation を発生させる。

```text
- newer response generated
- dependency graph changed
- acknowledgment chain stale
- propagation target removed
- federation scope changed
```

invalidated response を active propagation evidence として扱ってはならない。

---

## 17. response superseded

以下は response superseded を発生させる。

```text
- rerun generated newer response
- stabilization aggregation generated merged response
- rollback generated replacement response
```

superseded response は archive/replay source にできる。

---

## 18. response/source separation

重要：

```text
response projection
```

と：

```text
source governance evidence
```

を混同してはならない。

例：

```text
response says cleanup_hold resolved
```

でも：

```text
cleanup evidence stale
```

なら active resolution ではない。

---

## 19. response audit

response operation は audit mandatory。

記録：

```text
- propagation reason
- propagation chain
- severity
- scope
- stabilization policy
- acknowledgment chain
- replay refs
```

response audit missing は governance warning または blocker。

---

## 20. response report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "cross_project_governance_response_report",
  "response_taxonomy": "rerun_required",
  "response_severity": "high",
  "response_scope": "cross_project",
  "response_status": "response_propagated",
  "dependency_refs": [],
  "acknowledgment_refs": []
}
```

---

## 21. dashboard relation

Dashboard は response summary を表示できる。

表示対象：

```text
- active response summary
- federation-breaking summary
- unresolved acknowledgment count
- cleanup_hold propagation
- completion_superseded propagation
```

Dashboard は propagation decision を独自決定しない。

---

## 22. CI mapping

CI fail 条件：

```text
- federation_breaking ignored in affected scope
- stale response used as active propagation evidence
- isolation boundary violation
- unresolved acknowledgment in critical scope
- response/source mismatch unresolved in critical scope
```

CI warn 条件：

```text
- stabilization pending
- propagation aggregation pending
- response replay evidence incomplete
```

---

## 23. 禁止事項

以下を禁止する。

```text
- experimental change を federation-wide invalidation すること
- stale/superseded response を active federation evidence と扱うこと
- acknowledgment missing を silent ignore すること
- response projection を source of truth と扱うこと
- replay impossible propagation chain を valid governance response と扱うこと
```

---

## 24. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバック候補とする。

```text
- cross-project governance response model を formalize すべき
- federation dependency graph を formal artifact 化すべき
- response propagation / stabilization lifecycle を formalize すべき
- acknowledgment tracking を formal governance に含めるべき
- federation isolation boundary を formalize すべき
```

---

## 25. 結論

cross-project governance response model は、SansaVRM federation における cross-project governance propagation を定義する model である。

これにより、schema / validator / provenance / cleanup / authorization の変更を replayable federation response として管理しつつ、stabilization policy と isolation boundary により unsafe propagation を防止できる。
