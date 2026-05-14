# federation orchestration engine

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における federation orchestration engine を定義する。

federation orchestration engine は、SansaVRM と周辺 project 間の rerun、cleanup coordination、rollback coordination、cross-project execution ordering を実行計画として扱う。

federation validator は observer / gate であり、federation orchestration engine は coordination planner / executor である。

---

## 2. 基本方針

federation orchestration engine は以下を行う。

```text
- federation graph を読み込む
- cross-project dependency impact を解析する
- federated rerun plan を作成する
- federated cleanup coordination plan を作成する
- federated rollback coordination plan を作成する
- project 間 execution ordering を決定する
- downstream project への handoff / rerun requirement を生成する
```

federation orchestration engine は以下を行わない。

```text
- 他 project の repository を直接変更しない
- federation validator result を捏造しない
- stale artifact を fresh として扱わない
- cleanup gate validator を迂回しない
- handoff contract を自動承認しない
```

---

## 3. 入力

federation orchestration engine の入力は以下とする。

```text
- federation graph
- federation validator report
- reconstruction delta report
- cross-project handoff contracts
- external_artifact_index
- project validator reports
- project cleanup gate reports
- project execution reports
- CI context
```

---

## 4. 出力

federation orchestration engine は以下を出力する。

```text
- federation orchestration plan
- federated rerun plan
- federated cleanup coordination plan
- federated rollback coordination plan
- cross-project execution order
- downstream action request list
- federation dashboard projection input
```

---

## 5. orchestration lifecycle

federation orchestration lifecycle は以下とする。

```text
1. federation graph intake
2. federation validator result intake
3. impact analysis
4. coordination scope selection
5. rerun planning
6. cleanup coordination planning
7. rollback coordination planning
8. execution ordering
9. downstream action request generation
10. federation dashboard projection
```

---

## 6. coordination scope

coordination scope の種類：

```text
schema_scope
artifact_scope
handoff_scope
cleanup_scope
rollback_scope
reconstruction_delta_scope
full_federation_scope
```

### schema_scope

schema drift / schema dependency に関する coordination。

### artifact_scope

fixture、diagnostics、conversion_report 等の artifact freshness に関する coordination。

### handoff_scope

handoff contract の pending / accepted / superseded に関する coordination。

### cleanup_scope

project 間 cleanup dependency に関する coordination。

### rollback_scope

project 間 rollback boundary に関する coordination。

### reconstruction_delta_scope

project 間 reconstruction delta propagation に関する coordination。

### full_federation_scope

全 project 横断の再検証・再計画。

---

## 7. federated rerun plan

federated rerun plan は、project 間で必要な validator rerun を定義する。

含める情報：

```text
- rerun_plan_id
- source_delta_id
- affected_projects
- required_validators
- required_artifacts
- rerun_order
- blocking_dependencies
```

rerun_order は federation graph の dependency に従う。

例：

```text
1. SansaVRM schema / boundary validator
2. MuJoCo Adapter schema compatibility validator
3. Adapter fixture validator
4. SansaVRM cleanup gate validator
5. federation validator rerun
```

---

## 8. federated cleanup coordination

federated cleanup coordination は、cleanup_ready を project 間 dependency 付きで扱う。

cleanup coordination で確認すること：

```text
- downstream project が old schema / old artifact を消費していない
- handoff contract が accepted である
- required external artifact が fresh である
- downstream validator rerun が完了している
- rollback boundary が定義されている
```

上記が未達の場合、cleanup は project 内では cleanup_ready でも federation cleanup blocked とする。

---

## 9. federated rollback coordination

federated rollback coordination は、rollback が downstream project に与える影響を扱う。

確認項目：

```text
- rollback owner project
- affected downstream schemas
- affected downstream artifacts
- downstream rerun requirement
- stale artifact propagation
- manual recovery requirement
```

project 間 rollback は、単一 repository rollback と同一視しない。

