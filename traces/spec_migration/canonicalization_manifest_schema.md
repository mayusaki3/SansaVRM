# canonicalization manifest schema

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再配置 dry-run における `dry-doc-*` から最終 `doc_id` への収束を記録する canonicalization manifest の構造を定義する。

本スキーマは、HLDocS 側へのフィードバック候補を兼ねる。

---

## 2. 基本方針

canonicalization manifest は、migration workspace identifier と canonical identity の対応を記録する。

```text
migration identity:
dry-doc-xxxx

canonical identity:
final doc_id
```

`dry-doc-*` は最終 doc_id ではない。

---

## 3. canonicalization の位置づけ

canonicalization は document fate decision の後、legacy alias phase の前に行う。

```text
semantic verification
  ↓
document fate decision
  ↓
canonicalization
  ↓
legacy alias phase
  ↓
cleanup_ready
```

---

## 4. document fate

各 dry-doc は canonicalization 前に以下のいずれかへ分類する。

```text
maintain_original_doc_id
issue_new_doc_id
merge_into_other_document
split_into_multiple_documents
obsolete
drop
pending
```

`pending` は cleanup_ready へ進めない。

---

## 5. schema draft

```json
{
  "schema_version": "1.0",
  "canonicalization_id": "canonicalization-YYYYMMDD-NNN",
  "source_migration_id": "migration-YYYYMMDD-NNN",
  "ordering_note": "Numbering is filesystem ordering only. Canonical identity is represented by doc_id.",
  "entries": [
    {
      "entry_id": "canonical-entry-0001",
      "dry_doc_id": "dry-doc-example",
      "migration_entry_id": "migration-entry-0001",
      "document_fate": "maintain_original_doc_id",
      "old_doc_id": "doc-previous",
      "canonical_doc_id": "doc-previous",
      "canonicalization_status": "completed",
      "semantic_equivalent": true,
      "traceability_action": "preserve_existing_refs",
      "sec_id_action": "preserve_existing_sec_ids",
      "cleanup_allowed": false,
      "reason": "same semantic responsibility"
    }
  ]
}
```

---

## 6. entry fields

### entry_id

canonicalization entry の一意識別子。

### dry_doc_id

dry-run relocation 中に使用した migration workspace identifier。

### migration_entry_id

元となる migration manifest entry。

### document_fate

canonicalization 前の最終分類。

許容値：

```text
maintain_original_doc_id
issue_new_doc_id
merge_into_other_document
split_into_multiple_documents
obsolete
drop
pending
```

### old_doc_id

旧文書の doc_id。

旧 doc_id がない新規文書では null とする。

### canonical_doc_id

最終的に採用する doc_id。

`document_fate = drop` の場合は null とする。

### canonicalization_status

許容値：

```text
pending
completed
blocked
not_required
```

---

## 7. traceability_action

traceability_action は、既存コード・テスト・仕様参照に対する処理を表す。

許容値：

```text
preserve_existing_refs
update_refs_to_new_doc_id
merge_refs
split_refs
remove_refs
not_applicable
```

---

## 8. sec_id_action

sec_id_action は、sec_id の扱いを表す。

許容値：

```text
preserve_existing_sec_ids
map_existing_sec_ids
split_sec_ids
merge_sec_ids
remove_sec_ids
not_applicable
```

spec単体で新規sec_idを生成することは、このスキーマの目的ではない。

---

## 9. cleanup_allowed

cleanup_allowed は canonicalization 完了だけでは true にならない。

cleanup_allowed には、cleanup gate の全条件を満たす必要がある。

```text
- canonicalization_status = completed または not_required
- semantic_equivalent = true または document_fate が obsolete / drop
- placeholder relocation がない
- legacy alias policy が適用済みまたは不要
- federation validator が PASS
- CI validation が PASS
```

---

## 10. document_fate 別の扱い

### maintain_original_doc_id

旧 doc_id を canonical_doc_id として維持する。

### issue_new_doc_id

新しい canonical_doc_id を発行し、参照更新を traceability_action に記録する。

### merge_into_other_document

統合先 doc_id を canonical_doc_id に記録し、merge_refs を行う。

### split_into_multiple_documents

複数の canonical_doc_id を持つため、子 entry を作成する。

### obsolete

文書としては廃止候補にするが、参照互換のため legacy alias または historical note を残す可能性がある。

### drop

最終的に削除する。

ただし、参照が残る場合は drop できない。

### pending

判断未確定。

cleanup_ready へ進めない。

---

## 11. validator requirements

validator は以下を検査する。

```text
- dry_doc_id が migration manifest に存在する
- migration_entry_id が federation 内で解決できる
- canonical_doc_id の重複がない
- document_fate が pending のものは cleanup_allowed=false
- drop 対象に未解決参照が残っていない
- maintain_original_doc_id で old_doc_id と canonical_doc_id が一致する
- issue_new_doc_id で old_doc_id と canonical_doc_id が衝突していない
```

---

## 12. HLDocS feedback

本スキーマから、HLDocS 側へ以下をフィードバックする。

```text
- dry-run relocation 後に canonicalization manifest が必要
- document fate decision を canonicalization 前に定義すべき
- doc_id維持 / 新規発行 / merge / split / obsolete / drop を明示状態として扱うべき
- traceability_action と sec_id_action を記録すべき
- cleanup_ready は canonicalization 完了後にのみ評価すべき
```

---

## 13. 結論

canonicalization manifest は、dry-run relocation で使用した `dry-doc-*` を最終 `doc_id` へ収束させるための記録である。

これにより、再構成中の作業識別子と恒久的な traceability identifier を分離できる。
