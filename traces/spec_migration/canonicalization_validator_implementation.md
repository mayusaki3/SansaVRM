# canonicalization validator implementation

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における canonicalization validator implementation を定義する。

canonicalization validator は、canonicalization manifest と canonical_index を検査し、dry-doc / old_doc_id から canonical_doc_id への収束が安全に行えるかを確認する。

---

## 2. 基本方針

canonicalization validator は以下を行う。

```text
- document_fate の妥当性を検査する
- canonical_doc_id の存在・重複・衝突を検査する
- old_doc_id と canonical_doc_id の関係を検査する
- semantic_equivalent の状態を検査する
- traceability_action / sec_id_action の妥当性を検査する
- temporary dual canonical state を分類する
- cleanup_allowed の前提条件を検査する
```

canonicalization validator は以下を行わない。

```text
- canonical_doc_id を発行しない
- doc_id を書き換えない
- sec_id を生成しない
- rewrite を実行しない
- cleanup を実行しない
```

---

## 3. 入力

canonicalization validator の入力は以下とする。

```text
- canonical_index
- migration_index
- filesystem_index
- reference_index
- traceability_index
- sec_id_index
- hash_index
- manifest validator result
```

manifest validator が fail の場合、canonicalization validator は原則として `blocked` とする。

ただし、原因調査用の partial validation は許可する。

---

## 4. 出力

canonicalization validator は以下を出力する。

```text
- canonicalization validator JSON report
- canonicalization validator Markdown report section
- canonical conflict list
- pending decision list
- cleanup blocking reason list
- warning list
- dashboard projection input
- validator cache entry
```

---

## 5. 検査対象

canonicalization validator は以下を検査する。

```text
1. canonicalization manifest loadability
2. document_fate validity
3. pending fate detection
4. old_doc_id / canonical_doc_id relation
5. canonical_doc_id uniqueness
6. canonical_doc_id collision
7. temporary dual canonical state
8. semantic_equivalent consistency
9. traceability_action validity
10. sec_id_action validity
11. merge / split consistency
12. obsolete / drop reference safety
13. cleanup_allowed consistency
14. canonicalization_status consistency
```

---

## 6. document_fate validity

document_fate の許容値：

```text
maintain_original_doc_id
issue_new_doc_id
merge_into_other_document
split_into_multiple_documents
obsolete
drop
pending
```

不明な document_fate は `fail` とする。

`pending` は `warn` だが、cleanup_ready には進めない。

---

## 7. pending fate detection

以下の場合は pending decision として扱う。

```text
- document_fate = pending
- canonical_doc_id が必要だが未設定
- merge target が未設定
- split child entry が未設定
- obsolete / drop の参照安全性が未確認
```

pending decision がある場合、canonicalization validator の overall status は原則 `warn` とする。

ただし cleanup scope に含まれる場合は `fail` または `blocked` とする。

---

## 8. old_doc_id / canonical_doc_id relation

### maintain_original_doc_id

```text
old_doc_id == canonical_doc_id
```

でなければならない。

不一致の場合は `fail` とする。

### issue_new_doc_id

```text
old_doc_id != canonical_doc_id
canonical_doc_id != null
```

でなければならない。

old_doc_id と canonical_doc_id が一致する場合は `fail` とする。

### merge_into_other_document

canonical_doc_id は merge target を指す。

merge target が存在しない場合は `fail` とする。

### split_into_multiple_documents

親 entry は child canonical_doc_id set を持つ必要がある。

child entry が存在しない場合は `fail` とする。

### obsolete

old_doc_id は存在してよい。

canonical_doc_id は historical note / alias target がある場合のみ設定する。

### drop

canonical_doc_id は原則 null とする。

参照が残る場合は drop できない。

---

## 9. canonical_doc_id uniqueness

canonical_doc_id は原則一意である。

例外：

```text
- merge_into_other_document で複数 entry が同一 target を指す場合
- legacy alias が canonical target を指す場合
```

例外が manifest 上で明示されていない重複は `fail` とする。

---

## 10. canonical_doc_id collision

canonical collision は以下に分類する。

```text
temporary_dual_canonical
invalid_canonical_conflict
canonical_resolved
```

### temporary_dual_canonical

一時的な dual canonical state として manifest に明示され、終了条件がある場合。

判定は `warn` とする。

### invalid_canonical_conflict

明示されていない恒久的衝突。

判定は `fail` とする。

### canonical_resolved

legacy alias / canonical switch により解消済み。

判定は `pass` とする。

---

## 11. semantic_equivalent consistency

semantic_equivalent の扱い：

```text
true:
semantic preserving として扱える

false:
semantic change があるため review / rewrite / testspec update が必要

unknown / null:
cleanup_ready 不可
```

representation hash mismatch のみで semantic_equivalent=false としてはならない。

normalized semantic hash mismatch がある場合は、semantic_equivalent=false または review_required とする。

---

## 12. traceability_action validity

traceability_action の許容値：

```text
preserve_existing_refs
update_refs_to_new_doc_id
merge_refs
split_refs
remove_refs
not_applicable
```

検査例：

```text
maintain_original_doc_id:
preserve_existing_refs または not_applicable

issue_new_doc_id:
update_refs_to_new_doc_id が必要

merge_into_other_document:
merge_refs が必要

split_into_multiple_documents:
split_refs が必要

drop:
remove_refs または not_applicable
```

不整合は `fail` とする。

---

## 13. sec_id_action validity

sec_id_action の許容値：

```text
preserve_existing_sec_ids
map_existing_sec_ids
split_sec_ids
merge_sec_ids
remove_sec_ids
not_applicable
```

