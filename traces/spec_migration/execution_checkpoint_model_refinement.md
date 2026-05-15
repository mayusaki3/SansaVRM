# execution checkpoint model refinement

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における execution checkpoint model refinement を定義する。

execution checkpoint model は、federation execution orchestration における phase / stage progression を checkpoint 単位で制御する governance model である。

---

## 2. 基本方針

execution checkpoint model は以下を扱う。

```text
- checkpoint lifecycle
- checkpoint transition
- checkpoint promotion criteria
- checkpoint evidence freeze
- checkpoint invalidation
- checkpoint rollback
- checkpoint superseded
- checkpoint audit trail
```

execution checkpoint model は以下を行わない。

```text
- incomplete evidence で checkpoint pass しない
- stale checkpoint を active checkpoint として扱わない
- checkpoint projection を source of truth として扱わない
- rollback impossible state を completion checkpoint としない
```

---

## 3. checkpoint positioning

checkpoint は以下に属する。

```text
Execution Governance Layer
Operational Orchestration Layer
```

checkpoint 自体は source artifact ではない。

---

## 4. checkpoint kinds

checkpoint kinds：

```text
validator_checkpoint
comparison_checkpoint
cleanup_checkpoint
authorization_checkpoint
execution_checkpoint
post_validation_checkpoint
completion_checkpoint
rollback_checkpoint
```

---

## 5. checkpoint lifecycle

checkpoint lifecycle：

```text
checkpoint_pending
checkpoint_review_required
checkpoint_passed
checkpoint_blocked
checkpoint_invalidated
checkpoint_superseded
checkpoint_rolled_back
checkpoint_archived
```

checkpoint_passed は reconstruction_completed を意味しない。

---

## 6. checkpoint transition

標準 transition：

```text
validator_checkpoint
↓
comparison_checkpoint
↓
cleanup_checkpoint
↓
authorization_checkpoint
↓
execution_checkpoint
↓
post_validation_checkpoint
↓
completion_checkpoint
```

rollback_required の場合：

```text
execution_checkpoint
↓
rollback_checkpoint
↓
comparison_checkpoint or cleanup_checkpoint
```

へ戻りうる。

---

## 7. checkpoint transition rules

transition 条件：

```text
- required previous checkpoint passed
- checkpoint evidence frozen
- required validators passed
- required blockers resolved
- orchestration stage consistent
```

transition fail の場合、checkpoint_blocked とする。

---

## 8. validator checkpoint

validator_checkpoint 条件：

```text
- required validator reports generated
- required fail absent
- required blocked absent
- source refs resolved
```

validator taxonomy changed の場合 invalidated されうる。

---

## 9. comparison checkpoint

comparison_checkpoint 条件：

```text
- semantic comparison completed
- traceability comparison completed
- orphan detection completed
- contamination detection completed
- comparison evidence frozen
```

comparison incomplete のまま cleanup_checkpoint へ進めない。

---

## 10. cleanup checkpoint

cleanup_checkpoint 条件：

```text
- cleanup blockers classified
- rollback recoverability verified
- cleanup scope frozen
- cleanup_hold resolved
- federation cleanup safety reviewed
```

cleanup_ready は authorization_checkpoint を意味しない。

---

## 11. authorization checkpoint

authorization_checkpoint 条件：

```text
- authorization report generated
- irreversible cleanup review completed where applicable
- governance approval recorded
- execution scope frozen
```

authorization superseded の場合 invalidated。

---

## 12. execution checkpoint

execution_checkpoint 条件：

```text
- execution plan expanded
- execution units validated
- rollback execution order validated
- pre-execution snapshot generated
```

execution failure の場合 rollback_checkpoint へ遷移しうる。

---

## 13. post-validation checkpoint

post_validation_checkpoint 条件：

```text
- reference integrity confirmed
- traceability integrity confirmed
- provenance integrity confirmed where applicable
- no stale active artifact
- projection regenerated
```

post-validation fail の場合 completion_checkpoint へ進めない。

---

## 14. completion checkpoint

completion_checkpoint 条件：

```text
- legacy detachment completed
- cleanup completed where required
- no unresolved completion blocker
- completion evidence frozen
- audit trail completed
```

completion_checkpoint は final archive 前でもよい。

---

## 15. rollback checkpoint

rollback_checkpoint 条件：

