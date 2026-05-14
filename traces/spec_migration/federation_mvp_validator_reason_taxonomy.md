# federation MVP validator reason taxonomy

## 1. 目的

本ドキュメントは、SansaVRM federation MVP における validator reason taxonomy を定義する。

本 taxonomy は、validator finding、CI fail / warn、dashboard blocking reason、risk detection、bootstrap acceptance に共通利用する reason code system である。

本 taxonomy は Preview Federation MVP 用であり、Production Federation の完全 taxonomy ではない。

---

## 2. 基本方針

reason taxonomy は以下を満たす。

```text
- machine-readable reason code を持つ
- source_domain を持つ
- severity と分離する
- status と分離する
- dashboard 表示に利用できる
- CI fail / warn mapping に利用できる
- unknown / skeleton / stale / draft misuse を明示する
```

reason taxonomy は以下を行わない。

```text
- validator status を直接置き換えない
- cleanup_ready を直接判定しない
- governance decision を代替しない
- production taxonomy を確定しない
```

---

## 3. reason code format

reason code は snake_case とする。

例：

```text
unknown_as_pass
skeleton_index_used_as_pass
draft_schema_inside_canonical_boundary
stale_required_artifact
missing_source_of_truth_refs
```

---

## 4. source_domain

source_domain は以下とする。

```text
index
manifest
canonicalization
rewrite
cleanup_gate
federation
artifact
dashboard
ci
risk
release
governance
reconstruction_delta
```

---

## 5. severity

severity は reason code ではなく finding ごとに持つ。

許容値：

```text
info
warn
fail
blocked
```

同じ reason code でも scope により severity が変わる場合がある。

例：

```text
stale_optional_artifact:
warn

stale_required_artifact:
blocked
```

---

## 6. index reason codes

```text
index_bundle_missing
index_bundle_malformed
filesystem_index_missing
migration_index_missing
canonical_index_missing
rewrite_index_missing
external_artifact_index_missing
hash_index_missing
skeleton_index_present
skeleton_index_used_as_pass
unknown_index_state
```

重要：

```text
skeleton_index_present
```

は warn でよい場合がある。

ただし：

```text
skeleton_index_used_as_pass
```

は fail / blocked とする。

---

## 7. manifest reason codes

```text
manifest_missing
manifest_malformed
migration_entry_id_duplicate
dry_doc_id_duplicate
source_path_missing
target_path_missing
duplicate_path
placeholder_relocation_remaining
migration_state_unknown
mapping_status_unknown
migration_verified_target_missing
```

---

## 8. canonicalization reason codes

```text
document_fate_unknown
document_fate_pending
canonical_doc_id_missing
canonical_doc_id_duplicate
canonical_doc_id_collision
temporary_dual_canonical_unresolved
semantic_equivalent_unknown
cleanup_allowed_invalid
drop_target_has_unresolved_reference
obsolete_target_requires_alias
```

---

## 9. rewrite reason codes

```text
rewrite_transaction_missing
rewrite_transaction_malformed
transaction_status_unknown
rewrite_not_validated
rewrite_failed
operation_kind_unknown
affected_files_mismatch
rollback_scope_missing
reference_rewrite_incomplete
traceability_rewrite_incomplete
sec_id_rewrite_incomplete
```

MVP では reference / traceability / sec_id の詳細検査が skeleton の場合がある。

その場合でも、未実装を pass 根拠にしてはならない。

---

## 10. cleanup gate reason codes

```text
cleanup_ready_overissued
cleanup_blocked_by_unknown
cleanup_blocked_by_skeleton
cleanup_blocked_by_placeholder
cleanup_blocked_by_pending_fate
cleanup_blocked_by_unvalidated_rewrite
cleanup_blocked_by_stale_artifact
cleanup_blocked_by_handoff_pending
cleanup_pending_due_to_preview_scope
```

特に：

```text
cleanup_ready_overissued
```

は critical risk として扱う。

---

## 11. artifact reason codes

