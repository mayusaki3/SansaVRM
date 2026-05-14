# federation MVP execution lifecycle specification

## 1. 目的

本ドキュメントは、SansaVRM federation MVP における execution lifecycle を定義する。

本 lifecycle は Preview Federation MVP の範囲を対象とし、dry-run、validation、rerun、retry、abort、superseded execution を扱う。

本仕様は Production Federation の execution protocol ではない。

---

## 2. 基本方針

MVP execution lifecycle は以下を扱う。

```text
- dry-run execution
- validator execution
- dashboard projection execution
- CI dry-run execution
- rerun_required
- retry_allowed / retry_blocked
- abort / superseded
- execution artifact lifecycle との接続
```

MVP execution lifecycle は以下を行わない。

```text
- canonicalization apply を実行しない
- cleanup execution を実行しない
- downstream repository を変更しない
- production rollback を実行しない
- governance approval を自動化しない
```

---

## 3. execution kinds

MVP で扱う execution_kind は以下とする。

```text
index_build_execution
validator_execution
dashboard_projection_execution
ci_dry_run_execution
artifact_review_execution
acceptance_review_execution
```

MVP では以下を扱わない。

```text
apply_execution
cleanup_execution
downstream_execution
production_rollback_execution
```

---

## 4. execution state

execution_state は以下とする。

```text
planned
running
completed
completed_with_warning
failed
blocked
rerun_required
retry_allowed
retry_blocked
aborted
superseded
```

---

## 5. dry-run execution lifecycle

Dry-run execution の基本 lifecycle：

```text
planned
  ↓
running
  ↓
completed / completed_with_warning / failed / blocked
```

Dry-run は repository source を変更してはならない。

Dry-run は generated artifacts を出力できる。

---

## 6. validator execution lifecycle

Validator execution の lifecycle：

```text
planned
  ↓
running
  ↓
completed / completed_with_warning / failed / blocked
  ↓
rerun_required / superseded
```

validator execution は validator report を生成する。

validator execution は cleanup_ready を直接実行しない。

cleanup gate validator のみが cleanup_ready / cleanup_blocked / cleanup_pending を report として出力できる。

---

## 7. dashboard projection execution lifecycle

Dashboard projection execution の lifecycle：

```text
planned
  ↓
running
  ↓
completed / failed / blocked
  ↓
stale / superseded
```

Dashboard projection execution は source of truth を変更しない。

Dashboard artifact が stale / superseded になっても、validator result そのものは変更されない。

---

## 8. CI dry-run execution lifecycle

CI dry-run execution は以下を実行する。

```text
1. index build
2. validator execution
3. dashboard projection
4. artifact upload
```

CI execution state：

```text
planned
running
completed
completed_with_warning
failed
blocked
```

CI は destructive operation を実行しない。

---

## 9. rerun_required condition

rerun_required になる条件：

```text
- input registry changed
- schema changed
- reason taxonomy changed
- validator implementation changed
- strict mode changed
- reconstruction delta detected
- external artifact freshness changed
- source_of_truth_refs changed
```

rerun_required の artifact を active pass として扱ってはならない。

---

## 10. retry policy

retry_allowed 条件：

```text
- execution failed due to transient script error
- generated artifact missing but source input unchanged
- dashboard projection failed due to output directory missing
- CI artifact upload failed after local reports were generated
```

retry_blocked 条件：

```text
- source input changed during execution
- schema changed during execution
- reconstruction delta detected during execution
- strict mode failure detected
- unknown-as-pass detected
- skeleton-as-pass detected
```

retry_blocked の場合は rerun planning または acceptance rejection が必要である。

---

## 11. abort policy

abort が必要な条件：

```text
- destructive operation が検出された
- source file mutation が検出された
- downstream repository modification が検出された
- dashboard attempted source mutation
- generated artifact が source file として扱われた
```

abort 後は execution report を生成し、manual review を要求する。

---

## 12. superseded execution

execution は以下の場合 superseded となる。

```text
- replacement execution が生成された
- reconstruction delta により current execution が無効化された
- artifact lifecycle が superseded へ遷移した
- bootstrap schema が更新された
- acceptance criteria が更新された
```

superseded execution を active evidence として使ってはならない。

---

## 13. execution report structure

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "execution_report",
  "preview_only": true,
  "dry_run_only": true,
  "execution_id": "execution-YYYYMMDD-NNN",
  "execution_kind": "validator_execution",
  "execution_state": "completed_with_warning",
  "generated_at": "YYYY-MM-DDTHH:MM:SSZ",
  "source_of_truth_refs": [],
  "inputs": [],
  "outputs": [],
  "findings": []
}
```

---

## 14. execution finding reasons

execution 固有 reason code：

```text
execution_input_changed
execution_schema_changed
execution_delta_detected
execution_source_mutation_detected
execution_destructive_operation_detected
execution_output_missing
execution_retry_allowed
execution_retry_blocked
execution_superseded
execution_abort_required
```

---

## 15. artifact lifecycle との接続

execution は artifact lifecycle と接続する。

```text
execution completed:
artifact generated

execution completed_with_warning:
artifact generated + finding warn

execution failed:
artifact missing or invalid

execution superseded:
artifact superseded

execution rerun_required:
artifact stale
```

---

## 16. CI mapping

CI fail 条件：

```text
- execution failed
- execution blocked
- retry_blocked
- abort_required
- source mutation detected
- destructive operation detected
- superseded execution used as active
```

CI warn 条件：

```text
- completed_with_warning
- retry_allowed
- optional artifact upload failed
- rerun_required outside cleanup scope
```

---

## 17. dashboard display

Dashboard は execution lifecycle を表示する。

表示対象：

```text
- execution_id
- execution_kind
- execution_state
- generated artifacts
- rerun_required reason
- retry state
- superseded state
- abort reason
```

Dashboard は execution state を変更しない。

---

## 18. 禁止事項

以下を禁止する。

```text
- dry-run execution を apply execution として扱うこと
- completed_with_warning を production-ready とみなすこと
- rerun_required execution を active pass として扱うこと
- superseded execution を acceptance evidence として使うこと
- abort_required を warning のみで済ませること
- MVP execution で cleanup / apply / downstream modification を行うこと
```

---

## 19. HLDocS feedback

本 lifecycle から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction MVP には execution lifecycle が必要
- dry-run / apply / cleanup execution を分離すべき
- rerun_required / retry_allowed / retry_blocked / superseded を明示状態にすべき
- source mutation / destructive operation detection は abort 条件にすべき
- execution artifact と artifact lifecycle を接続すべき
```

---

## 20. 結論

federation MVP execution lifecycle specification は、Preview Federation MVP における dry-run、validator、dashboard、CI execution の状態遷移を定義する仕様である。

これにより、MVP 実装を destructive operation なしに運用しつつ、rerun、retry、abort、superseded execution を明確に扱える。
