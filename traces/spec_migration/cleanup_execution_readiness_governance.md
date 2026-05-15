# cleanup execution readiness governance

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における cleanup execution readiness governance を定義する。

cleanup execution readiness governance は、cleanup execution を実行可能と判定するための governance / validation / rollback / federation safety を扱う。

重要：

```text
cleanup_ready
```

と：

```text
cleanup_execution_authorized
```

は異なる。

---

## 2. 基本方針

cleanup execution readiness は以下を扱う。

```text
- cleanup readiness
- cleanup approval boundary
- rollback recoverability
- cleanup safety
- federation-wide cleanup safety
- comparison evidence
- cleanup blocker resolution
- cleanup execution authorization
```

cleanup execution readiness は以下を行わない。

```text
- cleanup_ready を cleanup_execution と混同しない
- validator pass のみで cleanup execution を許可しない
- rollback 不明状態で cleanup execution を許可しない
- unresolved contamination を cleanup warning に丸めない
```

---

## 3. readiness stages

cleanup readiness stages は以下とする。

```text
cleanup_not_ready
cleanup_candidate
cleanup_blocked
cleanup_hold
cleanup_ready
cleanup_review_required
cleanup_execution_authorized
cleanup_execution_in_progress
cleanup_execution_completed
cleanup_rollback_required
cleanup_rollback_completed
```

---

## 4. cleanup_candidate

cleanup_candidate は以下を満たす。

```text
- comparison completed
- semantic_equivalent or cleanup_equivalent
- traceability preserved
- rollback scope exists
- no unresolved orphan in cleanup scope
```

重要：

```text
cleanup_candidate
```

は：

```text
cleanup execution approval
```

ではない。

---

## 5. cleanup_hold

cleanup_hold は cleanup execution を停止する状態である。

cleanup_hold 条件：

```text
- unresolved contamination
- unresolved restriction merge
- unresolved provenance chain
- unresolved traceability mismatch
- rollback ambiguity
- unresolved federation dependency
- unresolved reconstruction delta
- temporary bridge unresolved
```

cleanup_hold 中は cleanup_execution_authorized にしてはならない。

---

## 6. cleanup_review_required

cleanup_review_required は human review が必要な状態である。

例：

```text
- restriction conflict
- provenance ambiguity
- mixed reconstruction boundary ambiguity
- semantic comparison ambiguity
- irreversible cleanup candidate
```

MVP では cleanup_review_required を重視する。

---

## 7. cleanup readiness gate

cleanup_ready 条件：

```text
- comparison evidence exists
- cleanup blockers resolved
- rollback recoverability confirmed
- no unresolved contamination in cleanup scope
- no unresolved orphan in cleanup scope
- no unresolved federation dependency in cleanup scope
- required validators pass or allowed warn
```

cleanup_ready は cleanup execution authorization を意味しない。

---

## 8. cleanup execution authorization

cleanup_execution_authorized 条件：

```text
- cleanup_ready=true
- cleanup_review_required resolved
- cleanup execution scope frozen
- rollback restore point verified
- cleanup impact reviewed
- federation cleanup safety confirmed
```

cleanup_execution_authorized なしに cleanup execution を開始してはならない。

---

## 9. cleanup scope freeze

cleanup execution 前に cleanup scope freeze を行う。

freeze 対象：

```text
- cleanup target list
- rollback scope
- affected references
- affected aliases
- affected provenance edges
- affected federation dependencies
```

cleanup scope freeze なしに cleanup execution を開始してはならない。

---

## 10. rollback recoverability

cleanup execution 前に rollback recoverability を確認する。

必要：

```text
- restore point exists
- before hash exists
- restore order exists
- alias rollback exists
- provenance rollback exists
- dashboard/report stale handling exists
```

rollback recoverability unresolved は cleanup blocker とする。

---

## 11. irreversible cleanup

以下は irreversible cleanup 候補である。

```text
- old artifact permanent deletion
- provenance edge removal
- alias expiration cleanup
- audit history detachment
```

irreversible cleanup は cleanup_review_required を mandatory とする。

---

## 12. federation-wide cleanup safety