```text
- rollback required declared
- rollback execution scope frozen
- rollback restore point verified
- rollback execution report generated
```

rollback_failed の場合 manual_recovery_required。

---

## 16. checkpoint evidence freeze

checkpoint 到達時に evidence freeze を行う。

対象：

```text
- validator reports
- comparison reports
- cleanup reports
- provenance reports
- authorization reports
- execution reports
- completion reports
```

freeze 後の変更は invalidation trigger になりうる。

---

## 17. checkpoint invalidation

以下は checkpoint invalidation を発生させる。

```text
- validator taxonomy changed
- comparison criteria changed
- provenance graph changed
- cleanup scope changed
- rollback semantics changed
- authorization scope changed
- completion blocker newly detected
```

invalidated checkpoint は active checkpoint として扱わない。

---

## 18. checkpoint superseded

以下は checkpoint superseded を発生させる。

```text
- newer checkpoint evidence exists
- rerun generated newer comparison evidence
- cleanup rerun generated newer cleanup evidence
- orchestration rerun generated newer execution evidence
```

superseded checkpoint を authorization source に使用してはならない。

---

## 19. checkpoint rollback

checkpoint rollback は previous checkpoint state へ戻す。

例：

```text
completion_checkpoint
↓ rollback
comparison_checkpoint
```

rollback 条件：

```text
- cleanup failure
- provenance corruption
- stale active artifact detected
- invalid execution scope detected
- unresolved completion blocker detected
```

---

## 20. checkpoint promotion criteria

checkpoint progression 条件：

```text
- required previous checkpoint passed
- no unresolved critical blocker
- checkpoint evidence frozen
- required orchestration stage completed
- required reports regenerated if stale
```

promotion criteria fail は checkpoint_review_required または blocked。

---

## 21. checkpoint audit trail

checkpoint operation を audit する。

記録：

```text
- checkpoint kind
- checkpoint status
- promotion reason
- invalidation reason
- rollback reason
- evidence refs
- orchestration stage refs
```

checkpoint audit missing は governance warning または blocker。

---

## 22. orchestration relation

execution checkpoint model は federation execution orchestration と接続される。

対象：

```text
- stage progression
- grouped rerun
- cleanup execution gating
- completion gating
- rollback orchestration
```

checkpoint progression は orchestration stage consistency を要求する。

---

## 23. projection relation

Dashboard projection は checkpoint projection を表示できる。

ただし：

```text
checkpoint projection
```

は：

```text
checkpoint evidence
```

ではない。

stale checkpoint projection を active authorization source に使用してはならない。

---

## 24. checkpoint report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "execution_checkpoint_report",
  "checkpoint_kind": "comparison_checkpoint",
  "checkpoint_status": "checkpoint_passed",
  "checkpoint_invalidated": false,
  "checkpoint_superseded": false,
  "evidence_refs": [],
  "blocked_reasons": []
}
```

---

## 25. CI mapping

CI fail 条件：

```text
- checkpoint progression without evidence freeze
- stale checkpoint used as authorization source
- invalidated checkpoint treated as passed
- completion checkpoint without audit trail
- rollback checkpoint missing after rollback_required
```

CI warn 条件：

```text
- checkpoint review required
- checkpoint projection stale
- superseded checkpoint retained outside archive scope
```

---

## 26. 禁止事項

以下を禁止する。

```text
- incomplete comparison で cleanup checkpoint へ進むこと
- stale checkpoint を active checkpoint と扱うこと
- invalidated checkpoint を pass 扱いすること
- checkpoint projection を source of truth と扱うこと
- rollback impossible state を completion checkpoint とすること
```

---

## 27. HLDocS feedback

本 refinement から、HLDocS 側へ以下をフィードバック候補とする。

```text
- execution checkpoint lifecycle を formalize すべき
- checkpoint evidence freeze を formal artifact 化すべき
- checkpoint invalidation / superseded lifecycle を持つべき
- checkpoint rollback model を formalize すべき
- orchestration stage と checkpoint progression を接続すべき
```

---

## 28. 結論

execution checkpoint model refinement は、SansaVRM 再構成における checkpoint-driven execution governance を定義する refinement である。

これにより、checkpoint transition、evidence freeze、rollback、invalidation、audit trail を通じて、安全に orchestration progression を管理できる。
