# governance drift detection

## 1. 目的

本ドキュメントは、SansaVRM federation における governance drift detection を定義する。

governance drift detection は、vocabulary / semantic / package / replay / provenance / orchestration / federation dependency の drift を検出し、federation replayability と governance consistency を維持する。

---

## 2. 基本方針

governance drift detection は以下を扱う。

```text
- governance drift taxonomy
- drift baseline model
- acceptable / forbidden drift
- drift propagation lifecycle
- drift stabilization policy
- drift replayability
- semantic freeze interaction
- cross-project drift synchronization
```

governance drift detection は以下を行わない。

```text
- experimental drift を即 federation-wide blocker としない
- semantic drift を cosmetic drift と誤分類しない
- stale baseline を drift baseline として扱わない
- replay-incompatible drift を warning のみで通過させない
```

---

## 3. drift positioning

drift detection は以下に属する。

```text
Federated Governance Layer
Operational Traceability Layer
Cross-Project Compatibility Layer
```

consistency validator / replay validator と連携する。

---

## 4. governance drift taxonomy

drift taxonomy：

```text
vocabulary drift
semantic drift
package drift
projection drift
replay drift
provenance drift
orchestration drift
federation dependency drift
```

taxonomy ambiguity は governance warning または blocker。

---

## 5. vocabulary drift

対象：

```text
- reserved vocabulary semantic change
- alias semantic divergence
- cross-project terminology mismatch
- lifecycle semantic mismatch
```

例：

```text
ready == authorized
```

化は forbidden drift。

---

## 6. semantic drift

対象：

```text
- checkpoint semantic change
- authorization semantic change
- completion semantic change
- replay semantic change
- projection/source semantic collapse
```

semantic drift は replay compatibility に影響する。

---

## 7. package drift

対象：

```text
- package dependency drift
- package compatibility drift
- package lifecycle drift
- federation baseline drift
```

package drift は rollout policy を再評価しうる。

---

## 8. projection drift

対象：

```text
- projection wording drift
- projection lifecycle drift
- projection/source semantic mismatch
- dashboard semantic divergence
```

projection wording drift は acceptable drift になりうる。

---

## 9. replay drift

対象：

```text
- replay evidence structure drift
- package replay incompatibility
- policy version replay incompatibility
- replay reason code drift
```

replay drift は replay validator fail 条件になりうる。

---

## 10. provenance drift

対象：

```text
- provenance graph semantic drift
- restriction merge semantic drift
- editor attribution semantic drift
- temporary bridge semantic drift
```

provenance drift は distribution readiness に影響する。

---

## 11. orchestration drift

対象：

```text
- checkpoint transition drift
- cleanup orchestration drift
- rollback semantic drift
- authorization gating drift
```

orchestration drift は cleanup/completion progression を block しうる。

---

## 12. federation dependency drift

対象：

```text
- dependency graph semantic drift
- propagation lifecycle drift
- isolation boundary drift
- federation scope drift
```

federation dependency drift は federation-wide response を発生させうる。

---

## 13. drift severity

drift severity：

```text
cosmetic_drift
operational_drift
semantic_drift
replay_breaking_drift
federation_breaking_drift
```

severity は propagation / replay / rollout に影響する。

---

## 14. drift baseline model

drift baseline 候補：

```text
federation baseline package
semantic freeze version
approved vocabulary registry
replay-compatible package set
stable orchestration baseline
```

baseline missing は drift validation blocker。

---

## 15. acceptable drift

acceptable drift 候補：

```text
- projection wording refinement
- non-semantic alias addition
- optional dashboard metadata addition
- replay-compatible package extension
```

acceptable drift は federation review のみで通過しうる。

---

## 16. forbidden drift

forbidden drift 候補：

```text
- ready == authorized semantic collapse
- passed == completed semantic collapse
- projection_valid == source_valid collapse
- replay-incompatible semantic change
- isolation boundary removal without review
```

forbidden drift は federation blocker。

---

## 17. drift propagation lifecycle

drift propagation lifecycle：