---

## 10. cross-project execution ordering

cross-project execution ordering は、project 間の実行順序を定義する。

基本順序：

```text
1. source project schema / contract update
2. source project validator rerun
3. consumer project fixture / artifact regeneration
4. consumer project validator rerun
5. federation validator rerun
6. cleanup gate reevaluation
7. cleanup coordination approval
```

ただし、artifact が draft / experimental の場合は dry-run ordering として扱う。

---

## 11. downstream action request

downstream action request は、他 project に要求する作業を明示する。

例：

```text
- regenerate fixture
- rerun adapter validator
- update draft schema
- respond to handoff contract
- reclassify diagnostics
- confirm artifact freshness
```

schema draft：

```json
{
  "action_request_id": "federation-action-YYYYMMDD-NNN",
  "target_project": "SansaVRM-MuJoCo-Adapter",
  "request_kind": "rerun_validator",
  "reason": "schema_drift_detected",
  "required_before": "federation_cleanup_approval"
}
```

---

## 12. reconstruction delta propagation

federation orchestration engine は reconstruction delta を federation graph に沿って伝播する。

処理：

```text
- source delta を受け取る
- affected project / schema / artifact を特定する
- required rerun を計画する
- cleanup_ready invalidation を伝播する
- replacement execution requirement を記録する
```

cross-project delta は localized patch として扱ってはならない。

---

## 13. federation orchestration status

status の許容値：

```text
planned
impact_analyzed
rerun_required
rerun_completed
cleanup_coordination_required
cleanup_coordination_blocked
rollback_coordination_required
ready_for_federation_cleanup
blocked
superseded
```

---

## 14. report schema draft

```json
{
  "schema_version": "1.0",
  "federation_orchestration_id": "federation-orchestration-YYYYMMDD-NNN",
  "status": "rerun_required",
  "coordination_scope": "reconstruction_delta_scope",
  "affected_projects": [
    "SansaVRM",
    "SansaVRM-MuJoCo-Adapter"
  ],
  "rerun_plan": [],
  "cleanup_coordination": [],
  "rollback_coordination": [],
  "downstream_action_requests": []
}
```

---

## 15. CI integration

CI では federation orchestration を dry-run coordination として扱う。

CI で行うこと：

```text
- federation graph impact analysis
- rerun_required detection
- stale artifact detection
- downstream action request preview
- federation cleanup blocked 判定
```

CI で行わないこと：

```text
- downstream repository の変更
- external artifact の自動再生成
- handoff contract の自動承認
- federation cleanup execution
```

---

## 16. dashboard projection

federation orchestration engine は dashboard projection input として以下を渡す。

```text
- federation orchestration status
- affected projects
- rerun plan status
- cleanup coordination status
- rollback coordination status
- downstream action requests
- blocking reasons
```

Dashboard は orchestration を表示するが、他 project への変更を実行しない。

---

## 17. 禁止事項

以下を禁止する。

```text
- federation validator fail を無視して cleanup coordination approval へ進むこと
- stale artifact を fresh として扱うこと
- downstream action request 未完了のまま federation cleanup を承認すること
- cross-project rollback を単一 project rollback として扱うこと
- draft schema を canonical contract として扱うこと
```

---

## 18. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバックする。

```text
- federation validator と federation orchestration engine を分離すべき
- multi-project reconstruction では rerun / cleanup / rollback coordination が必要
- downstream action request を traceable artifact として扱うべき
- federation cleanup は project 内 cleanup_ready だけで承認してはならない
- cross-project reconstruction delta は federation graph に沿って伝播すべき
```

---

## 19. 結論

federation orchestration engine は、SansaVRM と周辺 project 間の rerun、cleanup coordination、rollback coordination、execution ordering を管理する coordination planner / executor である。

これにより、単一 project 内の validator / cleanup だけでは扱えない multi-project reconstruction dependency を、安全に計画・表示・調整できる。
