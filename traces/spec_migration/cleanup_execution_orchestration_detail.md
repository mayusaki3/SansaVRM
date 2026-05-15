# cleanup execution orchestration detail

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における cleanup execution orchestration detail を定義する。

cleanup execution orchestration detail は、cleanup readiness / authorization 後に cleanup execution をどの順序で行い、失敗時にどう停止・rollback・post-validation へ接続するかを整理する。

---

## 2. 基本方針

cleanup execution orchestration は以下を扱う。

```text
- cleanup execution scope freeze
- execution authorization verification
- cleanup execution ordering
- cleanup failure handling
- cleanup rollback orchestration
- post-cleanup validation
- artifact lifecycle update
- cleanup audit trail
```

cleanup execution orchestration は以下を行わない。

```text
- cleanup_ready だけで cleanup execution を開始しない
- authorization missing の cleanup を実行しない
- rollback scope missing の cleanup を実行しない
- irreversible cleanup を unattended batch execution しない
- dashboard projection を authorization source として扱わない
```

---

## 3. cleanup execution stages

cleanup execution stages は以下とする。

```text
X1 authorization verification
X2 cleanup scope freeze verification
X3 rollback recoverability verification
X4 pre-cleanup snapshot
X5 cleanup execution plan expansion
X6 cleanup execution
X7 cleanup failure handling
X8 cleanup rollback where required
X9 post-cleanup validation
X10 artifact lifecycle update
X11 cleanup audit finalization
```

---

## 4. X1 authorization verification

確認：

```text
- cleanup_authorization_report exists
- cleanup_execution_authorized=true
- cleanup_review_required resolved
- irreversible cleanup review completed where applicable
- governance approval recorded
```

Fail 条件：

```text
- authorization missing
- authorization superseded
- cleanup_review_required unresolved
- irreversible cleanup without review
```

---

## 5. X2 cleanup scope freeze verification

確認：

```text
- cleanup target list frozen
- affected reference list frozen
- affected alias list frozen
- affected provenance edge list frozen
- affected federation dependency list frozen
```

Fail 条件：

```text
- cleanup scope changed after authorization
- cleanup target list missing
- affected provenance edge unknown
- affected alias unknown
```

---

## 6. X3 rollback recoverability verification

確認：

```text
- restore point exists
- before hash set exists
- rollback execution order exists
- alias rollback exists where applicable
- provenance rollback exists where applicable
- federation rollback notification path exists where applicable
```

Fail 条件：

```text
- rollback scope missing
- rollback restore point missing
- irreversible cleanup without fallback plan
- rollback owner missing
```

---

## 7. X4 pre-cleanup snapshot

cleanup 前に snapshot を取得する。

対象：

```text
- cleanup target files
- affected indexes
- affected registries
- alias registry
- provenance graph registry
- dashboard/report artifact references
```

Snapshot は rollback と audit に使用する。

---

## 8. X5 cleanup execution plan expansion

cleanup plan を実行単位へ展開する。

実行単位：

```text
- file removal
- registry entry deactivation
- obsolete artifact marking
- alias expiration
- stale report archival
- projection cache cleanup
```

各 execution unit は以下を持つ。

```text
- unit_id
- operation_kind
- target
- reversible
- rollback_action
- evidence_refs
```

---

## 9. X6 cleanup execution

cleanup execution は plan 順に実行する。

標準順序：

```text
1. projection/cache cleanup
2. stale report archival
3. obsolete artifact marking
4. registry entry deactivation
5. alias expiration
6. file removal
```

irreversible operation は authorization と review evidence を再確認してから実行する。

---

## 10. X7 cleanup failure handling

failure 発生時：

```text
- cleanup execution を停止する
- failure unit を記録する
- executed units を記録する
- rollback_required を判定する
- affected artifacts を stale にする
- dashboard を stale にする
```

cleanup failure を cleanup_completed として扱ってはならない。

---