```text
drift_detected
drift_classified
drift_propagated
drift_review_required
drift_stabilized
drift_resolved
drift_archived
```

forbidden drift は stabilization 前に federation-wide rollout してはならない。

---

## 18. drift propagation

例：

```text
semantic drift
↓
package compatibility drift
↓
replay drift
↓
federation-breaking drift
```

Propagation chain は replayable であるべき。

---

## 19. drift stabilization policy

stabilization policy 候補：

```text
- debounce window
- staged propagation
- semantic review threshold
- compatibility freeze
- replay verification window
```

experimental drift は stabilization review 前に production federation へ rollout しない。

---

## 20. drift replayability

drift change は replayable であるべき。

必要：

```text
- previous semantic refs
- changed semantic refs
- package version diff
- compatibility matrix diff
- replay impact refs
```

Replay 不可能な drift は governance risk。

---

## 21. semantic freeze interaction

semantic freeze 中：

```text
- semantic drift
- lifecycle drift
- replay-breaking drift
```

は critical または federation blocker。

freeze 中の cosmetic drift は review_required としうる。

---

## 22. cross-project drift synchronization

対象例：

```text
SansaVRM
SansaVRM Studio AI
SansaXR
HLDocS
```

同期対象：

```text
- vocabulary baseline
- package baseline
- replay compatibility baseline
- semantic freeze baseline
```

cross-project drift unsynchronized は federation compatibility risk。

---

## 23. drift invalidation

以下は drift invalidation を発生させる。

```text
- stale drift baseline
- superseded semantic mapping
- replay baseline invalidated
- package baseline replaced
```

invalidated drift analysis を active rollout evidence として扱ってはならない。

---

## 24. drift report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_drift_report",
  "drift_taxonomy": "semantic drift",
  "drift_severity": "replay_breaking_drift",
  "drift_status": "drift_review_required",
  "baseline_refs": [],
  "compatibility_impact_refs": [],
  "source_of_truth_refs": []
}
```

---

## 25. reason codes

候補 reason code：

```text
semantic_collapse_detected
replay_incompatible_drift
package_baseline_drift
projection_source_semantic_drift
forbidden_semantic_change
isolation_boundary_drift
semantic_freeze_violation
unsynchronized_cross_project_drift
```

---

## 26. orchestration relation

federation execution orchestration は forbidden drift / replay-breaking drift を受けた場合、以下を停止する。

```text
- cleanup authorization
- completion review
- production rollout
- federation baseline update
```

---

## 27. dashboard relation

Dashboard は drift summary を表示できる。

表示対象：

```text
- drift severity summary
- forbidden drift findings
- semantic freeze violations
- replay-breaking drift findings
- unsynchronized drift findings
```

Dashboard は drift classification を独自決定しない。

---

## 28. CI mapping

CI fail 条件：

```text
- forbidden drift detected
- replay-breaking drift unresolved
- semantic freeze violation
- unsynchronized cross-project drift in production scope
- stale baseline used for drift analysis
```

CI warn 条件：

```text
- acceptable drift pending review
- cosmetic drift unresolved
- replay verification pending
```

---

## 29. 禁止事項

以下を禁止する。

```text
- semantic drift を cosmetic drift として扱うこと
- replay-breaking drift を known limitation として production rollout すること
- stale baseline を drift baseline として扱うこと
- semantic freeze violation を local-only issue として扱うこと
- unsynchronized drift を federation stable baseline に混入すること
```

---

## 30. HLDocS feedback

本 drift model から、HLDocS 側へ以下をフィードバック候補とする。

```text
- governance drift taxonomy を formalize すべき
- semantic freeze と drift interaction を formalize すべき
- replay-breaking drift を federation blocker とすべき
- acceptable/forbidden drift classification を formalize すべき
- cross-project drift synchronization を governance layer に含めるべき
```

---

## 31. 結論

governance drift detection は、SansaVRM federation governance semantic の drift を検出・分類・伝播制御する model である。

これにより、semantic drift、replay drift、federation dependency drift を replayable に管理しつつ、semantic freeze と federation baseline の安定性を維持できる。
