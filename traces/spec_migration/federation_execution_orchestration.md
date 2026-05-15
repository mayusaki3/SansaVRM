# federation execution orchestration

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における federation execution orchestration を定義する。

federation execution orchestration は、validator、comparison、cleanup readiness、provenance governance、cross-project handoff を実行順序へ接続し、どの gate を通過したら次へ進めるかを整理する。

---

## 2. 基本方針

federation execution orchestration は以下を扱う。

```text
- execution sequence
- orchestration gate
- validator dependency
- comparison dependency
- cleanup readiness dependency
- provenance governance dependency
- cross-project handoff dependency
- stop / rerun / retry / superseded handling
```

federation execution orchestration は以下を行わない。

```text
- validator result を捏造しない
- cleanup execution を無断実行しない
- downstream repository を直接変更しない
- governance approval を自動承認しない
- dashboard を source of truth として扱わない
```

---

## 3. orchestration stages

orchestration stages は以下とする。

```text
O1 input intake
O2 index build
O3 validator execution
O4 artifact lifecycle propagation
O5 coexistence / mixed reconstruction review
O6 comparison workflow
O7 provenance / restriction validation
O8 cleanup readiness review
O9 execution authorization review
O10 execution / cleanup handoff
O11 post-execution validation
O12 completion review
```

---

## 4. O1 input intake

入力を収集する。

対象：

```text
- migration manifests
- canonicalization manifests
- reconstruction registries
- external artifact registries
- handoff responses
- provenance registries
- previous validator reports
```

Fail 条件：

```text
- required input missing
- malformed registry
- source_of_truth_refs missing in required scope
```

---

## 5. O2 index build

index builder を実行する。

生成：

```text
- index_bundle
- hash_index
- external_artifact_index
- reconstruction_delta_index
- provenance_index where available
```

index build は read-only でなければならない。

---

## 6. O3 validator execution

validator を順に実行する。

順序：

```text
1. manifest_validator
2. canonicalization_validator
3. rewrite_validator
4. federation_validator
5. provenance_validator where applicable
6. cleanup_gate_validator
7. projection_validator
8. risk_guard_validator
```

Fail / blocked が required scope にある場合、次 stage へ進めない。

---

## 7. O4 artifact lifecycle propagation

以下を評価する。

```text
- stale artifact
- superseded artifact
- rerun_required execution
- propagation record
- cleanup_ready invalidation
```

propagation source があるのに propagation record がない場合、orchestration を block する。

---

## 8. O5 coexistence / mixed reconstruction review

旧構成と新構成の共存状態を確認する。

確認：

```text
- separated / mixed classification
- coexistence boundary
- mixed scope tagging
- cleanup_hold
- contamination findings
```

mixed reconstruction を separated として扱ってはならない。

---

## 9. O6 comparison workflow

old/new comparison を実行する。

確認：

```text
- semantic equivalence
- traceability equivalence
- rewrite completeness
- orphan detection
- contamination detection
- cleanup blocker generation
- cleanup candidate generation
```

comparison evidence なしに cleanup readiness へ進めない。

---

## 10. O7 provenance / restriction validation

provenance / restriction / editor rights / tool provenance を検証する。

確認：

```text
- provenance graph validity
- restriction conflict
- editor attribution
- tool provenance
- distribution readiness impact
- cleanup impact
```

unresolved provenance blocker が cleanup / distribution scope にある場合、次 stage を block する。

---

## 11. O8 cleanup readiness review

cleanup execution readiness governance を評価する。

確認：

```text
- cleanup_candidate
- cleanup_hold
- cleanup_review_required
- cleanup_blockers
- rollback recoverability
- federation-wide cleanup safety
```

cleanup_ready は cleanup_execution_authorized を意味しない。

---

## 12. O9 execution authorization review

execution / cleanup の authorization を確認する。

確認：

```text
- scope frozen
- rollback restore point verified
- irreversible cleanup review completed
- governance approval recorded
- cleanup_authorization_report generated
```

authorization missing の場合、execution は開始しない。

---

## 13. O10 execution / cleanup handoff

実 execution が必要な場合、project-local execution へ handoff する。

Handoff 内容：

```text
- frozen scope
- cleanup target list
- rollback scope
- required validator reports
- authorization report
- recovery instructions
```

federation orchestration は downstream repository を直接変更しない。

---

## 14. O11 post-execution validation

execution 後に再検証する。

対象：

```text
- post-cleanup validator
- reference integrity
- traceability integrity
- provenance graph integrity
- dashboard projection
- artifact lifecycle
```

post-execution validation が fail の場合、completion へ進めない。

---

## 15. O12 completion review

reconstruction completion を確認する。

確認：

```text
- no unresolved reconstruction delta
- no unresolved cleanup blocker
- no unresolved federation dependency
- no unresolved provenance corruption
- old structure detached or archived
- audit trail recorded
```

new structure generated だけでは completion ではない。

---

## 16. orchestration stop conditions

以下の場合、orchestration を停止する。

```text
- required validator blocked
- unresolved reconstruction delta
- cleanup_hold=true in cleanup scope
- comparison evidence missing
- provenance blocker unresolved
- rollback recoverability missing
- authorization missing
- destructive operation detected in dry-run scope
```

---

## 17. rerun / retry handling

rerun_required：

```text
- schema changed
- registry changed
- validator changed
- reason taxonomy changed
- reconstruction delta detected
- external artifact freshness changed
```

retry_allowed：

```text
- transient script error
- output directory missing
- artifact upload retry
```

retry_blocked：

```text
- semantic input changed
- strict mode failure
- unknown-as-pass
- skeleton-as-pass
```

---

## 18. orchestration report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "federation_orchestration_report",
  "orchestration_status": "blocked",
  "current_stage": "O6 comparison workflow",
  "completed_stages": [],
  "blocked_reasons": [],
  "required_next_actions": [],
  "source_of_truth_refs": []
}
```

---

## 19. dashboard display

Dashboard は orchestration status を表示する。

表示対象：

```text
- current stage
- completed stages
- blocked reasons
- rerun_required
- cleanup authorization status
- completion readiness
```

Dashboard は orchestration state を独自変更しない。

---

## 20. CI mapping

CI fail 条件：

```text
- required stage skipped
- cleanup readiness without comparison evidence
- execution authorization missing
- post-execution validation fail
- completion review without cleanup evidence
```

CI warn 条件：

```text
- comparison in progress
- cleanup review required
- provenance review required outside distribution scope
```

---

## 21. 禁止事項

以下を禁止する。

```text
- validator stage を飛ばして comparison / cleanup へ進むこと
- comparison evidence なしで cleanup readiness へ進むこと
- cleanup_ready を execution authorization と扱うこと
- provenance blocker を warning のみで cleanup 通過させること
- new structure generated を reconstruction completed と扱うこと
```

---

## 22. HLDocS feedback

本 orchestration から、HLDocS 側へ以下をフィードバック候補とする。

```text
- reconstruction orchestration は stage/gate model を持つべき
- comparison / cleanup / completion は独立 stage とすべき
- provenance governance を cleanup readiness へ接続すべき
- cleanup_ready と execution authorization を分離すべき
- post-execution validation を completion 条件に含めるべき
```

---

## 23. 結論

federation execution orchestration は、SansaVRM 再構成における validator、comparison、provenance、cleanup readiness、execution authorization、completion review を統合する orchestration model である。

これにより、再構成作業を stage/gate 単位で制御し、unsafe cleanup や incomplete reconstruction completion を防止できる。
