# federation validator implementation detail

## 1. 目的

本ドキュメントは、SansaVRM federation MVP における validator implementation detail を定義する。

本 detail は、Preview Federation MVP の validator modules を実装単位へ分割し、入力、出力、実行順、reason code、strict mode、CI mapping を整理する。

本仕様は Production Federation の完全 validator 実装仕様ではない。

---

## 2. 基本方針

validator implementation は以下を満たす。

```text
- read-only で動作する
- source file を変更しない
- cleanup / apply を実行しない
- reason taxonomy を使用する
- unknown / skeleton / stale を pass 扱いしない
- source_of_truth_refs を report に含める
- strict mode で unsafe condition を fail / blocked にできる
```

validator implementation は以下を行わない。

```text
- canonicalization apply
- cleanup execution
- downstream repository modification
- governance approval
- dashboard state mutation
```

---

## 3. validator modules

MVP validator modules は以下とする。

```text
manifest_validator
canonicalization_validator
rewrite_validator
cleanup_gate_validator
federation_validator
projection_validator
risk_guard_validator
```

### manifest_validator

migration / manifest / index の基本整合性を検査する。

### canonicalization_validator

document_fate / canonical_doc_id / cleanup_allowed を検査する。

### rewrite_validator

rewrite transaction / rollback scope / rewrite_state を検査する。

### cleanup_gate_validator

cleanup_ready / cleanup_blocked / cleanup_pending を dry-run 判定する。

### federation_validator

handoff / external artifact / draft-canonical boundary を検査する。

### projection_validator

dashboard snapshot の projection 整合性を検査する。

### risk_guard_validator

bootstrap risk register と unsafe pass pattern を検査する。

---

## 4. execution order

MVP validator 実行順：

```text
1. manifest_validator
2. canonicalization_validator
3. rewrite_validator
4. federation_validator
5. cleanup_gate_validator
6. projection_validator
7. risk_guard_validator
```

cleanup_gate_validator は、前段 validator reports を入力として参照する。

projection_validator は dashboard generation 後に実行してもよい。

risk_guard_validator は最終 gate として実行する。

---

## 5. common input

全 validator の common input：

```text
reports/federation/index_bundle.json
traces/spec_migration/reconstruction_delta_registry.json
traces/spec_migration/external_artifact_registry.json
tools/federation_migration/schemas/*.schema.json
```

必要に応じて以下を参照する。

```text
previous validator reports
reason taxonomy
bootstrap risk register
acceptance criteria
```

---

## 6. common output

各 validator は以下を出力する。

```text
reports/federation/{validator_module}_report.json
reports/federation/{validator_module}_summary.md
```

Report は以下を含む。

```text
schema_version
artifact_kind
validator_module
preview_only
dry_run_only
stub
strict
status
source_of_truth_refs
summary
findings
```

---

## 7. common status aggregation

validator report の status aggregation：

```text
blocked が 1件以上:
status = blocked

fail が 1件以上:
status = fail

warn が 1件以上:
status = warn

finding がなく正常:
status = pass
```

`not_applicable` は module 全体が対象外の場合のみ使用する。

---

## 8. manifest_validator detail

入力：

```text
index_bundle.indexes.filesystem_index
index_bundle.indexes.migration_index
index_bundle.skeleton_indexes
```

検査：

```text
- index_bundle_missing
- index_bundle_malformed
- migration_index_missing
- migration_entry_id_duplicate
- dry_doc_id_duplicate
- placeholder_relocation_remaining
- migration_state_unknown
- skeleton_index_used_as_pass
```

MVP では migration_index が空でも warn としてよい。

ただし、空 index を根拠に cleanup_ready を出してはならない。

---

## 9. canonicalization_validator detail

入力：

```text
index_bundle.indexes.canonical_index
manifest_validator_report
```

検査：

```text
- document_fate_unknown
- document_fate_pending
- canonical_doc_id_missing
- canonical_doc_id_duplicate
- temporary_dual_canonical_unresolved
- semantic_equivalent_unknown
- cleanup_allowed_invalid
```

MVP では semantic_equivalent unknown を cleanup_ready 不可として扱う。

---

## 10. rewrite_validator detail

入力：

```text
index_bundle.indexes.rewrite_index
canonicalization_validator_report
```

検査：

```text
- rewrite_transaction_missing
- transaction_status_unknown
- rewrite_not_validated
- operation_kind_unknown
- affected_files_mismatch
- rollback_scope_missing
- reference_rewrite_incomplete
- traceability_rewrite_incomplete
- sec_id_rewrite_incomplete
```

MVP では reference / traceability / sec_id の詳細抽出が skeleton の場合、pass 根拠にしない。

---

## 11. federation_validator detail

入力：

```text
index_bundle.indexes.external_artifact_index
external_artifact_registry.json
reconstruction_delta_registry.json
```

検査：

