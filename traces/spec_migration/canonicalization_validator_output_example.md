# canonicalization validator output example

## 1. 目的

本ドキュメントは、canonicalization manifest を validator が処理した場合の出力例を定義する。

対象は以下である。

```text
- machine-readable JSON report
- human-readable Markdown report
```

---

## 2. 前提

本例では、`dry-doc-*` は migration workspace identifier として扱う。

最終的な traceability identifier は `canonical_doc_id` である。

filesystem ordering と canonical identity は分離する。

---

## 3. machine-readable JSON report 例

```json
{
  "report_version": "1.0",
  "canonicalization_id": "canonicalization-20260510-001",
  "source_migration_id": "migration-20260510-002-import-export-layer-dry-run",
  "result": "WARN",
  "summary": {
    "total_entries": 5,
    "completed_entries": 3,
    "pending_entries": 1,
    "blocked_entries": 1,
    "cleanup_ready_entries": 0,
    "cleanup_blocked_entries": 5
  },
  "document_fate_summary": {
    "maintain_original_doc_id": 2,
    "issue_new_doc_id": 1,
    "merge_into_other_document": 0,
    "split_into_multiple_documents": 0,
    "obsolete": 0,
    "drop": 0,
    "pending": 1
  },
  "entries": [
    {
      "entry_id": "canonical-entry-0001",
      "dry_doc_id": "dry-doc-6001-vrm-0x-10-difference-summary",
      "migration_entry_id": "import-export-entry-0002",
      "old_doc_id": "doc-existing-vrm-diff",
      "canonical_doc_id": "doc-existing-vrm-diff",
      "document_fate": "maintain_original_doc_id",
      "canonicalization_status": "completed",
      "traceability_action": "preserve_existing_refs",
      "sec_id_action": "not_applicable",
      "cleanup_allowed": false,
      "blocking_reasons": [
        "legacy_alias_not_ready",
        "ci_validation_not_passed"
      ]
    },
    {
      "entry_id": "canonical-entry-0002",
      "dry_doc_id": "dry-doc-6002-humanoid-property-design",
      "migration_entry_id": "import-export-entry-0003",
      "old_doc_id": "doc-existing-humanoid-property",
      "canonical_doc_id": "doc-new-import-export-humanoid-property",
      "document_fate": "issue_new_doc_id",
      "canonicalization_status": "completed",
      "traceability_action": "update_refs_to_new_doc_id",
      "sec_id_action": "not_applicable",
      "cleanup_allowed": false,
      "blocking_reasons": [
        "legacy_alias_not_ready"
      ]
    },
    {
      "entry_id": "canonical-entry-0003",
      "dry_doc_id": "dry-doc-4002-mujoco-integration-placeholder",
      "migration_entry_id": "runtime-entry-0002",
      "old_doc_id": "doc-existing-mujoco-integration",
      "canonical_doc_id": null,
      "document_fate": "pending",
      "canonicalization_status": "blocked",
      "traceability_action": "not_applicable",
      "sec_id_action": "not_applicable",
      "cleanup_allowed": false,
      "blocking_reasons": [
        "pending_document_fate",
        "placeholder_relocation_remaining"
      ]
    }
  ]
}
```

---

## 4. human-readable Markdown report 例

```markdown
# canonicalization report

## summary

| item | value |
|---|---:|
| result | WARN |
| total_entries | 5 |
| completed_entries | 3 |
| pending_entries | 1 |
| blocked_entries | 1 |
| cleanup_ready_entries | 0 |
| cleanup_blocked_entries | 5 |

---

## document fate summary

| document_fate | count |
|---|---:|
| maintain_original_doc_id | 2 |
| issue_new_doc_id | 1 |
| merge_into_other_document | 0 |
| split_into_multiple_documents | 0 |
| obsolete | 0 |
| drop | 0 |
| pending | 1 |

---

## pending entries

| dry_doc_id | reason | required_decision |
|---|---|---|
| dry-doc-4002-mujoco-integration-placeholder | placeholder_relocation_remaining | decide maintain / issue / merge / split / obsolete / drop |

---

## cleanup blocked entries

| dry_doc_id | document_fate | blocking_reasons |
|---|---|---|
| dry-doc-6001-vrm-0x-10-difference-summary | maintain_original_doc_id | legacy_alias_not_ready, ci_validation_not_passed |
| dry-doc-6002-humanoid-property-design | issue_new_doc_id | legacy_alias_not_ready |
| dry-doc-4002-mujoco-integration-placeholder | pending | pending_document_fate, placeholder_relocation_remaining |
```

---

## 5. 判定例の意味

上記例では `result = WARN` とする。

理由：

```text
- canonicalization completed の entry が存在する
- pending entry が存在する
- cleanup_ready entry はまだ存在しない
- dry-run 継続は可能
- cleanup / canonical switch は不可
```

---

## 6. blocking_reasons の扱い

blocking_reasons は、cleanup_ready へ進めない理由を列挙する。

例：

```text
pending_document_fate:
maintain / issue / merge / split / obsolete / drop が未確定

placeholder_relocation_remaining:
placeholder relocation が残っている

legacy_alias_not_ready:
旧path互換フェーズが未準備

ci_validation_not_passed:
CI validation が未通過
```

---

## 7. validator 実装時の注意

validator 実装時は、JSON report と Markdown report を同じ source data から生成する。

```text
canonicalization manifest
  ↓
validator execution
  ↓
JSON report
  ↓
Markdown report
```

Markdown report はレビュー用であり、最終判定は JSON report を正とする。

---

## 8. HLDocS feedback

本例から、HLDocS 側へ以下をフィードバックする。

```text
- canonicalization validator には machine-readable / human-readable の両出力が必要
- WARN は dry-run 継続可能だが cleanup 不可の状態として有用
- pending document_fate は必ず一覧化すべき
- cleanup_blocked の理由を機械可読にすべき
```

---

## 9. 結論

canonicalization validator output は、dry-run relocation から final doc_id への収束状況を機械可読・人間可読の両方で示す。

これにより、canonicalization 未完了状態と cleanup_ready 状態を明確に区別できる。
