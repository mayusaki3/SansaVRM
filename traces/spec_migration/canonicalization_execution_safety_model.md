# canonicalization execution safety model

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における canonicalization execution safety model を定義する。

本 model は、canonicalization execution をいつ許可するか、どの gate で止めるか、どの状態を cleanup に接続してはならないかを整理する。

本 model は Preview Federation MVP では dry-run / planning / validation gate として扱い、production apply execution は対象外とする。

---

## 2. 基本方針

canonicalization execution safety は以下を扱う。

```text
- canonicalization preflight safety
- document fate decision safety
- rewrite transaction safety
- rollback scope safety
- artifact freshness safety
- reconstruction delta safety
- cleanup connection safety
- federation dependency safety
```

canonicalization execution safety は以下を行わない。

```text
- document_fate を自動決定しない
- canonical_doc_id を自動発行しない
- rewrite transaction を自動 apply しない
- cleanup を実行しない
- governance approval を代替しない
```

---

## 3. safety gate taxonomy

safety gate は以下とする。

```text
G1 input completeness gate
G2 document fate gate
G3 canonical identity gate
G4 rewrite transaction gate
G5 rollback scope gate
G6 validation gate
G7 artifact freshness gate
G8 reconstruction delta gate
G9 federation dependency gate
G10 cleanup connection gate
```

---

## 4. G1 input completeness gate

確認：

```text
- canonicalization manifest が存在する
- migration_index が存在する
- canonical_index が存在する
- rewrite_index が存在する
- required registry が存在する
- source_of_truth_refs が存在する
```

Fail 条件：

```text
- required input missing
- malformed manifest
- skeleton index を pass 根拠に使用
```

---

## 5. G2 document fate gate

確認：

```text
- document_fate が決定済み
- pending document_fate がない
- merge / split / obsolete / drop の影響範囲が記録済み
- fate decision の source が traceable
```

Fail 条件：

```text
- document_fate_unknown
- document_fate_pending
- drop target has unresolved reference
- split / merge mapping missing
```

---

## 6. G3 canonical identity gate

確認：

```text
- canonical_doc_id が存在する
- canonical_doc_id duplicate がない
- canonical_doc_id collision がない
- temporary dual canonical が解消済み、または dry-run scope に限定されている
```

Fail 条件：

```text
- canonical_doc_id_missing
- canonical_doc_id_duplicate
- canonical_doc_id_collision
- temporary_dual_canonical_unresolved in apply / cleanup scope
```

---

## 7. G4 rewrite transaction gate

確認：

```text
- required rewrite transaction が存在する
- operation order が正しい
- affected_files が明示されている
- identity / sec_id / traceability / reference rewrite が整合している
- cleanup_preparation が cleanup execution と混同されていない
```

Fail 条件：

```text
- rewrite_transaction_missing
- operation_order_invalid
- affected_files_mismatch
- partial_failed transaction
- executed but not validated transaction used as evidence
```

---

## 8. G5 rollback scope gate

確認：

```text
- rollback_scope が transaction に存在する
- affected file before hash が存在する
- restore_order が定義されている
- generated alias / report / index の rollback 方針がある
```

Fail 条件：

```text
- rollback_scope_missing
- rollback boundary ambiguous
- rollback_required but owner missing
```

MVP では rollback execution は行わない。

ただし rollback scope の欠落は apply candidate を block する。

---

## 9. G6 validation gate

確認：

```text
- manifest_validator pass または allowed warn
- canonicalization_validator pass または allowed warn
- rewrite_validator pass または allowed warn
- cleanup_gate_validator が unsafe ready を出していない
- risk_guard_validator が fail / blocked ではない
```

Fail 条件：

```text
- validator report missing
- required validator fail / blocked
- risk_guard_validator fail / blocked
- unknown_as_pass
- skeleton_as_pass
```

---

## 10. G7 artifact freshness gate

確認：

```text
- required external artifact が fresh または not_applicable
- cleanup_impact=required の artifact が stale / unknown ではない
- superseded artifact が active evidence になっていない
- dashboard artifact stale と source artifact stale を混同していない
```