## 11. X8 cleanup rollback where required

rollback_required の場合：

```text
- rollback execution order に従う
- registry を復元する
- alias を復元する
- provenance edge を復元する
- file を復元する
- rollback report を生成する
```

rollback_failed の場合、manual_recovery_required とする。

---

## 12. X9 post-cleanup validation

cleanup 後に検証する。

検証対象：

```text
- reference integrity
- traceability integrity
- provenance graph integrity
- alias integrity
- external artifact dependency
- federation dependency
- dashboard projection regeneration
```

post-cleanup validation fail は reconstruction_completed を block する。

---

## 13. X10 artifact lifecycle update

cleanup 後に artifact lifecycle を更新する。

例：

```text
cleanup target → archived / removed / obsolete
validator report → stale or superseded
cleanup execution report → generated
post-validation report → generated
dashboard snapshot → regenerated
```

superseded artifact を active evidence として残してはならない。

---

## 14. X11 cleanup audit finalization

cleanup audit trail を確定する。

保持する evidence：

```text
- cleanup authorization report
- cleanup execution report
- rollback report where applicable
- post-cleanup validation report
- artifact lifecycle update report
- completion review report reference
```

cleanup audit missing は completion blocker とする。

---

## 15. cleanup execution report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "cleanup_execution_report",
  "cleanup_execution_id": "cleanup-YYYYMMDD-NNN",
  "cleanup_execution_status": "failed",
  "authorized": true,
  "scope_frozen": true,
  "rollback_recoverability_confirmed": true,
  "executed_units": [],
  "failed_units": [],
  "rollback_required": false,
  "source_of_truth_refs": []
}
```

---

## 16. cleanup execution status

cleanup_execution_status は以下とする。

```text
planned
authorized
in_progress
completed
completed_with_warning
failed
rollback_required
rolled_back
rollback_failed
manual_recovery_required
superseded
```

---

## 17. execution unit status

execution unit status は以下とする。

```text
pending
executed
skipped
failed
rolled_back
rollback_failed
```

---

## 18. CI mapping

CI fail 条件：

```text
- cleanup execution without authorization
- cleanup execution without frozen scope
- cleanup execution without rollback recoverability
- irreversible unit executed without review evidence
- cleanup failed but marked completed
- post-cleanup validation failed
- cleanup audit missing
```

CI warn 条件：

```text
- completed_with_warning
- skipped optional cleanup unit
- post-cleanup dashboard regeneration pending
```

---

## 19. dashboard display

Dashboard は cleanup execution status を表示する。

表示対象：

```text
- cleanup execution status
- authorization status
- frozen scope status
- rollback readiness
- execution unit summary
- failed unit summary
- post-validation status
- audit finalization status
```

Dashboard は cleanup execution を実行しない。

---

## 20. 禁止事項

以下を禁止する。

```text
- cleanup_ready のみで cleanup execution を開始すること
- authorization superseded のまま cleanup execution すること
- rollback scope missing のまま cleanup execution すること
- irreversible operation を review evidence なしで実行すること
- cleanup failure を cleanup completed と扱うこと
- post-cleanup validation なしで reconstruction completed と扱うこと
```

---

## 21. HLDocS feedback

本 detail から、HLDocS 側へ以下をフィードバック候補とする。

```text
- cleanup execution は readiness / authorization / execution / post-validation / audit に分けるべき
- cleanup scope freeze と rollback recoverability を execution 前 gate にすべき
- cleanup execution unit を evidence refs 付きで管理すべき
- cleanup failure と rollback_required を formal state にすべき
- post-cleanup validation を completion 条件に含めるべき
```

---

## 22. 結論

cleanup execution orchestration detail は、cleanup readiness 後の実 cleanup execution を安全に行うための orchestration detail である。

これにより、authorization、scope freeze、rollback recoverability、execution unit、failure handling、post-validation、audit finalization を経て cleanup execution を安全に reconstruction completion へ接続できる。
