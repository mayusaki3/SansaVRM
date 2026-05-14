# federation MVP stale superseded propagation model

## 1. 目的

本ドキュメントは、SansaVRM federation MVP における stale / superseded propagation model を定義する。

本 model は、reconstruction delta、schema 更新、validator 更新、registry 更新、external artifact freshness 変更が、artifact、execution、validator report、dashboard snapshot、cleanup gate result にどのように波及するかを整理する。

本 model は Preview Federation MVP 用であり、Production Federation の完全 propagation engine ではない。

---

## 2. 基本方針

stale / superseded propagation は以下を扱う。

```text
- artifact stale propagation
- execution superseded propagation
- validator rerun propagation
- dashboard stale propagation
- cleanup_ready invalidation
- CI fail / warn mapping
```

stale / superseded propagation は以下を行わない。

```text
- artifact を自動削除しない
- cleanup execution を実行しない
- canonicalization apply を実行しない
- downstream repository を変更しない
- producer authority を代替しない
```

---

## 3. propagation source

propagation source は以下とする。

```text
schema_changed
registry_changed
validator_changed
reason_taxonomy_changed
strict_mode_changed
reconstruction_delta_detected
external_artifact_freshness_changed
handoff_contract_changed
dashboard_generator_changed
acceptance_criteria_changed
```

---

## 4. propagation target

propagation target は以下とする。

```text
index_bundle
validator_report
cleanup_gate_report
federation_validator_report
dashboard_snapshot
dashboard_summary
execution_report
acceptance_record
external_artifact_registry_entry
reconstruction_delta_registry_entry
```

---

## 5. propagation actions

propagation action は以下とする。

```text
mark_stale
mark_superseded
mark_rerun_required
mark_cleanup_pending
mark_cleanup_blocked
mark_dashboard_stale
mark_acceptance_superseded
```

---

## 6. stale と superseded の違い

```text
stale:
再生成または再検証が必要だが、replacement が確定していない状態

superseded:
replacement artifact / execution / acceptance が存在し、旧対象を active evidence として使えない状態
```

stale は再実行で解消できる場合がある。

superseded は active evidence として使ってはならない。

---

## 7. schema_changed propagation

schema が変更された場合：

```text
index_bundle → stale
validator_report → stale
dashboard_snapshot → stale
acceptance_record → superseded candidate
```

影響範囲：

```text
- schema を参照する generated artifacts
- schema validation に依存する validator reports
- dashboard projection
- bootstrap acceptance
```

cleanup scope に関係する場合、cleanup_ready を cleanup_pending または cleanup_blocked に戻す。

---

## 8. registry_changed propagation

registry が変更された場合：

```text
index_bundle → stale
validator_report → stale
cleanup_gate_report → stale
dashboard_snapshot → stale
```

対象 registry：

```text
reconstruction_delta_registry
external_artifact_registry
```

registry_changed 後は index builder から再実行する。

---

## 9. validator_changed propagation

validator implementation が変更された場合：

```text
validator_report → stale
cleanup_gate_report → stale where dependent
dashboard_snapshot → stale
acceptance_record → superseded candidate
```

validator module version が変わった場合、旧 validator report を active pass として使ってはならない。

---

## 10. reason_taxonomy_changed propagation

reason taxonomy が変更された場合：

```text
validator_report → stale
dashboard_snapshot → stale
risk review → rerun_required
acceptance_record → superseded candidate
```

理由：

```text
同じ input でも finding の意味が変わる可能性があるため。
```

---

## 11. strict_mode_changed propagation

strict mode が変更された場合：

```text
validator_report → stale
cleanup_gate_report → stale
CI result → stale
acceptance_record → superseded candidate
```

strict mode が弱くなる場合、acceptance は再 review 必須とする。

---

## 12. reconstruction_delta_detected propagation

reconstruction delta が検出された場合：

```text
index_bundle → stale
validator_report → stale
cleanup_gate_report → stale
dashboard_snapshot → stale
execution_report → superseded candidate
acceptance_record → superseded candidate
```

cleanup_ready は原則再評価する。

semantic_delta / validation_delta / cleanup_delta / cross_project_delta は cleanup_ready を invalidated とする。

