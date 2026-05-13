# rewrite validator implementation

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における rewrite validator implementation を定義する。

rewrite validator は、rewrite transaction plan/report を検査し、canonicalization manifest で確定した document fate decision が、実ファイル・参照・traceability・sec_id・testspec/code reference へ安全に反映されているかを確認する。

---

## 2. 基本方針

rewrite validator は以下を行う。

```text
- rewrite transaction の構造を検査する
- operation ordering を検査する
- old_value / new_value の反映状態を検査する
- transaction atomicity を検査する
- rollback scope を検査する
- reference rewrite の整合性を検査する
- traceability rewrite の整合性を検査する
- sec_id rewrite の整合性を検査する
- testspec/code reference rewrite の整合性を検査する
- cleanup_preparation の妥当性を検査する
```

rewrite validator は以下を行わない。

```text
- rewrite を実行しない
- rollback を実行しない
- doc_id を発行しない
- sec_id を生成しない
- cleanup を実行しない
```

---

## 3. 入力

rewrite validator の入力は以下とする。

```text
- rewrite_index
- canonical_index
- migration_index
- filesystem_index
- reference_index
- traceability_index
- sec_id_index
- hash_index
- canonicalization validator result
```

canonicalization validator が fail の場合、rewrite validator は原則として `blocked` とする。

ただし、既存 rewrite transaction の原因調査用 partial validation は許可する。

---

## 4. 出力

rewrite validator は以下を出力する。

```text
- rewrite validator JSON report
- rewrite validator Markdown report section
- rewrite transaction validation summary
- atomicity violation list
- rollback validation list
- reference rewrite issue list
- cleanup blocking reason list
- dashboard projection input
- validator cache entry
```

---

## 5. 検査対象

rewrite validator は以下を検査する。

```text
1. rewrite transaction loadability
2. rewrite_transaction_id uniqueness
3. transaction_status validity
4. operation_kind validity
5. operation ordering
6. old_value / new_value consistency
7. affected_files consistency
8. transaction atomicity
9. rollback scope completeness
10. reference rewrite consistency
11. traceability rewrite consistency
12. sec_id rewrite consistency
13. testspec/code reference rewrite consistency
14. path rewrite consistency
15. legacy_alias_rewrite consistency
16. cleanup_preparation consistency
```

---

## 6. transaction_status validity

transaction_status の許容値：

```text
planned
ready
executing
executed
validating
validated
blocked
failed
rolled_back
superseded
```

不明な transaction_status は `fail` とする。

cleanup gate へ進めるには `validated` が必要である。

`executed` は validator 未確認状態であり、cleanup_ready にしてはならない。

---

## 7. operation_kind validity

operation_kind の許容値：

```text
identity_rewrite
reference_rewrite
sec_id_rewrite
traceability_rewrite
testspec_rewrite
code_reference_rewrite
path_rewrite
legacy_alias_rewrite
cleanup_preparation
```

不明な operation_kind は `fail` とする。

---

## 8. operation ordering

基本順序：

```text
1. identity_rewrite
2. sec_id_rewrite
3. traceability_rewrite
4. reference_rewrite
5. testspec_rewrite
6. code_reference_rewrite
7. path_rewrite
8. legacy_alias_rewrite
9. cleanup_preparation
```

順序違反がある場合は `fail` とする。

ただし、path rewrite を先行させる必要がある場合は、rewrite transaction 内に temporary path mapping が明示されていれば `warn` として扱う。

---

## 9. old_value / new_value consistency

rewrite operation は、old_value と new_value の反映状態を検査する。

判定：

```text
planned / ready:
old_value が current snapshot に存在すること

executed / validating / validated:
new_value が current snapshot に存在すること

rolled_back:
old_value が復元されていること、または rollback report が存在すること
```

old_value も new_value も確認できない場合は `fail` とする。

---

## 10. affected_files consistency

affected_files は rewrite operation の target_file と一致している必要がある。

