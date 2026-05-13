# canonicalization report format

## 1. 目的

本ドキュメントは、canonicalization manifest を validator が処理した結果として出力する human-readable report の形式を定義する。

canonicalization report は、`dry-doc-*` から最終 `doc_id` への収束状態を確認するために使用する。

---

## 2. 基本方針

canonicalization report は、machine-readable な canonicalization manifest の検査結果を、人間がレビュー可能な形で表示する。

```text
canonicalization manifest:
入力・機械可読

canonicalization report:
出力・人間可読
```

---

## 3. 出力単位

report は以下の単位で出力する。

```text
- summary
- document fate summary
- pending list
- maintain_original_doc_id list
- issue_new_doc_id list
- merge list
- split list
- obsolete list
- drop list
- traceability action summary
- sec_id action summary
- cleanup readiness summary
- blocking reason list
```

---

## 4. summary

summary では以下を表示する。

```text
canonicalization_id
source_migration_id
total_entries
completed_entries
pending_entries
blocked_entries
cleanup_ready_entries
cleanup_blocked_entries
```

---

## 5. document fate summary

文書の最終扱いを集計する。

```text
maintain_original_doc_id: count
issue_new_doc_id: count
merge_into_other_document: count
split_into_multiple_documents: count
obsolete: count
drop: count
pending: count
```

---

## 6. entry report format

各 entry は以下の形式で表示する。

```text
entry_id:
dry_doc_id:
old_doc_id:
canonical_doc_id:
document_fate:
canonicalization_status:
traceability_action:
sec_id_action:
cleanup_allowed:
blocking_reasons:
```

---

## 7. pending list

pending list には、document_fate が未確定の entry を表示する。

```text
- dry_doc_id
- migration_entry_id
- reason
- required_decision
```

pending は cleanup_ready へ進めない。

---

## 8. maintain_original_doc_id list

旧 doc_id を維持する entry を表示する。

```text
- dry_doc_id
- old_doc_id
- canonical_doc_id
- traceability_action
- sec_id_action
```

validator は old_doc_id と canonical_doc_id が一致していることを確認する。

---

## 9. issue_new_doc_id list

新規 doc_id を発行する entry を表示する。

```text
- dry_doc_id
- old_doc_id
- canonical_doc_id
- reason
- traceability_action
- sec_id_action
```

validator は old_doc_id と canonical_doc_id の衝突がないことを確認する。

---

## 10. merge / split list

merge / split は、traceability 更新量が大きいため個別に表示する。

### merge

```text
source_dry_doc_id
target_canonical_doc_id
traceability_action: merge_refs
blocking_reasons
```

### split

```text
source_dry_doc_id
child_canonical_doc_ids
traceability_action: split_refs
blocking_reasons
```

---

## 11. obsolete / drop list

obsolete / drop は、参照残存確認が必要なため個別に表示する。

### obsolete

```text
dry_doc_id
old_doc_id
historical_note_required
legacy_alias_required
blocking_reasons
```

### drop

```text
dry_doc_id
old_doc_id
unresolved_references
cleanup_allowed
blocking_reasons
```

未解決参照が残る drop は cleanup_allowed=false とする。

---

## 12. traceability action summary

traceability_action の集計を表示する。

```text
preserve_existing_refs: count
update_refs_to_new_doc_id: count
merge_refs: count
split_refs: count
remove_refs: count
not_applicable: count
```

---

## 13. sec_id action summary

sec_id_action の集計を表示する。

```text
preserve_existing_sec_ids: count
map_existing_sec_ids: count
split_sec_ids: count
merge_sec_ids: count
remove_sec_ids: count
not_applicable: count
```

---

## 14. cleanup readiness summary

cleanup readiness を表示する。

```text
cleanup_ready: count
cleanup_blocked: count
cleanup_pending: count
```

cleanup_allowed=true の entry のみ cleanup_ready とする。

---

## 15. blocking reason list

blocking reason は以下のように分類する。

```text
pending_document_fate
semantic_equivalent_unknown
canonical_doc_id_missing
canonical_doc_id_collision
traceability_action_missing
sec_id_action_missing
unresolved_references
placeholder_relocation_remaining
legacy_alias_not_ready
federation_validator_not_passed
ci_validation_not_passed
```

---

## 16. PASS / WARN / FAIL

report 全体の判定は以下とする。

```text
PASS:
すべての entry が canonicalization completed または not_required であり、cleanup gate へ進める

WARN:
pending / blocked が存在するが、dry-run 継続は可能

FAIL:
canonical collision、manifest不整合、未解決参照付きdropなど重大問題が存在する
```

---

## 17. HLDocS feedback

本 report format から、HLDocS 側へ以下をフィードバックする。

```text
- canonicalization manifest には human-readable report が必要
- document fate decision は一覧化してレビュー可能にすべき
- pending / blocked / cleanup_ready を明示すべき
- traceability_action と sec_id_action を report 化すべき
- drop / obsolete は未解決参照を必ず表示すべき
```

---

## 18. 結論

canonicalization report は、dry-run relocation 後に `dry-doc-*` が最終 `doc_id` へどう収束するかをレビューするための人間向け出力である。

これにより、canonicalization 前の未解決状態と cleanup_ready 状態を分離して確認できる。