cleanup execution は project-local safety だけでは不十分。

確認対象：

```text
- downstream dependency
- external artifact dependency
- handoff dependency
- provenance dependency
- distribution governance dependency
```

project-local cleanup_ready を federation cleanup authorization と混同してはならない。

---

## 13. cleanup blocker taxonomy

cleanup blocker は以下に分類する。

```text
semantic_blocker
traceability_blocker
rewrite_blocker
contamination_blocker
provenance_blocker
restriction_blocker
rollback_blocker
federation_blocker
review_blocker
```

cleanup blocker unresolved は cleanup execution 不可。

---

## 14. cleanup evidence

cleanup execution 前に evidence artifact を保持する。

候補：

```text
cleanup_candidate_report
cleanup_blocker_report
comparison_report
rollback_recoverability_report
cleanup_authorization_report
```

cleanup evidence なしに cleanup execution を行ってはならない。

---

## 15. cleanup authorization report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "cleanup_authorization_report",
  "cleanup_execution_authorized": false,
  "cleanup_scope_frozen": false,
  "rollback_recoverability_confirmed": false,
  "cleanup_blockers": [],
  "comparison_evidence_refs": []
}
```

---

## 16. cleanup execution governance

cleanup execution 中は以下を追跡する。

```text
- cleanup progress
- cleanup failure
- cleanup rollback trigger
- stale artifact propagation
- alias cleanup impact
- provenance cleanup impact
```

cleanup execution failure は cleanup_execution_completed を意味しない。

---

## 17. cleanup rollback

cleanup rollback 条件：

```text
- cleanup failure
- unresolved downstream breakage
- unresolved provenance corruption
- unresolved traceability corruption
- invalid cleanup scope
```

cleanup rollback は restore order に従う。

---

## 18. reconstruction completion relation

cleanup_execution_completed 後も：

```text
reconstruction_completed
```

とは限らない。

必要：

```text
- cleanup audit completed
- no unresolved federation dependency
- no unresolved reconstruction delta
- no unresolved provenance corruption
```

---

## 19. validator interaction

cleanup readiness governance は以下 validator と接続する。

```text
cleanup_gate_validator
risk_guard_validator
rewrite_validator
provenance_validator
restriction_merge_validator
integrity_validator
```

validator fail / blocked は cleanup execution authorization を block する。

---

## 20. dashboard display

Dashboard は cleanup readiness を表示する。

表示対象：

```text
- readiness stage
- cleanup blockers
- rollback recoverability
- cleanup scope freeze
- cleanup authorization status
- cleanup rollback state
- federation cleanup impact
```

Dashboard は cleanup authorization を独自決定しない。

---

## 21. CI mapping

CI fail 条件：

```text
- cleanup execution without authorization
- cleanup execution without rollback recoverability
- unresolved cleanup blocker ignored
- project-local cleanup_ready treated as federation-safe
- irreversible cleanup without review
```

CI warn 条件：

```text
- cleanup_review_required
- cleanup_hold
- optional provenance ambiguity outside cleanup scope
```

---

## 22. 禁止事項

以下を禁止する。

```text
- cleanup_ready を cleanup_execution_authorized と扱うこと
- rollback recoverability 不明で cleanup execution を行うこと
- unresolved cleanup blocker を warning に丸めること
- irreversible cleanup を review なしで行うこと
- project-local cleanup_ready を federation-safe cleanup と扱うこと
```

---

## 23. HLDocS feedback

本 governance から、HLDocS 側へ以下をフィードバック候補とする。

```text
- cleanup readiness と cleanup execution authorization を分離すべき
- cleanup scope freeze を formal step にすべき
- rollback recoverability review を mandatory にすべき
- irreversible cleanup は review-required にすべき
- federation-wide cleanup safety を扱うべき
```

---

## 24. 結論

cleanup execution readiness governance は、cleanup execution を安全に実施するための readiness / review / rollback / federation safety governance である。

これにより、comparison evidence、rollback recoverability、cleanup blocker resolution を経るまで cleanup execution を authorize せず、安全に reconstruction completion へ進めることができる。