検査：

```text
- target_file が filesystem_index に存在する
- affected_files に target_file が含まれる
- create/delete の場合は operation に明示されている
- unexpected changed file がない
```

unexpected changed file がある場合は `warn` または `fail` とする。

cleanup scope に関係する場合は `fail` とする。

---

## 11. transaction atomicity

rewrite transaction は以下の atomicity を満たす必要がある。

```text
- canonical_doc_id rewrite と traceability rewrite が不整合な中間状態で validated になっていない
- sec_id rewrite と testspec rewrite が一致している
- path rewrite と legacy alias rewrite が cleanup gate 前に整合している
- drop / obsolete に unresolved references が残っていない
```

atomicity violation がある場合は `fail` とする。

---

## 12. rollback scope completeness

rollback scope は transaction 単位で存在しなければならない。

rollback scope に必要な情報：

```text
- changed files
- old values
- new values
- generated aliases
- generated reports
- updated indexes
- validator outputs
```

rollback scope が不足している場合は `warn` とする。

semantic transaction が含まれる場合の rollback scope 不足は `fail` とする。

---

## 13. reference rewrite consistency

reference_rewrite では以下を検査する。

```text
- old_doc_id reference が canonical_doc_id へ更新されている
- old_path reference が canonical_path へ更新されている、または alias で解決可能
- unresolved reference が残っていない
- reference_index が new target を解決できる
```

unresolved reference が残る場合は `fail` とする。

legacy alias により一時的に解決可能な場合は `warn` とする。

---

## 14. traceability rewrite consistency

traceability_rewrite では以下を検査する。

```text
- traceability_action と rewrite operation が一致する
- doc_id reference が canonical_doc_id へ更新されている
- merge_refs / split_refs が traceability_index に反映されている
- remove_refs 対象に未解決参照が残っていない
```

traceability reference が解決不能な場合は `fail` とする。

---

## 15. sec_id rewrite consistency

sec_id_rewrite では以下を検査する。

```text
- sec_id_action と rewrite operation が一致する
- preserve_existing_sec_ids が保持されている
- map_existing_sec_ids の mapping が存在する
- split_sec_ids の child mapping が存在する
- merge_sec_ids の target mapping が存在する
- remove_sec_ids の参照が残っていない
- sec_id collision がない
```

sec_id collision は `fail` とする。

sec_id が存在しない文書は `not_applicable` として扱える。

---

## 16. testspec/code reference rewrite consistency

testspec_rewrite / code_reference_rewrite では以下を検査する。

```text
- testspec が canonical_doc_id / sec_id を参照している
- code reference が canonical_doc_id / sec_id を参照している
- obsolete / drop 対象への参照が残っていない
- generated test / code reference が traceability_index と一致する
```

未解決の場合は `fail` とする。

対象 index が未実装の場合は `blocked` または `not_applicable` とする。

---

## 17. path rewrite consistency

path_rewrite では以下を検査する。

```text
- target path が存在する
- old path が alias または historical note として扱われている
- path rewrite によって duplicate path が発生していない
- filesystem ordering が semantic dependency として扱われていない
```

path collision は `fail` とする。

---

## 18. legacy_alias_rewrite consistency

legacy_alias_rewrite では以下を検査する。

```text
- alias required の node に alias が生成されている
- old_doc_id / old_path が canonical target に解決できる
- alias が canonical source of truth と競合していない
- alias expiration policy が存在する
```

alias required だが未生成の場合は `fail` または cleanup_blocking `warn` とする。

cleanup scope に含まれる場合は `fail` とする。

---

## 19. cleanup_preparation consistency

cleanup_preparation では以下を検査する。

```text
- cleanup gate に必要な input が生成されている
- unresolved references = 0
- required alias generated
- rewrite_state = rewrite_validated
- validator report が存在する
- dashboard projection input が生成可能
```

cleanup_preparation が不足している場合、cleanup_ready にしてはならない。

---

## 20. report schema draft

