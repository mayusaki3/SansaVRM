# federation state machine

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における federation state machine を定義する。

federation state machine は、migration / canonicalization / rewrite / validation / cleanup / reconstruction delta / federation execution の状態を統合し、許可遷移・禁止遷移・再評価条件を整理する。

---

## 2. 基本方針

federation state machine は以下を行う。

```text
- project-local state と federation state を分離する
- validator state と executor state を分離する
- cleanup_ready と federation_cleanup_ready を分離する
- reconstruction delta による invalidation を扱う
- superseded / rollback_required / rerun_required を統合する
- dashboard projection の基準状態を提供する
```

federation state machine は以下を行わない。

```text
- validator result を生成しない
- execution を実行しない
- cleanup を実行しない
- project-local approval を代替しない
- handoff contract を自動承認しない
```

---

## 3. state domain

state domain は以下に分離する。

```text
migration_state
canonicalization_state
rewrite_state
validation_state
cleanup_state
reconstruction_delta_state
federation_validation_state
federation_orchestration_state
federation_execution_state
federation_cleanup_state
```

1つの対象は複数 domain の state を同時に持つ。

---

## 4. project-local lifecycle

project-local lifecycle は以下を基本とする。

```text
planned
  ↓
relocated
  ↓
fate_decided
  ↓
canonicalized
  ↓
rewrite_validated
  ↓
cleanup_ready
  ↓
cleanup_completed
```

ただし、これは単純化した happy path であり、実際には validator / execution / delta により blocked / superseded / rollback_required へ遷移する。

---

## 5. federation lifecycle

federation lifecycle は以下を基本とする。

```text
federation_planned
  ↓
federation_validated
  ↓
federation_orchestration_ready
  ↓
federation_execution_ready
  ↓
federation_execution_completed
  ↓
federation_cleanup_ready
  ↓
federation_cleanup_completed
```

project-local cleanup_ready は federation_cleanup_ready を意味しない。

---

## 6. reconstruction delta lifecycle

reconstruction delta lifecycle は以下とする。

```text
delta_detected
  ↓
impact_analyzed
  ↓
rerun_required
  ↓
replacement_execution_planned
  ↓
validated_after_delta
  ↓
delta_resolved
```

freeze 後 delta は current execution を superseded にする。

---

## 7. federation execution lifecycle

federation execution lifecycle は federation execution protocol に従う。

```text
proposed
  ↓
acknowledgement_pending
  ↓
preflight_running
  ↓
checkpoint_ready
  ↓
dry_run_completed
  ↓
approval_required
  ↓
approved
  ↓
local_execution_running
  ↓
local_validation_running
  ↓
federation_validation_running
  ↓
completed
```

途中で fail / delta / rollback が発生した場合は、failed / rollback_required / superseded へ遷移する。

---

## 8. cleanup state separation

cleanup state は project-local と federation に分ける。

```text
project cleanup_state:
cleanup_ready / cleanup_blocked / cleanup_pending / cleanup_completed

federation cleanup_state:
federation_cleanup_ready / federation_cleanup_blocked / federation_cleanup_pending / federation_cleanup_completed
```

project-local cleanup_ready であっても、以下がある場合は federation_cleanup_blocked とする。

```text
- downstream project が old schema を消費している
- stale artifact が残っている
- handoff contract pending
- reconstruction delta unresolved
- federation rollback boundary missing
```

---

## 9. allowed transitions

主な許可遷移：

```text
planned → relocated
relocated → fate_decided
fate_decided → canonicalized
canonicalized → rewrite_planned
rewrite_planned → rewrite_executed
rewrite_executed → rewrite_validated
rewrite_validated → cleanup_ready
cleanup_ready → cleanup_completed
```

federation 側：

```text
federation_validated → federation_orchestration_ready
federation_orchestration_ready → federation_execution_ready
federation_execution_ready → federation_execution_completed
federation_execution_completed → federation_cleanup_ready
```

---

## 10. blocked transitions

以下は blocked に遷移する。

