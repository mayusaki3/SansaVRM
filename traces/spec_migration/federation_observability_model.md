# federation observability model

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における federation observability model を定義する。

federation observability model は、multi-project reconstruction における validator、execution、cleanup、recovery、governance、handoff、artifact freshness を監視・記録・監査するための観測モデルである。

---

## 2. 基本方針

federation observability は以下を行う。

```text
- state transition を記録する
- validator result を監査可能にする
- execution / rollback / cleanup を traceable にする
- governance decision を audit trail 化する
- cross-project handoff と acknowledgement を記録する
- stale artifact / schema drift / reconstruction delta を観測する
- dashboard / CI / release review の入力を生成する
```

federation observability は以下を行わない。

```text
- validator result を変更しない
- execution を実行しない
- governance decision を承認しない
- stale artifact を fresh として扱わない
- dashboard projection を source of truth として扱わない
```

---

## 3. observability domains

observability domain は以下とする。

```text
state_observability
validator_observability
execution_observability
cleanup_observability
recovery_observability
governance_observability
handoff_observability
artifact_observability
consistency_observability
```

---

## 4. event model

観測イベントは append-only を基本とする。

主な event_kind：

```text
state_transition
validator_run
execution_started
execution_completed
execution_failed
rollback_requested
rollback_completed
cleanup_gate_evaluated
cleanup_executed
reconstruction_delta_detected
handoff_acknowledged
artifact_freshness_changed
governance_decision_recorded
consistency_violation_detected
manual_recovery_required
```

---

## 5. event schema draft

```json
{
  "schema_version": "1.0",
  "event_id": "federation-event-YYYYMMDD-NNN",
  "event_kind": "validator_run",
  "occurred_at": "YYYY-MM-DDTHH:MM:SSZ",
  "source_project": "SansaVRM",
  "target_id": "target-example",
  "related_ids": {
    "validator_run_id": "validator-YYYYMMDD-NNN",
    "federation_execution_id": null,
    "governance_decision_id": null
  },
  "status": "pass",
  "summary": "manifest validator pass"
}
```

---

## 6. audit trail

Audit trail は以下の対象について保持する。

```text
- canonicalization decision
- rewrite transaction execution
- cleanup gate decision
- cleanup execution
- federation execution approval
- handoff contract acceptance
- governance decision
- rollback / recovery decision
- release decision
```

Audit trail は dashboard snapshot とは別に保持する。

Dashboard snapshot は audit trail の projection であり、audit trail そのものではない。

---

## 7. telemetry targets

telemetry target は以下とする。

```text
- validator duration
- validator status count
- cleanup blocked count
- rerun_required count
- stale artifact count
- schema drift count
- reconstruction delta count
- manual recovery count
- federation execution duration
- rollback duration
```

Telemetry は運用改善のために使う。

Telemetry のみで validation pass / cleanup_ready を判断してはならない。

---

## 8. trace correlation

複数 project 間の trace correlation には以下を用いる。

```text
- federation_execution_id
- federation_orchestration_id
- reconstruction_delta_id
- handoff_contract_id
- governance_decision_id
- validator_run_id
- cleanup_execution_id
- rollback_package_id
```

これらは秘密情報ではなく、traceability identifier として扱う。

---

## 9. freshness observability

external artifact freshness は observability 対象とする。

観測項目：

```text
- artifact_id
- source_project
- source_revision
- schema_version
- content_hash
- freshness_status
- last_validated_at
- consumed_by
```

freshness_status の候補：

```text
fresh
stale
unknown
not_applicable
```

---

## 10. governance observability

Governance decision は audit trail として記録する。

対象：

```text
- approval authority
- cleanup owner
- rollback owner
- handoff contract authority
- manual recovery owner
- release authority
```

Governance decision が missing の場合、dashboard / CI で検出できるようにする。

---

## 11. recovery observability

Recovery 状態は以下を記録する。

```text
- failure kind
- recovery action request
- responsible authority
- recovery status
- validation after recovery
- unresolved manual recovery
```

manual_recovery_required は必ず audit trail に記録する。

---

## 12. dashboard projection

Dashboard は observability event / audit trail を表示できる。

表示対象：

```text
- latest state
- state history
- validator history
- cleanup gate history
- reconstruction delta history
- handoff history
- recovery history
- governance decision history
```

Dashboard は audit trail を変更してはならない。

---

## 13. CI integration

CI では以下を検査する。

```text
- required audit trail が存在する
- validator run id が traceable である
- cleanup gate decision に source refs がある
- governance decision が必要な scope で欠落していない
- stale artifact event が未解決のまま cleanup_ready になっていない
```

CI fail 条件：

```text
- required audit trail missing
- governance decision missing in destructive scope
- stale artifact unresolved in cleanup scope
- manual_recovery_required unresolved
- forbidden transition event detected
```

---

## 14. retention policy

観測情報の保持方針：

```text
short_lived:
temporary dashboard snapshot

medium_lived:
validator reports / execution reports

long_lived:
audit trail / governance decisions / release decisions / handoff contracts
```

保持期間は project policy に従う。

ただし、canonicalization / cleanup / release に関わる audit trail は長期保持対象とする。

---

## 15. 禁止事項

以下を禁止する。

```text
- dashboard snapshot を audit trail の代替にすること
- telemetry metric のみで cleanup_ready を判断すること
- governance decision missing を warning のみで済ませること
- manual_recovery_required を audit trail なしに解消すること
- stale artifact event を無視して federation cleanup を承認すること
```

---

## 16. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバックする。

```text
- multi-project reconstruction には observability / auditability model が必要
- dashboard snapshot と audit trail を分離すべき
- governance decision / cleanup gate / release decision は audit trail に残すべき
- external artifact freshness を観測対象にすべき
- telemetry は validation 判定の代替ではないと明記すべき
```

---

## 17. 結論

federation observability model は、multi-project reconstruction の状態、検証、実行、cleanup、recovery、governance、handoff、artifact freshness を監視・監査するための観測モデルである。

これにより、分散した project 間の状態を dashboard / CI / release review で確認しつつ、source of truth と audit trail を混同せずに運用できる。