spec 単体で新規 sec_id を生成してはならない。

sec_id collision がある場合は `fail` とする。

sec_id が存在しない文書は `not_applicable` として扱える。

---

## 14. merge / split consistency

merge / split は traceability 影響が大きいため個別検査する。

### merge

検査：

```text
- merge target canonical_doc_id が存在する
- merge_refs が指定されている
- source references が target へ解決可能
- source document の obsolete / alias 方針が定義されている
```

### split

検査：

```text
- child canonical_doc_id set が存在する
- split_refs が指定されている
- sec_id split 方針がある
- child entry が canonical_index に存在する
```

---

## 15. obsolete / drop reference safety

obsolete / drop は unresolved reference が残る場合 cleanup_ready にできない。

検査：

```text
- unresolved references = 0
- required legacy alias が定義されている
- historical note が必要な場合に指定されている
- remove_refs が traceability_action として妥当である
```

unresolved references が残る drop は `fail` とする。

obsolete で legacy alias が必要だが未生成の場合は `warn` または `blocked` とする。

---

## 16. cleanup_allowed consistency

cleanup_allowed=true にできる条件：

```text
- canonicalization_status = completed または not_required
- document_fate != pending
- semantic_equivalent = true または document_fate が obsolete / drop
- unresolved references = 0
- required traceability_action が妥当
- required sec_id_action が妥当
- invalid_canonical_conflict がない
- temporary_dual_canonical が解消済み
```

上記を満たさない cleanup_allowed=true は `fail` とする。

---

## 17. canonicalization_status consistency

canonicalization_status の許容値：

```text
pending
completed
blocked
not_required
```

不明な値は `fail` とする。

`completed` で canonical_doc_id が未設定の場合は `fail` とする。

`pending` で cleanup_allowed=true の場合は `fail` とする。

---

## 18. report schema draft

```json
{
  "schema_version": "1.0",
  "validator_module": "canonicalization_validator",
  "validator_run_id": "validator-YYYYMMDD-NNN",
  "status": "warn",
  "checked_entries": 10,
  "canonical_conflicts": [],
  "pending_decisions": [],
  "failures": [],
  "warnings": [
    {
      "reason": "temporary_dual_canonical",
      "source_domain": "canonicalization",
      "dry_doc_id": "dry-doc-example",
      "canonical_doc_id": "doc-example"
    }
  ],
  "cache_status": "not_cached"
}
```

---

## 19. blocking reasons

canonicalization validator が出力する blocking reason 候補：

```text
fate_not_decided
canonical_doc_id_missing
canonical_doc_id_collision
invalid_canonical_conflict
semantic_equivalent_unknown
traceability_action_missing
traceability_action_invalid
sec_id_action_missing
sec_id_action_invalid
sec_id_collision
merge_target_missing
split_child_missing
unresolved_references
cleanup_allowed_invalid
temporary_dual_canonical_unresolved
```

---

## 20. CI mapping

CI fail 条件：

```text
- invalid document_fate
- maintain_original_doc_id で old_doc_id != canonical_doc_id
- issue_new_doc_id で old_doc_id == canonical_doc_id
- canonical_doc_id collision
- invalid_canonical_conflict
- unresolved references on drop target
- sec_id collision
- cleanup_allowed=true だが条件未達
```

CI warn 条件：

```text
- pending document_fate
- temporary_dual_canonical with declared exit condition
- semantic_equivalent unknown outside cleanup scope
- obsolete requiring legacy alias
```

---

## 21. cache 条件

canonicalization validator の cache reuse 条件：

```text
- canonical_index hash が一致する
- migration_index hash が一致する
- reference_index hash が一致する
- traceability_index hash が一致する
- sec_id_index hash が一致する
- validator module version が一致する
- configuration hash が一致する
```

cache reuse 禁止条件：

```text
- canonicalization manifest changed
- canonical_doc_id mapping changed
- document_fate changed
- reference_index changed
- traceability_index changed
- sec_id_index changed
- validator module version changed
```

---

## 22. dashboard projection

canonicalization validator は dashboard projection input として以下を渡す。

```text
- fate_state
- canonicalization_state
- canonical conflict state
- semantic_equivalent state
- traceability_action state
- sec_id_action state
- cleanup_allowed consistency
- blocking reasons
- warnings
```

Dashboard はこれを表示するが、canonicalization result を変更してはならない。

---

## 23. 禁止事項

以下を禁止する。

```text
- canonicalization validator が canonical_doc_id を発行すること
- pending document_fate を cleanup_ready とすること
- representation hash mismatch のみで semantic_equivalent=false と断定すること
- temporary dual canonical を恒久状態として許容すること
- unresolved references が残る drop を許可すること
- cleanup_allowed を cleanup gate の代替として扱うこと
```

---

## 24. HLDocS feedback

本 implementation model から、HLDocS 側へ以下をフィードバックする。

```text
- canonicalization validator は document_fate と canonical_doc_id の整合性を検査すべき
- temporary dual canonical state には終了条件が必要
- semantic_equivalent は representation hash ではなく semantic projection に基づくべき
- cleanup_allowed は cleanup gate の代替ではなく事前条件として扱うべき
- merge / split / obsolete / drop は traceability 影響を個別検査すべき
```

---

## 25. 結論

canonicalization validator は、dry-doc / old_doc_id から canonical_doc_id への収束が安全に成立しているかを検査する validator module である。

これにより、rewrite transaction、legacy alias、cleanup gate へ進む前に、document fate decision と canonical identity の整合性を確認できる。