---

## 13. external_artifact_freshness_changed propagation

external artifact freshness が変化した場合：

```text
external_artifact_registry_entry → updated
index_bundle → stale
federation_validator_report → stale
cleanup_gate_report → stale where cleanup_impact required/unknown
dashboard_snapshot → stale
```

cleanup_impact=required の artifact が stale / unknown になった場合、cleanup_ready を cleanup_blocked とする。

---

## 14. handoff_contract_changed propagation

handoff contract が変更された場合：

```text
external_artifact_registry_entry → stale or updated
federation_validator_report → stale
cleanup_gate_report → stale
dashboard_snapshot → stale
acceptance_record → superseded candidate
```

handoff pending が発生した場合、federation cleanup を blocked とする。

---

## 15. dashboard_generator_changed propagation

Dashboard generator が変更された場合：

```text
dashboard_snapshot → stale
dashboard_summary → stale
```

validator report は stale にしない。

理由：

```text
Dashboard は projection であり、validator source of truth ではないため。
```

---

## 16. acceptance_criteria_changed propagation

acceptance criteria が変更された場合：

```text
acceptance_record → superseded
dashboard_snapshot → stale
risk review → rerun_required
```

bootstrap_accepted は再 acceptance を必要とする。

---

## 17. propagation matrix

```text
source                         index validator cleanup dashboard acceptance
schema_changed                  S     S         S       S         SC
registry_changed                S     S         S       S         -
validator_changed               -     S         S       S         SC
reason_taxonomy_changed          -     S         S       S         SC
strict_mode_changed              -     S         S       S         SC
reconstruction_delta_detected    S     S         S       S         SC
external_artifact_freshness      S     -         S       S         -
handoff_contract_changed         S     -         S       S         SC
dashboard_generator_changed      -     -         -       S         -
acceptance_criteria_changed      -     -         -       S         SS
```

凡例：

```text
S  = stale
SC = superseded candidate
SS = superseded
-  = no direct propagation
```

---

## 18. propagation record schema draft

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "propagation_record",
  "preview_only": true,
  "dry_run_only": true,
  "propagation_id": "propagation-YYYYMMDD-NNN",
  "source_kind": "reconstruction_delta_detected",
  "source_id": "delta-example",
  "actions": [
    {
      "target_id": "cleanup_gate_validator_report.json",
      "action": "mark_stale",
      "reason": "cleanup_ready_invalidated_by_delta"
    }
  ]
}
```

---

## 19. CI mapping

CI fail 条件：

```text
- superseded artifact used as active
- stale required validator report used as active
- cleanup_ready not invalidated after required propagation
- acceptance_record superseded but still active
- propagation source detected but no propagation record generated
```

CI warn 条件：

```text
- dashboard stale but regenerated in same run
- optional artifact stale
- superseded candidate pending review
```

---

## 20. dashboard display

Dashboard は propagation state を表示する。

表示対象：

```text
- propagation source
- affected targets
- action
- reason
- cleanup_ready invalidation
- superseded candidate
- rerun_required
```

Dashboard は propagation を独自実行しない。

---

## 21. 禁止事項

以下を禁止する。

```text
- superseded artifact を active evidence として使うこと
- stale validator report を active pass として使うこと
- reconstruction delta 後に cleanup_ready を再評価しないこと
- dashboard stale を validator stale と混同すること
- propagation source を検出して propagation record を残さないこと
```

---

## 22. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction artifact には stale / superseded propagation model が必要
- schema / registry / validator / reason taxonomy 変更は rerun_required を発生させるべき
- dashboard generator 変更と validator result stale を分離すべき
- reconstruction delta 後は cleanup_ready invalidation を伝播すべき
- propagation record を machine-readable artifact として扱うべき
```

---

## 23. 結論

federation MVP stale/superseded propagation model は、Preview Federation MVP における stale / superseded / rerun_required / cleanup invalidation の波及を定義する model である。

これにより、schema、registry、validator、reason taxonomy、reconstruction delta、external artifact freshness の変更が、どの artifact / execution / dashboard / cleanup gate に影響するかを machine-readable に管理できる。