```text
manifest fail → migration_blocked
pending document_fate → canonicalization_blocked
invalid canonical conflict → canonicalization_blocked
rewrite failed → rewrite_blocked
rewrite executed but not validated → cleanup_blocked
unresolved reference → cleanup_blocked
stale required artifact → federation_cleanup_blocked
handoff pending → federation_cleanup_blocked
```

---

## 11. forbidden transitions

以下の遷移は禁止する。

```text
pending document_fate → cleanup_ready
canonicalized → cleanup_ready without rewrite_validated
rewrite_executed → cleanup_ready
cleanup_ready → federation_cleanup_ready without federation validator pass
federation_execution_proposed → local_execution_running without acknowledgement
approved → completed without project-local validation
superseded → cleanup_ready
semantic_delta_detected → cleanup_ready without rerun
```

---

## 12. superseded handling

superseded は、過去の状態が現在の判断に使えないことを表す。

superseded になる条件：

```text
- freeze 後 reconstruction delta
- schema drift affecting execution scope
- validation rule change
- cleanup gate rule change
- cross-project artifact dependency change
- external artifact freshness invalidation
```

superseded state は cleanup_ready / federation_cleanup_ready の判定に使用してはならない。

---

## 13. rerun_required handling

rerun_required は、validator または federation validator の再実行が必要な状態である。

rerun_required になる条件：

```text
- reconstruction delta detected
- validator module version changed
- graph hash changed
- dependency hash changed
- external artifact hash changed
- cross-project handoff response changed
```

rerun_required のまま cleanup_ready へ進めてはならない。

---

## 14. rollback_required handling

rollback_required は、execution 結果を維持できない可能性がある状態である。

発生条件：

```text
- project-local validation fail
- federation validator fail after execution
- unexpected execution diff
- approval scope violation
- downstream artifact incompatibility
```

rollback_required は project-local rollback と federation rollback coordination を分離して扱う。

---

## 15. dashboard projection

Dashboard は state machine の projection を表示する。

表示対象：

```text
- current state domain values
- allowed next transitions
- blocked transitions
- forbidden transition violation
- superseded reason
- rerun_required reason
- rollback_required reason
```

Dashboard は state transition を実行しない。

---

## 16. report schema draft

```json
{
  "schema_version": "1.0",
  "state_machine_snapshot_id": "state-machine-YYYYMMDD-NNN",
  "target_id": "target-example",
  "state_domains": {
    "migration_state": "migration_verified",
    "canonicalization_state": "completed",
    "rewrite_state": "rewrite_validated",
    "cleanup_state": "cleanup_ready",
    "federation_cleanup_state": "federation_cleanup_blocked"
  },
  "blocked_reasons": [
    "handoff_contract_pending"
  ],
  "allowed_next_transitions": [],
  "forbidden_transition_violations": []
}
```

---

## 17. CI mapping

CI fail 条件：

```text
- forbidden transition violation
- cleanup_ready with rewrite_state != rewrite_validated
- federation_cleanup_ready with federation validator fail
- superseded execution used as active
- rerun_required ignored
- rollback_required ignored
```

CI warn 条件：

```text
- cleanup_pending exists
- federation_cleanup_pending exists
- rerun_required outside cleanup scope
- stale optional artifact
```

---

## 18. 禁止事項

以下を禁止する。

```text
- project-local cleanup_ready を federation_cleanup_ready とみなすこと
- superseded execution を active execution として扱うこと
- rerun_required を無視して cleanup へ進むこと
- rollback_required を warning のみで処理すること
- dashboard projection を state source of truth として扱うこと
```

---

## 19. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction / federation / cleanup は state domain を分離すべき
- project-local cleanup_ready と federation_cleanup_ready を分離すべき
- superseded / rerun_required / rollback_required を共通状態として扱うべき
- forbidden transition を validator / CI で検出すべき
- dashboard は state machine projection として扱うべき
```

---

## 20. 結論

federation state machine は、SansaVRM の大規模仕様再構成における project-local state と federation state を統合管理する状態遷移モデルである。

これにより、validator、execution、cleanup、reconstruction delta、cross-project federation を矛盾なく接続できる。