```text
- external_artifact_registry_missing
- artifact_stage_unknown
- artifact_freshness_unknown
- stale_required_artifact
- draft_artifact_used_as_canonical_dependency
- experimental_artifact_used_for_cleanup_dependency
- handoff_response_missing
- handoff_response_exists_but_contract_pending
- cross_project_delta_unresolved
```

MVP では full schema drift detection は optional とする。

ただし、draft / canonical boundary は検査対象とする。

---

## 12. cleanup_gate_validator detail

入力：

```text
manifest_validator_report
canonicalization_validator_report
rewrite_validator_report
federation_validator_report
index_bundle
```

判定：

```text
cleanup_ready
cleanup_blocked
cleanup_pending
```

cleanup_ready を出す条件：

```text
- required validator status が pass または allowed warn
- skeleton index が pass 根拠になっていない
- required external artifact が stale / unknown ではない
- pending document_fate がない
- rewrite_not_validated がない
- temporary dual canonical がない
```

MVP では不明なものは cleanup_pending または cleanup_blocked とする。

---

## 13. projection_validator detail

入力：

```text
dashboard_snapshot.json
validator reports
cleanup_gate_validator_report
```

検査：

```text
- dashboard_snapshot_missing
- dashboard_projection_failed
- dashboard_missing_source_of_truth_refs
- dashboard_cleanup_state_mismatch
- dashboard_projection_only_missing
```

projection_validator は dashboard を修正しない。

---

## 14. risk_guard_validator detail

入力：

```text
validator reports
dashboard_snapshot.json
bootstrap risk register
acceptance criteria
```

検査：

```text
- stub_validator_overtrust
- unknown_as_pass
- skeleton_as_pass
- dashboard_used_as_source_of_truth
- draft_schema_used_as_canonical
- preview_treated_as_production
- missing_preview_only_flag
- missing_dry_run_only_flag
- missing_projection_only_flag
```

risk_guard_validator は CI 最終 gate として使用する。

---

## 15. strict mode

`--strict true` の場合、以下を fail / blocked とする。

```text
- unknown_as_pass
- skeleton_as_pass
- missing_source_of_truth_refs
- missing_preview_only_flag
- missing_dry_run_only_flag
- missing_projection_only_flag
- cleanup_ready_overissued
- superseded artifact used as active
- stale required artifact used as active
```

Preview Federation MVP CI では strict mode を有効にする。

---

## 16. report generation rule

各 validator は、自 module の report を必ず生成する。

validator が対象外の場合でも、以下を出す。

```text
status = not_applicable
findings = []
source_of_truth_refs = [...]
```

Report missing は CI fail とする。

---

## 17. CI fail / warn mapping

CI fail 条件：

```text
- required report missing
- validator execution error
- status = fail / blocked in required module
- risk_guard_validator fail / blocked
- cleanup_ready_overissued
- unknown_as_pass
- skeleton_as_pass
```

CI warn 条件：

```text
- status = warn
- skeleton_index_present without pass usage
- stale optional artifact
- cleanup_pending_due_to_preview_scope
```

---

## 18. dashboard connection

Dashboard は validator reports を入力として表示する。

Dashboard は以下を保持する。

```text
- validator_module
- status
- findings
- reason codes
- source_of_truth_refs
```

Dashboard は finding を削除・改変してはならない。

---

## 19. implementation files

実装候補：

```text
tools/federation_migration/validators/run_validators.py
tools/federation_migration/validators/manifest_validator.py
tools/federation_migration/validators/canonicalization_validator.py
tools/federation_migration/validators/rewrite_validator.py
tools/federation_migration/validators/federation_validator.py
tools/federation_migration/validators/cleanup_gate_validator.py
tools/federation_migration/validators/projection_validator.py
tools/federation_migration/validators/risk_guard_validator.py
```

初期実装では単一 `run_validators.py` 内にまとめてもよい。

ただし module 名と report は分離する。

---

## 20. 禁止事項

以下を禁止する。

```text
- validator が source file を変更すること
- validator が cleanup / apply を実行すること
- validator が dashboard snapshot を source of truth として扱うこと
- unknown / skeleton / stale を generic warn に丸めること
- report missing を success とすること
- risk_guard_validator を CI から外すこと
```

---

## 21. HLDocS feedback

本 detail から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction validator は module 分割と実行順を持つべき
- cleanup gate validator は前段 validator reports を入力とすべき
- risk guard validator を最終 gate として置くべき
- report missing を fail とすべき
- strict mode により unsafe pass を fail / blocked にすべき
```

---

## 22. 結論

federation validator implementation detail は、Preview Federation MVP の validator modules を実装可能な単位へ分割する仕様である。

これにより、index、manifest、canonicalization、rewrite、federation、cleanup gate、dashboard projection、risk guard を read-only / dry-run / validator-first の原則で実装できる。