Fail 条件：

```text
- stale_required_artifact
- required artifact freshness unknown
- superseded artifact used as active
- draft artifact used as canonical dependency
```

---

## 11. G8 reconstruction delta gate

確認：

```text
- open reconstruction delta が execution scope にない
- delta による rerun_required が解消済み
- delta による cleanup_ready invalidation が再評価済み
- superseded execution が active evidence に使われていない
```

Fail 条件：

```text
- reconstruction_delta_open in execution scope
- rerun_required_after_delta unresolved
- cleanup_ready_invalidated_by_delta ignored
- superseded_execution_used_as_active
```

---

## 12. G9 federation dependency gate

確認：

```text
- cross-project handoff pending が cleanup / release scope にない
- federation validator が fail / blocked ではない
- required downstream artifact freshness が確認済み
- federation cleanup dependency が未解決ではない
```

Fail 条件：

```text
- handoff_contract_pending
- federation_validator_not_run
- federation_cleanup_dependency_unresolved
- cross_project_delta_unresolved
```

---

## 13. G10 cleanup connection gate

確認：

```text
- canonicalization completed と cleanup_ready を混同していない
- rewrite_executed を rewrite_validated と扱っていない
- cleanup_preparation を cleanup execution と扱っていない
- project-local cleanup_ready と federation_cleanup_ready を混同していない
```

Fail 条件：

```text
- cleanup_ready_overissued
- rewrite_executed_used_as_validated
- cleanup_preparation_used_as_cleanup_execution
- project_cleanup_ready_used_as_federation_cleanup_ready
```

---

## 14. canonicalization safety status

canonicalization_safety_status は以下とする。

```text
safe_for_dry_run
safe_for_validator_handoff
blocked
rerun_required
superseded
unsafe_for_cleanup
```

MVP では `safe_for_apply` は使用しない。

---

## 15. safety report structure

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "canonicalization_safety_report",
  "preview_only": true,
  "dry_run_only": true,
  "canonicalization_safety_status": "blocked",
  "gates": [
    {
      "gate_id": "G2",
      "gate_name": "document fate gate",
      "status": "blocked",
      "findings": []
    }
  ],
  "source_of_truth_refs": []
}
```

---

## 16. CI mapping

CI fail 条件：

```text
- any safety gate blocked in required scope
- cleanup_ready_overissued
- superseded execution used as active
- stale required artifact used as active
- open reconstruction delta ignored
- project-local cleanup_ready used as federation_cleanup_ready
```

CI warn 条件：

```text
- safe_for_dry_run only
- allowed validator warn
- optional artifact stale
- preview-only limitation
```

---

## 17. dashboard display

Dashboard は canonicalization safety を表示する。

表示対象：

```text
- canonicalization_safety_status
- gate status
- blocking reason
- rerun_required reason
- superseded reason
- cleanup connection block
```

Dashboard は safety status を独自判定しない。

---

## 18. 禁止事項

以下を禁止する。

```text
- pending document_fate のまま canonicalization apply へ進むこと
- rewrite_executed を rewrite_validated と扱うこと
- rollback scope なしに apply candidate とすること
- stale required artifact のまま cleanup_ready にすること
- reconstruction delta 後に rerun なしで cleanup_ready を維持すること
- project-local cleanup_ready を federation_cleanup_ready とみなすこと
```

---

## 19. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバックする。

```text
- canonicalization には execution safety gate が必要
- document fate / canonical identity / rewrite / rollback / validation / artifact freshness / reconstruction delta を分けて gate 化すべき
- safe_for_dry_run と safe_for_apply を分離すべき
- cleanup_preparation と cleanup_execution を分離すべき
- cleanup_ready への接続には dedicated cleanup connection gate が必要
```

---

## 20. 結論

canonicalization execution safety model は、SansaVRM 再構成において canonicalization を安全に dry-run / validation / future apply へ進めるための safety gate model である。

これにより、document fate 未決定、rewrite 未検証、rollback scope 不足、artifact freshness 未確認、reconstruction delta 未反映のまま cleanup / release に進むことを防止できる。
