# federation execution protocol

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における federation execution protocol を定義する。

federation execution protocol は、federation orchestration engine が生成した coordination plan を、複数 project 間で安全に実行するための handshake / approval / checkpoint / rollback / completion protocol である。

---

## 2. 基本方針

federation execution protocol は以下を扱う。

```text
- cross-project execution handshake
- distributed approval boundary
- execution token
- distributed checkpoint
- downstream action request execution
- federation validator rerun confirmation
- distributed rollback coordination
- completion acknowledgement
```

federation execution protocol は以下を行わない。

```text
- 他 project の repository を無断で変更しない
- handoff contract を自動承認しない
- stale artifact を fresh として扱わない
- project local validator を代替しない
- project local cleanup gate を迂回しない
```

---

## 3. protocol lifecycle

federation execution protocol の lifecycle は以下とする。

```text
1. execution proposal
2. participant acknowledgement
3. distributed preflight
4. distributed checkpoint
5. distributed dry-run
6. approval boundary
7. project-local execution
8. project-local validation
9. federation validator rerun
10. federation completion acknowledgement
11. federation dashboard projection
```

---

## 4. participant

participant は federation execution に関与する project である。

例：

```text
- SansaVRM
- SansaVRM-MuJoCo-Adapter
- SansaVRM Studio AI
- future adapter project
- future export pipeline project
```

participant は project-local execution authority を持つ。

federation execution protocol は participant に action request を出すが、participant の repository を直接変更しない。

---

## 5. execution proposal

execution proposal は federation orchestration engine が生成する。

含める情報：

```text
- federation_execution_id
- federation_orchestration_id
- coordination_scope
- affected_projects
- downstream_action_requests
- required_validators
- required_artifacts
- approval_policy
- rollback_policy
```

---

## 6. participant acknowledgement

各 participant は execution proposal に対して acknowledgement を返す。

acknowledgement status：

```text
accepted
accepted_with_warning
rejected
needs_clarification
not_applicable
```

`accepted` でない participant が required scope に含まれる場合、federation execution は apply に進めない。

---

## 7. distributed preflight

distributed preflight は、各 participant の local preflight result を集約する。

確認項目：

```text
- local validator pass / warn / fail
- required artifact freshness
- schema compatibility
- handoff contract acceptance
- local rollback availability
- local cleanup gate state
```

required participant の preflight が fail / missing の場合、federation execution は blocked とする。

---

## 8. distributed checkpoint

distributed checkpoint は、各 participant の local checkpoint reference を集約する。

含める情報：

```text
- participant project id
- repository revision
- artifact hash
- validator report id
- local checkpoint id
- rollback package reference
```

federation-level checkpoint は、local checkpoint の集合である。

各 project の checkpoint content は各 project が保持する。

---

## 9. distributed dry-run

distributed dry-run は、各 participant の local dry-run result を集約する。

確認項目：

```text
- expected local diff
- expected artifact diff
- expected schema change
- expected validator rerun scope
- expected downstream impact
```

unexpected diff がある participant が required scope に含まれる場合、approval boundary へ進めない。

---

## 10. approval boundary

federation execution apply の前に approval boundary を置く。

approval 対象：

```text
- affected projects
- distributed checkpoint
- dry-run result
- downstream action requests
- rollback policy
- known risks
```

approval は federation-level approval と project-local approval に分離する。

federation-level approval があっても、project-local approval を省略してはならない。

---

## 11. execution token

execution token は、approved federation execution を識別するための非秘密識別子である。

含める情報：

```text
- federation_execution_id
- approved_at
- approved_scope
- participant list
- checkpoint references
- expiration
```

execution token は認証秘密ではない。

execution token は、どの approval と checkpoint に基づく execution かを traceable にするために使用する。

---

## 12. project-local execution

実際の変更は各 participant が project-local execution として実行する。

例：

```text
- schema update
- fixture regeneration
- validator rerun
- artifact regeneration
- cleanup execution
```

federation protocol は project-local execution の結果を受け取る。

---

## 13. project-local validation

各 participant は local execution 後に local validation を実行する。

必要な出力：

```text
- local execution report
- local validator report
- updated artifact hash
- rollback availability
- unresolved issue list
```

