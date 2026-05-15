# operational batching governance

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における operational batching governance を定義する。

operational batching governance は、M2 operational reconstruction / M3 production reconstruction において、どの作業を batch execution 可能とし、どの作業を checkpoint-only confirmation または manual governance 対象とするかを整理する。

---

## 2. 基本方針

operational batching governance は以下を扱う。

```text
- batch execution scope
- non-batchable governance
- checkpoint model
- summarized reporting
- batch rollback
- batch invalidation
- maturity-aware execution policy
- operational orchestration
```

operational batching governance は以下を行わない。

```text
- unresolved blocker を batch continuation しない
- irreversible cleanup を unattended execution しない
- mixed reconstruction overwrite を silent execution しない
- dashboard を source of truth として扱わない
```

---

## 3. maturity-aware execution policy

execution policy は reconstruction maturity に依存する。

```text
M0 exploratory reconstruction
M1 structured reconstruction
M2 operational reconstruction
M3 production reconstruction
```

---

## 4. M0 exploratory reconstruction

特徴：

```text
- governance unstable
- terminology unstable
- validator taxonomy unstable
- lifecycle unstable
- cleanup semantics unstable
```

必要運用：

```text
- micro-step validation
- concept-by-concept confirmation
- manual checkpointing
- rollback-heavy governance
```

batch execution は原則禁止。

---

## 5. M1 structured reconstruction

特徴：

```text
- phase structure mostly stable
- validator taxonomy mostly stable
- lifecycle semantics mostly stable
- rollback semantics mostly stable
```

許可候補：

```text
- phase-level batching
- grouped validator execution
- grouped registry regeneration
- grouped comparison rerun
```

ただし cleanup authorization は manual review を維持する。

---

## 6. M2 operational reconstruction

特徴：

```text
- governance stabilized
- validator automation available
- propagation engine available
- cleanup governance stabilized
- coexistence governance stabilized
```

許可候補：

```text
- work-package batching
- checkpoint-only confirmation
- summarized reporting
- grouped rerun handling
- grouped cleanup candidate generation
```

---

## 7. M3 production reconstruction

特徴：

```text
- operational governance stabilized
- federation orchestration stabilized
- cleanup execution stabilized
- rollback orchestration stabilized
```

許可候補：

```text
- orchestration-driven execution
- approval checkpoint workflow
- automated propagation rerun
- operational dashboard review
```

ただし irreversible cleanup は review-required を維持する。

---

## 8. batchable execution scope

batch execution 候補：

```text
- validator rerun
- registry regeneration
- comparison rerun
- stale artifact propagation
- cleanup candidate regeneration
- dashboard projection regeneration
- report regeneration
```

これらは grouped execution を許可できる。

---

## 9. non-batchable governance

non-batchable 候補：

```text
- irreversible cleanup
- mixed reconstruction overwrite
- provenance conflict resolution
- restriction conflict resolution
- federation-wide authorization
- rollback boundary change
- source_of_truth change
```

これらは checkpoint-only または manual governance を要求する。

---

## 10. checkpoint model

checkpoint は batch execution の停止点である。

checkpoint 種類：

```text
validator_checkpoint
comparison_checkpoint
cleanup_checkpoint
authorization_checkpoint
completion_checkpoint
rollback_checkpoint
```

checkpoint は summarized report を伴う。

---

## 11. checkpoint-only confirmation

M2/M3 では micro-step confirmation を減らす。

代わりに：

```text
- phase completion
- work package completion
- blocker summary
- cleanup summary
- rerun summary
```

単位で確認する。

---

## 12. summarized reporting

Operational mode では per-document report を減らす。

summary 対象：

```text
- phase progress
- blocker summary
- rerun summary
- cleanup summary
- provenance issue summary
- restriction conflict summary
- rollback summary
```

summarized reporting は source of truth の代替ではない。

---

## 13. batch invalidation

以下は batch invalidation を発生させる。

```text
- validator taxonomy changed
- reconstruction delta changed
- provenance graph changed
- restriction merge criteria changed
- cleanup scope changed
- rollback boundary changed
- federation dependency changed
```

batch invalidation 発生時は rerun_required とする。

---

## 14. grouped rerun

grouped rerun 候補：

```text
- validator rerun
- comparison rerun
- cleanup candidate rerun
- dashboard projection rerun
- provenance validation rerun
```

ただし authorization review は grouped rerun に含めない。

---

## 15. batch rollback

batch execution は rollback scope を持つ。

必要：

```text
- batch restore point
- batch before hash set
- rollback execution order
- rollback invalidation handling
- stale artifact propagation rollback
```

rollback scope 不明の batch execution は許可しない。

---

## 16. cleanup batching

cleanup batching は限定的に許可する。

batchable cleanup 候補：

```text
- obsolete temporary artifact cleanup
- stale report cleanup
- regenerated projection cleanup
- non-canonical cache cleanup
```

non-batchable cleanup 候補：

```text
- provenance edge removal
- irreversible alias expiration
- active canonical artifact removal
- mixed reconstruction overwrite cleanup
```

---

## 17. provenance batching

provenance validation は grouped execution を許可できる。

ただし以下は manual review を維持する。

```text
- restriction conflict resolution
- editor attribution ambiguity
- temporary provenance bridge resolution
- distribution-ready authorization
```

---

## 18. federation orchestration relation

operational batching governance は federation execution orchestration と接続される。

特に：

```text
- stage grouping
- grouped rerun
- checkpoint progression
- authorization boundary
- completion review grouping
```

を orchestration policy に反映する。

---

## 19. batch execution report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "batch_execution_report",
  "batch_execution_scope": [],
  "batch_status": "rerun_required",
  "checkpoint_status": "comparison_checkpoint",
  "blocked_reasons": [],
  "rollback_scope_defined": false
}
```

---

## 20. dashboard display

Dashboard は operational batching status を表示する。

表示対象：

```text
- current maturity
- batch execution status
- checkpoint status
- grouped rerun status
- blocker summary
- rollback readiness
- cleanup batching status
```

Dashboard は batch authorization を独自決定しない。

---

## 21. CI mapping

CI fail 条件：

```text
- irreversible cleanup executed without checkpoint
- unresolved blocker ignored in batch execution
- rollback scope undefined in batch execution
- authorization review grouped into unattended execution
- mixed reconstruction overwrite executed as batch-safe
```

CI warn 条件：

```text
- grouped rerun pending
- checkpoint review required
- summarized reporting stale
```

---

## 22. 禁止事項

以下を禁止する。

```text
- M0 governance で large batch execution を行うこと
- irreversible cleanup を unattended batch execution すること
- restriction conflict resolution を automatic batch resolution すること
- checkpoint を飛ばして completion へ進むこと
- summarized reporting を source of truth と扱うこと
```

---

## 23. HLDocS feedback

本 governance から、HLDocS 側へ以下をフィードバック候補とする。

```text
- reconstruction maturity-aware execution policy が必要
- batchable governance と non-batchable governance を分離すべき
- checkpoint-only confirmation model を formalize すべき
- summarized operational reporting を formal artifact 化すべき
- batch invalidation / grouped rerun governance を扱うべき
```

---

## 24. 結論

operational batching governance は、SansaVRM 再構成における operational / production reconstruction の batch execution policy を定義する governance である。

これにより、M2/M3 では summarized reporting と checkpoint-only confirmation を利用しつつ、irreversible cleanup や provenance conflict resolution のような high-risk governance を manual review 下へ維持できる。
