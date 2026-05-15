# rewrite execution transaction detail

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における rewrite execution transaction detail を定義する。

本 detail は、rewrite validator が検査する対象である rewrite transaction 自体の境界、実行順、rollback scope、partial rewrite invalidation、traceability / sec_id / reference rewrite の整合条件を整理する。

本仕様は Preview Federation MVP では dry-run / planning 対象であり、production apply execution ではない。

---

## 2. 基本方針

rewrite transaction は以下を満たす。

```text
- transaction boundary を明示する
- operation order を明示する
- old_value / new_value を明示する
- affected_files を明示する
- rollback scope を明示する
- partial rewrite を validated と扱わない
- rewrite execution と rewrite validation を分離する
```

rewrite transaction は以下を行わない。

```text
- document_fate を独自決定しない
- canonical_doc_id を独自発行しない
- validator pass を捏造しない
- cleanup_ready を直接決定しない
- governance approval を代替しない
```

---

## 3. transaction kinds

rewrite_transaction_kind は以下とする。

```text
identity_rewrite_transaction
reference_rewrite_transaction
traceability_rewrite_transaction
sec_id_rewrite_transaction
testspec_rewrite_transaction
code_reference_rewrite_transaction
path_rewrite_transaction
legacy_alias_rewrite_transaction
cleanup_preparation_transaction
composite_rewrite_transaction
```

MVP では `composite_rewrite_transaction` を dry-run / planning のみで扱う。

---

## 4. transaction boundary

transaction boundary は以下を含む。

```text
- rewrite_transaction_id
- canonicalization_id
- source_document_ids
- target_document_ids
- affected_files
- affected_indexes
- affected_references
- affected_sec_ids
- affected_traceability_links
- rollback_scope_id
```

transaction boundary が曖昧な rewrite は apply してはならない。

---

## 5. operation order

標準 operation order：

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

順序を変更する場合は、transaction 内に例外理由と temporary mapping を明示する。

---

## 6. operation record

operation record の最小構造：

```json
{
  "operation_id": "rewrite-op-0001",
  "operation_kind": "identity_rewrite",
  "target_file": "docs/example.md",
  "old_value": "old-doc-id",
  "new_value": "new-doc-id",
  "status": "planned",
  "rollback_required": true
}
```

---

## 7. transaction status

transaction_status は以下とする。

```text
planned
ready
dry_run_completed
approval_required
approved
executing
executed
validating
validated
failed
partial_failed
rollback_required
rolled_back
rollback_failed
superseded
```

`executed` は `validated` を意味しない。

---

## 8. dry-run transaction

MVP では rewrite transaction は dry-run で扱う。

Dry-run で生成するもの：

```text
- expected diff
- affected file list
- affected reference list
- expected validator scope
- rollback scope draft
- unsafe condition list
```

Dry-run は source file を変更しない。

---

## 9. rollback scope

rollback scope は transaction 単位で定義する。

rollback scope に含める情報：

```text
- affected_files before hash
- affected_files after expected hash
- old_value list
- new_value list
- generated_aliases
- generated_reports
- affected_indexes
- restore_order
```

rollback scope がない transaction は apply してはならない。

MVP では rollback scope の存在確認までとし、rollback execution は行わない。

---

## 10. partial rewrite invalidation

partial rewrite は以下の場合に発生する。

```text
- operation の一部のみ成功
- affected_files の一部のみ更新
- identity_rewrite は完了したが reference_rewrite が未完了
- sec_id_rewrite と testspec_rewrite が不一致
- path_rewrite 後に alias generation が未完了
```

partial rewrite は `partial_failed` とする。

partial rewrite を validated / cleanup_ready として扱ってはならない。

---

## 11. identity rewrite

identity_rewrite は old_doc_id / canonical_doc_id の切替を扱う。

必要条件：

```text
- document_fate が決定済み
- canonical_doc_id が存在する
- canonicalization validator が fail ではない
- rollback scope がある
```

identity_rewrite 単独で cleanup_ready にしてはならない。

---

## 12. sec_id rewrite

sec_id_rewrite は section-level traceability の識別子を扱う。

必要条件：

```text
- sec_id_action が定義済み
- sec_id mapping が存在する
- collision がない
- testspec / code reference への影響が記録されている
```