local validation が fail の場合、federation execution は completed にできない。

---

## 14. federation validator rerun

project-local validation 完了後、federation validator を再実行する。

検査：

```text
- schema drift 解消
- stale artifact 解消
- handoff contract consistency
- reconstruction delta propagation completion
- cleanup dependency resolution
- rollback boundary consistency
```

federation validator が fail の場合、federation execution は failed とする。

---

## 15. distributed rollback coordination

rollback は project-local rollback と federation-level coordination に分離する。

rollback trigger：

```text
- project-local validation fail
- federation validator fail
- downstream artifact incompatibility
- approval scope violation
- unexpected execution result
```

federation-level rollback は各 participant に rollback action request を発行する。

federation protocol は他 project の rollback を直接実行しない。

---

## 16. completion acknowledgement

federation execution 完了時、各 participant から completion acknowledgement を受け取る。

completion status：

```text
completed
completed_with_warning
failed
rolled_back
manual_recovery_required
```

required participant が completed / completed_with_warning でない場合、federation execution は completed にできない。

---

## 17. status model

federation_execution_status の許容値：

```text
proposed
acknowledgement_pending
preflight_running
preflight_blocked
checkpoint_ready
dry_run_completed
approval_required
approved
local_execution_running
local_validation_running
federation_validation_running
completed
completed_with_warning
failed
rollback_required
rolled_back
manual_recovery_required
superseded
```

---

## 18. report schema draft

```json
{
  "schema_version": "1.0",
  "federation_execution_id": "federation-exec-YYYYMMDD-NNN",
  "federation_orchestration_id": "federation-orchestration-YYYYMMDD-NNN",
  "status": "approval_required",
  "participants": [
    {
      "project_id": "SansaVRM",
      "acknowledgement": "accepted",
      "local_checkpoint_id": "checkpoint-YYYYMMDD-NNN",
      "local_validation_status": "not_run"
    }
  ],
  "execution_token": null,
  "blocking_reasons": [],
  "warnings": []
}
```

---

## 19. CI integration

CI では federation execution protocol を dry-run / validation preview として扱う。

CI で行うこと：

```text
- execution proposal generation
- required participant detection
- distributed preflight preview
- dry-run requirement check
- federation validator preview
```

CI で行わないこと：

```text
- project-local apply execution
- project-local cleanup execution
- downstream repository modification
- federation-level approval
```

---

## 20. dashboard projection

federation execution protocol は dashboard projection input として以下を渡す。

```text
- federation execution status
- participant acknowledgement status
- distributed preflight status
- distributed checkpoint status
- approval status
- project-local validation status
- federation validator rerun status
- rollback coordination status
```

Dashboard は federation execution を表示するが、project-local execution を直接実行しない。

---

## 21. reconstruction delta との関係

federation execution 中に reconstruction delta が発生した場合、現在の federation execution は superseded 候補となる。

処理：

```text
- reconstruction_delta_id を記録する
- affected participants を特定する
- current federation execution を superseded または rollback_required にする
- federation orchestration plan を再生成する
- distributed preflight から再開する
```

freeze 後 delta を current federation execution に直接混入してはならない。

---

## 22. 禁止事項

以下を禁止する。

```text
- participant acknowledgement なしに apply へ進むこと
- distributed checkpoint なしに project-local execution を進めること
- federation-level approval のみで project-local approval を省略すること
- stale artifact を freshness 確認なしに completed とすること
- federation validator fail のまま completed とすること
- federation protocol が他 project repository を直接変更すること
```

---

## 23. HLDocS feedback

本 protocol から、HLDocS 側へ以下をフィードバックする。

```text
- multi-project reconstruction には federation execution protocol が必要
- federation-level approval と project-local approval を分離すべき
- distributed checkpoint は local checkpoint の集合として扱うべき
- federation protocol は他 project を直接変更せず action request と acknowledgement で接続すべき
- reconstruction delta 発生時は current federation execution を superseded / rollback_required として扱うべき
```

---

## 24. 結論

federation execution protocol は、federation orchestration plan を複数 project 間で安全に実行するための protocol である。

これにより、cross-project execution handshake、distributed checkpoint、approval boundary、project-local execution、federation validator rerun、distributed rollback coordination を traceable に管理できる。