```json
{
  "schema_version": "1.0",
  "validator_module": "rewrite_validator",
  "validator_run_id": "validator-YYYYMMDD-NNN",
  "status": "warn",
  "checked_transactions": 1,
  "atomicity_violations": [],
  "rollback_issues": [],
  "failures": [],
  "warnings": [
    {
      "reason": "legacy_alias_required_but_not_generated",
      "source_domain": "rewrite",
      "rewrite_transaction_id": "rewrite-YYYYMMDD-NNN"
    }
  ],
  "cache_status": "not_cached"
}
```

---

## 21. blocking reasons

rewrite validator が出力する blocking reason 候補：

```text
rewrite_transaction_missing
rewrite_transaction_id_duplicate
transaction_status_invalid
rewrite_not_validated
rewrite_failed
operation_kind_invalid
operation_order_invalid
old_value_missing
new_value_missing
affected_files_mismatch
atomicity_violation
rollback_scope_missing
reference_rewrite_incomplete
traceability_rewrite_incomplete
sec_id_rewrite_incomplete
sec_id_collision
testspec_reference_unresolved
code_reference_unresolved
path_collision
legacy_alias_not_generated
cleanup_preparation_missing
```

---

## 22. CI mapping

CI fail 条件：

```text
- rewrite transaction missing for required canonicalization
- invalid transaction_status
- operation_kind invalid
- operation_order invalid without temporary mapping
- atomicity violation
- unresolved reference after rewrite
- traceability unresolved after rewrite
- sec_id collision
- path collision
- cleanup_ready candidate with rewrite_state != validated
```

CI warn 条件：

```text
- rollback scope incomplete for representation-only transaction
- legacy alias required but not yet generated outside cleanup scope
- temporary path mapping used
- unexpected changed file outside cleanup scope
```

---

## 23. cache 条件

rewrite validator の cache reuse 条件：

```text
- rewrite_index hash が一致する
- canonical_index hash が一致する
- filesystem_index hash が一致する
- reference_index hash が一致する
- traceability_index hash が一致する
- sec_id_index hash が一致する
- validator module version が一致する
- configuration hash が一致する
```

cache reuse 禁止条件：

```text
- rewrite transaction changed
- canonical_doc_id mapping changed
- filesystem snapshot changed
- reference_index changed
- traceability_index changed
- sec_id_index changed
- rollback rule changed
- validator module version changed
```

---

## 24. dashboard projection

rewrite validator は dashboard projection input として以下を渡す。

```text
- rewrite_state
- transaction_status
- operation status
- atomicity status
- rollback status
- reference rewrite status
- traceability rewrite status
- sec_id rewrite status
- cleanup_preparation status
- blocking reasons
- warnings
```

Dashboard はこれを表示するが、rewrite validation result を変更してはならない。

---

## 25. 禁止事項

以下を禁止する。

```text
- rewrite validator が rewrite を実行すること
- rewrite validator が rollback を実行すること
- rewrite validator が missing reference を自動修正すること
- rewrite validator が sec_id を生成すること
- executed 状態を validated とみなすこと
- rollback scope 不足の semantic transaction を cleanup_ready にすること
- legacy_alias_required のまま cleanup_ready にすること
```

---

## 26. HLDocS feedback

本 implementation model から、HLDocS 側へ以下をフィードバックする。

```text
- rewrite は transaction として validation すべき
- executed と validated を分離すべき
- rollback scope は transaction 単位で検査すべき
- doc_id / sec_id / traceability / testspec / code reference rewrite は atomicity 条件を持つべき
- rewrite validator は executor ではなく observer / gate として扱うべき
```

---

## 27. 結論

rewrite validator は、canonicalization decision が実ファイル・参照・traceability・sec_id・testspec/code reference へ安全に反映されたかを検査する validator module である。

これにより、legacy alias generation、cleanup gate、dashboard projection へ進む前に rewrite transaction の整合性を確認できる。