```text
external_artifact_registry_missing
artifact_id_missing
artifact_stage_unknown
artifact_freshness_unknown
stale_required_artifact
stale_optional_artifact
artifact_cleanup_impact_unknown
draft_artifact_used_as_canonical_dependency
experimental_artifact_used_for_cleanup_dependency
```

---

## 12. federation reason codes

```text
handoff_response_missing
handoff_response_exists_but_contract_pending
handoff_contract_missing
handoff_contract_pending
schema_drift_detected
draft_schema_inside_canonical_boundary
cross_project_delta_unresolved
federation_cleanup_dependency_unresolved
federation_validator_not_run
```

MVP では full schema drift detection は limited でよい。

ただし draft / canonical boundary は MVP でも検出対象とする。

---

## 13. dashboard reason codes

```text
dashboard_snapshot_missing
dashboard_projection_failed
dashboard_missing_source_of_truth_refs
dashboard_attempted_source_mutation
dashboard_cleanup_state_mismatch
dashboard_projection_only_missing
```

---

## 14. CI reason codes

```text
required_report_missing
required_artifact_missing
ci_dry_run_failed
ci_attempted_destructive_operation
ci_generated_artifact_committed_as_source
ci_strict_mode_failed
```

---

## 15. risk reason codes

```text
stub_validator_overtrust
unknown_as_pass
skeleton_as_pass
dashboard_used_as_source_of_truth
draft_schema_used_as_canonical
preview_treated_as_production
missing_preview_only_flag
missing_dry_run_only_flag
missing_projection_only_flag
```

---

## 16. reconstruction delta reason codes

```text
reconstruction_delta_registry_missing
reconstruction_delta_unrecorded
reconstruction_delta_open
rerun_required_after_delta
cleanup_ready_invalidated_by_delta
superseded_execution_used_as_active
```

---

## 17. governance / release reason codes

```text
governance_decision_missing
release_authority_missing
bootstrap_acceptance_missing
bootstrap_superseded
preview_release_missing_known_limitations
release_candidate_has_open_delta
canonical_release_has_draft_dependency
```

---

## 18. CI mapping baseline

MVP CI fail / blocked にする reason：

```text
unknown_as_pass
skeleton_as_pass
skeleton_index_used_as_pass
cleanup_ready_overissued
missing_source_of_truth_refs
dashboard_missing_source_of_truth_refs
draft_schema_inside_canonical_boundary
stale_required_artifact
required_report_missing
required_artifact_missing
ci_attempted_destructive_operation
missing_preview_only_flag
missing_dry_run_only_flag
```

MVP CI warn にできる reason：

```text
skeleton_index_present
stale_optional_artifact
artifact_freshness_unknown outside cleanup scope
cleanup_pending_due_to_preview_scope
preview_release_missing_known_limitations
```

---

## 19. dashboard display rule

Dashboard は reason code を表示する。

表示項目：

```text
- reason code
- severity
- source_domain
- target_id
- message
- blocking flag
```

Dashboard は reason code を変換・削除してはならない。

---

## 20. finding schema draft

```json
{
  "finding_id": "finding-0001",
  "reason": "skeleton_index_used_as_pass",
  "severity": "blocked",
  "source_domain": "index",
  "target_id": "traceability_index",
  "blocking": true,
  "message": "skeleton traceability_index cannot be used as pass evidence"
}
```

---

## 21. 禁止事項

以下を禁止する。

```text
- reason code を free text のみで表現すること
- severity と reason code を混同すること
- status と reason code を混同すること
- unknown / skeleton / stale を generic warn に丸めること
- dashboard が reason code を独自変更すること
```

---

## 22. HLDocS feedback

本 taxonomy から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction validator には reason code taxonomy が必要
- unknown / skeleton / stale / draft misuse を明示 reason として扱うべき
- severity / status / reason code を分離すべき
- dashboard は reason code を保持して表示すべき
- CI fail / warn mapping は reason code に基づくべき
```

---

## 23. 結論

federation MVP validator reason taxonomy は、Preview Federation MVP における validator finding、CI 判定、dashboard 表示、risk detection を統一する reason code system である。

これにより、unsafe pass、draft/canonical 混同、stale artifact、source of truth 混同を machine-readable に検出・表示できる。