MVP では sec_id index が skeleton の場合、rewrite は planning のみとする。

---

## 13. traceability rewrite

traceability_rewrite は doc_id / sec_id / ref_id 間の関係を更新する。

必要条件：

```text
- traceability_action が定義済み
- source refs が解決可能
- target refs が解決可能
- merge / split / remove_refs の方針が明示されている
```

traceability_rewrite incomplete は cleanup_blocked とする。

---

## 14. reference rewrite

reference_rewrite は文書間リンク、path reference、external artifact reference を更新する。

必要条件：

```text
- old target が特定されている
- new target が特定されている
- unresolved references が残らない
- alias で一時解決する場合は alias policy がある
```

unresolved reference が残る rewrite は validated にできない。

---

## 15. testspec / code reference rewrite

testspec / code reference rewrite は検証仕様・コード参照の整合を扱う。

必要条件：

```text
- affected testspec が特定されている
- affected code references が特定されている
- sec_id mapping と一致している
- removed target への参照が残らない
```

MVP では詳細抽出が skeleton の場合、cleanup_ready を block または pending とする。

---

## 16. path rewrite

path_rewrite は filesystem path の変更を扱う。

必要条件：

```text
- target_path が決定済み
- duplicate path がない
- old_path reference の扱いが決定済み
- legacy alias / redirect / historical note 方針がある
```

path_rewrite は semantic identity の代替ではない。

---

## 17. legacy alias rewrite

legacy_alias_rewrite は old_doc_id / old_path から canonical target への互換参照を扱う。

必要条件：

```text
- alias required / not_required が決定済み
- alias target が存在する
- expiration policy がある
- alias が source of truth と競合しない
```

alias required だが未生成の場合、cleanup_ready にしてはならない。

---

## 18. cleanup preparation

cleanup_preparation は cleanup gate に渡すための準備である。

含める情報：

```text
- cleanup candidate list
- unresolved reference check result
- required alias status
- stale artifact status
- rollback scope reference
```

cleanup_preparation は cleanup execution ではない。

---

## 19. transaction report structure

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "rewrite_transaction_report",
  "preview_only": true,
  "dry_run_only": true,
  "rewrite_transaction_id": "rewrite-YYYYMMDD-NNN",
  "transaction_kind": "composite_rewrite_transaction",
  "transaction_status": "dry_run_completed",
  "operations": [],
  "affected_files": [],
  "rollback_scope": null,
  "findings": []
}
```

---

## 20. validator connection

rewrite_validator は transaction report を検査する。

検査項目：

```text
- transaction_status
- operation order
- affected_files consistency
- rollback_scope presence
- partial_failed absence
- unresolved reference absence
- sec_id / traceability skeleton-as-pass absence
```

---

## 21. CI mapping

CI fail 条件：

```text
- partial_failed transaction
- executed but not validated transaction used as cleanup evidence
- rollback_scope missing in apply candidate
- operation order invalid
- unresolved references after rewrite
- skeleton traceability used as pass evidence
```

CI warn 条件：

```text
- dry-run only transaction
- planning-only sec_id rewrite
- alias generation pending outside cleanup scope
```

---

## 22. 禁止事項

以下を禁止する。

```text
- partial rewrite を validated と扱うこと
- executed を validated と扱うこと
- rollback scope なしに apply candidate とすること
- identity rewrite のみで cleanup_ready とすること
- skeleton sec_id / traceability index を pass 根拠にすること
- cleanup_preparation を cleanup execution と混同すること
```

---

## 23. HLDocS feedback

本 detail から、HLDocS 側へ以下をフィードバックする。

```text
- rewrite transaction は validator とは別に定義すべき
- executed と validated を分離すべき
- partial rewrite invalidation を正式状態として扱うべき
- rollback scope は transaction boundary に含めるべき
- identity / sec_id / traceability / reference rewrite の順序を明示すべき
```

---

## 24. 結論

rewrite execution transaction detail は、SansaVRM 再構成における rewrite transaction の境界、順序、rollback、partial failure、traceability / sec_id / reference rewrite 整合を定義する仕様である。

これにより、rewrite validator が検査すべき transaction semantics を明確化し、unsafe rewrite を cleanup_ready へ進めないための基盤を作る。
